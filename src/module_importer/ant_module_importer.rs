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

            // 考虑在再次出现 bug 时尝试启用他
            // let table_num_def_cnt =
            //     self
            //         .vm
            //         .globals
            //         .borrow()
            //         .iter()
            //         .filter(|global| &*global.borrow() != &*UNINIT_OBJECT)
            //         .count() + 1;

            let table_num_def_cnt = self.vm.global_count + 1;

            let mut compiler = Compiler::with_state(
                {
                    let table = SymbolTable::new();

                    let table = rc_ref_cell!(table);

                    Compiler::init_builtin_map(table.clone());

                    table.borrow_mut().num_definitions += table_num_def_cnt;

                    table
                },
                rc_ref_cell!(vec![
                    rc_ref_cell!(UNINIT_OBJECT.clone());
                    self.vm.constants.len()
                ]),
                rc_ref_cell!(vec![
                    String::from("invaild string");
                    self.vm.field_pool.len()
                ]),
                self.file.clone().into(),
            );

            let result = compiler.start_compile(it);

            let bytecode = match result {
                Ok(_) => compiler.bytecode(),
                Err(msg) => return Err(format!("error compile module: {msg}")),
            };

            #[cfg(feature = "debug")]
            {
                use colored::Colorize;

                println!(
                    "{}, ByteCode: {:#?}, Instructions: {}",
                    "机器已上电".green(),
                    bytecode,
                    crate::byte_code_vm::code::code::instruction_to_str(&bytecode.instructions),
                );
            }

            let mut vm = Vm::new(bytecode);

            match vm.run() {
                Ok(_) => {}
                Err(msg) => return Err(format!("error running module: {msg}")),
            };

            let vm_globals = vm.globals.borrow_mut();

            let vm_field_pool = &vm.field_pool[self.vm.field_pool.len()..];
            let vm_constants = &vm.constants[self.vm.constants.len()..];

            let symbol_table = compiler.symbol_table.borrow();

            let mut globals = HashMap::new();

            symbol_table.store.iter().for_each(|(name, symbol)| {
                if symbol.scope == SymbolScope::Builtin {
                    return;
                }

                let global = vm_globals[symbol.index].borrow().clone();

                globals.insert(name.clone(), global);
            });

            self.vm.constants.append(&mut vm_constants.to_vec());
            self.vm.field_pool.append(&mut vm_field_pool.to_vec());

            // 将子 VM 的新全局按符号索引写回主 VM，保持原有语义
            for (_name, symbol) in symbol_table.store.iter() {
                if symbol.scope == SymbolScope::Builtin {
                    continue;
                }

                self.vm.globals.borrow_mut()[symbol.index] = 
                    vm_globals[symbol.index].clone();
            }

            return Ok(AntClass::from(globals));
        }

        Err(String::from("parse failed!"))
    }
}
