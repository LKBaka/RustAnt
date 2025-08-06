use colored::Colorize;

use crate::{byte_code_vm::{code::code::instruction_to_str, compiler::utils::compile_it, vm::vm::Vm}, object::{object::Object, utils::create_error_with_name}};

pub enum RunError {
    RuntimeError(Object),
    CompileError(String),
}

pub fn run(code: String, file: String) -> Result<Option<Object>, RunError> {
    let bytecode = {
        let compile_result = compile_it(code, file);
        match compile_result {
            Ok(bytecode) => bytecode,
            Err(msg) => return Err(RunError::CompileError(msg))
        }
    };

    println!("{}, ByteCode: {:?}, Instructions: {}", "机器已上电".green(), bytecode, instruction_to_str(&bytecode.instructions));

    let mut vm = Vm::new(bytecode);

    match vm.run() {
        Ok(_) => {
            if let Some(result) = vm.last_popped_stack_elem() {
                Ok(Some(result))
            } else {
                Ok(None)
            }
        },
        Err(msg) => Err(RunError::RuntimeError(create_error_with_name(
            "RuntimeError",
            msg
        ))),
    }
}