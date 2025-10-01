#[cfg(target_arch = "wasm32")]
use crate::println;

use std::{cell::RefCell, rc::Rc};

use crate::{
    byte_code_vm::{
        compiler::{
            compile_error::CompileErrorBox, symbol_table::symbol_table::SymbolTable, utils::compile_with_state
        }, vm::vm::Vm
    },
    obj_enum::object::Object,
    object::ant_string::AntString,
};

#[derive(Debug)]
pub enum RunError {
    RuntimeError(Object),
    CompileError(CompileErrorBox),
}

pub fn run(
    code: String,
    file: String,
    symbol_table: Rc<RefCell<SymbolTable>>,
    constants: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,
    field_pool: Rc<RefCell<Vec<String>>>,
    globals: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,
) -> Result<Option<Object>, RunError> {
    #[cfg(feature = "debug")]
    use colored::Colorize;

    #[cfg(feature = "debug")]
    use crate::byte_code_vm::vm::frame::fmt_frames;

    let bytecode = {
        let compile_result = compile_with_state(
            code, file, symbol_table, constants, field_pool
        );

        match compile_result {
            Ok(bytecode) => bytecode,
            Err(err) => return Err(RunError::CompileError(err)),
        }
    };

    #[cfg(feature = "debug")]
    println!(
        "{}, ByteCode: {:#?}, Instructions: {}",
        "机器已上电".green(),
        bytecode,
        crate::byte_code_vm::code::code::instruction_to_str(&bytecode.instructions),
    );

    let mut vm = Vm::with_globals(bytecode, globals);

    match vm.run() {
        Ok(_) => {
            #[cfg(feature = "debug")]
            {
                use crate::byte_code_vm::constants::UNINIT_OBJECT;

                println!("{}", fmt_frames(&vm.frames()));
                println!(
                    "Globals: {:#?}",
                    vm.globals
                        .borrow()
                        .iter()
                        .filter(|global| &*global.borrow() != &*UNINIT_OBJECT)
                        .collect::<Vec<&Rc<RefCell<Object>>>>()
                );
            }

            if let Some(result) = vm.last_popped_stack_elem() {
                Ok(Some(result.borrow().clone()))
            } else {
                Ok(None)
            }
        }
        Err(msg) => {
            #[cfg(feature = "debug")]
            {
                use crate::byte_code_vm::constants::UNINIT_OBJECT;

                println!("{}", fmt_frames(&vm.frames()));
                println!(
                    "Globals: {:#?}",
                    vm.globals
                        .borrow()
                        .iter()
                        .filter(|global| &*global.borrow() != &*UNINIT_OBJECT)
                        .collect::<Vec<&Rc<RefCell<Object>>>>()
                );
            }

            Err(RunError::RuntimeError(Object::AntString(AntString::new(
                format!("{}\n{}", vm.traceback_string(), msg),
            ))))
        }
    }
}

pub fn run_pop(
    code: String,
    file: String,
    symbol_table: Rc<RefCell<SymbolTable>>,
    constants: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,
    field_pool: Rc<RefCell<Vec<String>>>,
    globals: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,
) -> Result<Option<Object>, RunError> {
    #[cfg(feature = "debug")]
    use colored::Colorize;

    #[cfg(feature = "debug")]
    use crate::byte_code_vm::vm::frame::fmt_frames;

    let bytecode = {
        let compile_result = compile_with_state(
            code, file, symbol_table, constants, field_pool
        );

        match compile_result {
            Ok(bytecode) => bytecode,
            Err(err) => return Err(RunError::CompileError(err)),
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

            if let Some(result) = vm.pop() {
                Ok(Some(result.borrow().clone()))
            } else {
                Ok(None)
            }
        }
        Err(msg) => {
            #[cfg(feature = "debug")]
            println!("{}", fmt_frames(&vm.frames()));

            Err(RunError::RuntimeError(Object::AntString(AntString::new(
                format!("{}\n{}", vm.traceback_string(), msg),
            ))))
        }
    }
}
