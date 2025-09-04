use std::{cell::RefCell, rc::Rc};


use crate::{
    byte_code_vm::{
        compiler::{symbol_table::symbol_table::SymbolTable, utils::compile_with_state},
        vm::vm::Vm,
    }, obj_enum::object::Object, object::utils::create_error_with_name
};

pub enum RunError {
    RuntimeError(Object),
    CompileError(String),
}

pub fn run(
    code: String,
    file: String,
    symbol_table: Rc<RefCell<SymbolTable>>,
    constants: Rc<RefCell<Vec<Object>>>,
    globals: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,
) -> Result<Option<Object>, RunError> {
    #[cfg(feature = "debug")]
    use colored::Colorize;

    #[cfg(feature = "debug")]
    use crate::byte_code_vm::vm::frame::fmt_frames;

    let bytecode = {
        let compile_result = compile_with_state(code, file, symbol_table, constants);

        match compile_result {
            Ok(bytecode) => bytecode,
            Err(msg) => return Err(RunError::CompileError(msg)),
        }
    };

    #[cfg(feature = "debug")]
    println!(
        "{}, ByteCode: {:#?}, Instructions: {}",
        "机器已上电".green(),
        bytecode,
        crate::byte_code_vm::code::code::instruction_to_str(&bytecode.instructions)
    );

    let mut vm = Vm::with_globals(bytecode, globals);

    match vm.run() {
        Ok(_) => {
            #[cfg(feature = "debug")]
            println!("{}", fmt_frames(&vm.frames()));

            if let Some(result) = vm.last_popped_stack_elem() {
                Ok(Some(result.borrow().clone()))
            } else {
                Ok(None)
            }
        }
        Err(msg) => {
            #[cfg(feature = "debug")]
            println!("{}", fmt_frames(&vm.frames()));

            Err(RunError::RuntimeError(create_error_with_name(
                "RuntimeError",
                msg,
            )))
        }
    }
}
