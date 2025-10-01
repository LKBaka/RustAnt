#[cfg(target_arch = "wasm32")]
use crate::println;

use std::{cell::RefCell, fs, rc::Rc};

use colored::Colorize;

use crate::byte_code_vm::compiler::compiler::Compiler;
use crate::byte_code_vm::compiler::symbol_table::symbol_table::SymbolTable;
use crate::byte_code_vm::constants::UNINIT_OBJ;
use crate::byte_code_vm::run::run;
use crate::byte_code_vm::vm::vm::GLOBALS_SIZE;
use crate::obj_enum::object::Object;
use crate::object::object::{IAntObject, ERROR};
use crate::rc_ref_cell;

pub struct FileRunner {
    file_path: String,
}

impl FileRunner {
    pub fn new(file_path: String) -> Self {
        FileRunner { file_path }
    }

    pub fn run(&self) {
        // 读取文件
        let contents = fs::read_to_string(&self.file_path);
        match contents {
            Ok(contents) => {
                #[cfg(feature = "get_code_run_seconds")]
                use std::time::Instant;

                #[cfg(feature = "get_code_run_seconds")]
                let start = Instant::now();

                let uninit: Rc<RefCell<Object>> = rc_ref_cell!(Object::AntUninit(UNINIT_OBJ.clone()));

                let symbol_table = rc_ref_cell!(SymbolTable::new());
                let constants = rc_ref_cell!(vec![]);
                let globals = rc_ref_cell!(vec![uninit.clone(); GLOBALS_SIZE as usize]);

                Compiler::init_builtin_map(symbol_table.clone());

                let result = run(
                    contents,
                    self.file_path.clone(),
                    symbol_table,
                    constants,
                    rc_ref_cell!(vec![]),
                    globals,
                );

                if let Err(err_enum) = result {
                    use crate::byte_code_vm::run::RunError;

                    if let RunError::CompileError(msg) = err_enum {
                        use colored::Colorize;

                        eprintln!("{}", msg.to_string().red());
                    } else if let RunError::RuntimeError(err) = err_enum {
                        use colored::Colorize;

                        eprintln!("{}", err.inspect().red());
                    }
                } else if let Ok(Some(result)) = result
                    && result.get_type() == ERROR
                {
                    eprintln!("{}", result.inspect().red());
                }

                #[cfg(feature = "get_code_run_seconds")]
                let start_elapsed = start.elapsed();

                #[cfg(feature = "get_code_run_seconds")]
                println!(
                    "{}",
                    format!(
                        "Code run time: {} seconds, {} milliseconds, {} nanoseconds",
                        start_elapsed.as_secs_f64(),
                        start_elapsed.as_millis(),
                        start_elapsed.as_nanos()
                    )
                );
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
                return;
            }
        }
    }
}
