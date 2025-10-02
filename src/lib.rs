use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{byte_code_vm::{compiler::{compiler::Compiler, symbol_table::symbol_table::SymbolTable}, constants::UNINIT_OBJ, run::{run, RunError}, vm::vm::GLOBALS_SIZE}, obj_enum::object::Object, object::object::IAntObject};

pub mod arg_structure;
pub mod ast;
pub mod builtin;
pub mod byte_code_vm;
pub mod constants;
pub mod lexer;
pub mod module_importer;
pub mod obj_enum;
pub mod object;
pub mod parser;
pub mod runner;
pub mod token;
pub mod utils;

#[wasm_bindgen]
pub fn run_wasm(code: String, file: String) {
    let uninit: Rc<RefCell<Object>> = rc_ref_cell!(Object::AntUninit(UNINIT_OBJ.clone()));

    let symbol_table = rc_ref_cell!(SymbolTable::new());
    let constants = rc_ref_cell!(vec![]);
    let field_pool = rc_ref_cell!(vec![]);
    let globals = rc_ref_cell!(vec![uninit.clone(); GLOBALS_SIZE as usize]);

    Compiler::init_builtin_map(symbol_table.clone());

    let result = run(
        code.to_owned(),
        file.to_owned(),
        symbol_table, constants, field_pool, globals
    );

    match result {
        Ok(_) => (),
        Err(err) => {
            match err {
                RunError::CompileError(msg) => println!("{}", msg),
                RunError::RuntimeError(msg) => {
                    println!("{}", msg.inspect());
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
use web_sys::console;

// 为 WASM 环境重写 println! print! 宏
#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {{
        use web_sys::console;

        let message = format!($($t)*);
        console::log_1(&format!("{}", message).into());
    }}
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
    macro_rules! print {
        ($($t:tt)*) => {
            use web_sys::console;
            
            console::log_1(&format!($($t)*).into());
        };
    }