use std::{collections::HashMap, fs};

use crate::{
    byte_code_vm::{
        compiler::{
            compiler::Compiler,
            symbol_table::symbol_table::{SymbolScope, SymbolTable},
        },
        constants::UNINIT_OBJECT,
        vm::vm::Vm,
    },
    object::ant_class::AntClass,
    parser::utils::parse,
    rc_ref_cell,
};

pub struct AntModuleImporter<'a> {
    pub file: String,
    pub vm: &'a mut Vm,
}

impl<'a> AntModuleImporter<'a> {
    pub fn import(&mut self) -> Result<AntClass, String> {
        let code = fs::read_to_string(&self.file);
        if let Err(err) = code {
            return Err(format!("{err}"));
        }

        let program = parse(code.unwrap(), self.file.clone());

        if let Ok(it) = program {
            #[cfg(feature = "debug")]
            {
                use colored::Colorize;

                use crate::ast::ast::INode;

                println!("AST: {}", it.to_string().yellow());
            }

            let mut compiler = Compiler::with_state(
                {
                    let mut table = SymbolTable::new();

                    table.num_definitions = self
                        .vm
                        .globals
                        .borrow()
                        .iter()
                        .filter(|global| &*global.borrow() != &*UNINIT_OBJECT)
                        .count();

                    let table = rc_ref_cell!(table);

                    Compiler::init_builtin_map(table.clone());
                    table
                },
                rc_ref_cell!(self.vm.constants.clone()),
                rc_ref_cell!(self.vm.field_pool.clone()),
                self.file.clone().into(),
            );

            let result = compiler.start_compile(it);

            let bytecode = match result {
                Ok(_) => compiler.bytecode(),
                Err(msg) => return Err(format!("error compile module: {msg}")),
            };

            let mut vm = Vm::new(bytecode);
            match vm.run() {
                Ok(_) => {}
                Err(msg) => return Err(format!("error running module: {msg}")),
            };

            let mut vm_globals = vm.globals.borrow_mut();
            let symbol_table = compiler.symbol_table.borrow();

            let mut globals = HashMap::new();

            symbol_table.store.iter().for_each(|(name, symbol)| {
                if symbol.scope == SymbolScope::Builtin {
                    return;
                }

                let global = vm_globals[symbol.index].borrow().clone();

                globals.insert(name.clone(), global);
            });

            self.vm.globals.borrow_mut().append(&mut vm_globals);

            return Ok(AntClass::from(globals));
        }

        Err(String::from("parse failed!"))
    }
}
