use std::{collections::HashMap, ffi::OsStr, fs, path::Path};

use crate::{
    byte_code_vm::{
        compiler::{compiler::Compiler, symbol_table::symbol_table::SymbolTable},
        constants::UNINIT_OBJECT,
        vm::vm::{GLOBALS_SIZE, Vm},
    },
    obj_enum::object::Object,
    object::{ant_class::AntClass, object::IAntObject},
    parser::utils::parse,
    rc_ref_cell,
};

pub struct AntModuleImporter<'a, 'b> {
    pub file: String,
    pub vm: &'a mut Vm<'b>,
}

impl<'a, 'b> AntModuleImporter<'a, 'b> {
    pub fn import(&mut self) -> Result<AntClass, String> {
        let code = fs::read_to_string(&self.file);
        if let Err(err) = code {
            return Err(format!("{err}"));
        }

        let class_name = Path::new(&self.file)
            .file_stem()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("");

        let code = format!("class {class_name} {{{}}} {class_name}", code.unwrap());

        let program = parse(code, self.file.clone());

        if let Ok(it) = program {
            #[cfg(feature = "debug")]
            {
                use colored::Colorize;

                use crate::ast::ast::INode;

                println!("AST: {}", it.to_string().yellow());
            }

            let mut compiler = Compiler::with_state(
                {
                    let t = rc_ref_cell!(SymbolTable::new());
                    
                    Compiler::init_builtin_map(t.clone());

                    t
                },
                rc_ref_cell!(vec![]),
                rc_ref_cell!(vec![]),
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

            let mut temp_globals = vec![rc_ref_cell!(UNINIT_OBJECT.clone()); GLOBALS_SIZE];
            let mut vm = Vm::new(bytecode, &mut temp_globals);

            match vm.run() {
                Ok(_) => {}
                Err(msg) => return Err(format!("error running module: {msg}")),
            };

            let globals = HashMap::new();

            // 回收闭包变量
            let o = match vm.last_popped_stack_elem() {
                None => {
                    return Ok(AntClass::from((class_name, globals)));
                }

                Some(it) => it,
            };

            let o_binding = o.borrow();

            let clazz = match &*o_binding {
                Object::AntClass(clazz) => clazz.clone(),
                it => return Err(format!("expected class, got: {}", it.inspect())),
            };

            return Ok(clazz);
        }

        Err(String::from("parse failed!"))
    }
}
