use std::{cell::RefCell, rc::Rc};


use crate::{
    byte_code_vm::compiler::{
        compiler::{ByteCode, Compiler},
        symbol_table::symbol_table::SymbolTable,
    }, obj_enum::object::Object, parser::utils::parse
};

pub fn compile_it(code: String, file: String) -> Result<ByteCode, String> {
    let program = parse(code, file);

    if let Ok(it) = program {
        #[cfg(feature = "debug")]
        {
            use colored::Colorize;

            use crate::ast::ast::INode;

            println!("AST: {}", it.to_string().yellow());
        }

        let mut compiler = Compiler::new();

        let result = compiler.start_compile(it);

        return match result {
            Ok(_) => Ok(compiler.bytecode()),
            Err(msg) => Err(msg),
        };
    }

    Err(String::from("parse failed!"))
}

pub fn compile_with_state(
    code: String,
    file: String,
    symbol_table: Rc<RefCell<SymbolTable>>,
    constants: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,
) -> Result<ByteCode, String> {
    let program = parse(code, file);

    if let Ok(it) = program {
        #[cfg(feature = "debug")]
        {
            use colored::Colorize;

            use crate::ast::ast::INode;

            println!("AST: {}", it.to_string().yellow());
        }
        
        let mut compiler = Compiler::with_state(symbol_table, constants);

        let result = compiler.start_compile(it);

        return match result {
            Ok(_) => Ok(compiler.bytecode()),
            Err(msg) => Err(msg),
        };
    }

    Err(String::from("parse failed!"))
}
