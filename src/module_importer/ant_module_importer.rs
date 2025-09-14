use std::{collections::HashMap, fs};

use crate::{builtin::builtin_map::BUILTIN_MAP_INDEX, byte_code_vm::{compiler::compiler::Compiler, vm::vm::Vm}, object::ant_class::AntClass, parser::utils::parse};

pub struct AntModuleImporter {
    pub file: String
}

impl AntModuleImporter {
    pub fn import(&self) -> Result<AntClass, String> {
        let code = fs::read_to_string(&self.file);
        if let Err(err) = code {
            return Err(format!("{err}"))
        }

        let program = parse(code.unwrap(), self.file.clone());

        if let Ok(it) = program {
            #[cfg(feature = "debug")]
            {
                use colored::Colorize;

                use crate::ast::ast::INode;

                println!("AST: {}", it.to_string().yellow());
            }

            let mut compiler = Compiler::new();

            let result = compiler.start_compile(it);

            let bytecode =  match result {
                Ok(_) => compiler.bytecode(),
                Err(msg) => return Err(format!("error compile module: {msg}")),
            };

            let mut vm = Vm::new(bytecode);
            match vm.run() {
                Ok(_) => {},
                Err(msg) => return Err(format!("error running module: {msg}"))   
            };

            let vm_globals = vm.globals.borrow();
            let symbol_table = compiler.symbol_table.borrow();

            let mut globals = HashMap::new();

            symbol_table
                .store
                .iter()
                .for_each(|(name, symbol)| {
                    if BUILTIN_MAP_INDEX.contains(name) {
                        return;
                    }

                    let global = vm_globals[symbol.index].borrow().clone();

                    globals.insert(
                        name.clone(),
                        global
                    );
                });

            return Ok(AntClass::from(globals))
        }

        Err(String::from("parse failed!"))
    }
}