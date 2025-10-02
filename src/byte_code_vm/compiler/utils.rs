use std::{cell::RefCell, rc::Rc};

use crate::{
    byte_code_vm::compiler::{
        compile_error::CompileErrorBox,
        compiler::{ByteCode, CompileError, Compiler},
        symbol_table::symbol_table::SymbolTable,
    },
    obj_enum::object::Object,
    parser::utils::parse,
};

pub fn compile_it(code: String, file: String) -> Result<ByteCode, CompileErrorBox> {
    let program = parse(code, file.clone());

    if let Ok(it) = program {
        #[cfg(feature = "debug")]
        {
            use colored::Colorize;

            println!("AST: {}", format!("{:#?}", it).yellow());
        }

        let mut compiler = Compiler::new(file.into());

        let result = compiler.start_compile(it);

        return match result {
            Ok(_) => Ok(compiler.bytecode()),
            Err(err) => Err(CompileErrorBox::from_traceback_string(
                err,
                compiler.traceback_string()
            )),
        };
    }

    Err(CompileErrorBox::from_traceback_string(
        CompileError::from_none_token(String::from("parse failed")),
        String::new()
    ))
}

pub fn compile_with_state(
    code: String,
    file: String,
    symbol_table: Rc<RefCell<SymbolTable>>,
    constants: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,
    field_pool: Rc<RefCell<Vec<String>>>,
) -> Result<ByteCode, CompileErrorBox> {
    let program = parse(code, file.clone());

    if let Ok(it) = program {
        #[cfg(feature = "debug")]
        {
            use colored::Colorize;

            println!("AST: {}", format!("{:#?}", it).yellow());
        }

        let mut compiler = Compiler::with_state(
            symbol_table, constants, field_pool, file.into()
        );

        let result = compiler.start_compile(it);

        return match result {
            Ok(_) => Ok(compiler.bytecode()),
            Err(err) => Err(CompileErrorBox::from_traceback_string(
                err,
                compiler.traceback_string()
            )),
        };
    }

    Err(CompileErrorBox::from_traceback_string(
        CompileError::from_none_token(String::from("parse failed")),
        String::new()
    ))
}
