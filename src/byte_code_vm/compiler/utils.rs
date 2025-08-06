use crate::{byte_code_vm::compiler::compiler::{ByteCode, Compiler}, parser::utils::parse};

pub fn compile_it(code: String, file: String) -> Result<ByteCode, String> {
    let program = parse(code, file);

    if let Ok(it) = program {
        let mut compiler = Compiler::new();

        let result = compiler.start_compile(it);

        return match result {
            Ok(_) => Ok(compiler.bytecode()),
            Err(msg) => Err(msg)
        }
    }

    Err(String::from("parse failed!"))
}