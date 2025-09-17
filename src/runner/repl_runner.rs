use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::rc::Rc;

use colored::Colorize;

use crate::byte_code_vm::compiler::compiler::Compiler;
use crate::byte_code_vm::compiler::symbol_table::symbol_table::SymbolTable;
use crate::byte_code_vm::constants::UNINIT_OBJ;
use crate::byte_code_vm::run::{RunError, run};
use crate::byte_code_vm::vm::vm::GLOBALS_SIZE;
use crate::obj_enum::object::Object;
use crate::object::object::IAntObject;
use crate::rc_ref_cell;

pub struct REPLRunner {}

const REPL_FILE_NAME: &'static str = "repl";

impl REPLRunner {
    pub fn new() -> Self {
        REPLRunner {}
    }

    pub fn run(&self) {
        let uninit: Rc<RefCell<Object>> = rc_ref_cell!(Object::AntUninit(UNINIT_OBJ.clone()));

        let symbol_table = rc_ref_cell!(SymbolTable::new());
        let constants = rc_ref_cell!(vec![]);
        let globals = rc_ref_cell!(vec![uninit.clone(); GLOBALS_SIZE as usize]);

        Compiler::init_builtin_map(symbol_table.clone());

        loop {
            let mut code: String = String::new();

            print!(">>> ");

            let flush_result = io::stdout().flush(); // 刷新缓冲区（重要！）
            match flush_result {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    continue;
                }
            }

            let read_line_result = io::stdin().read_line(&mut code);
            match read_line_result {
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    continue;
                }
                Ok(_) => {}
            }

            // 使用多行输入功能
            let mut full_code = self.read_multi_line(&mut code);

            // 如果用户输入为空，跳过执行
            if full_code.trim().is_empty() {
                continue;
            }

            full_code = full_code.trim_end().to_string(); // 去除末尾的换行符

            #[cfg(feature = "get_code_run_seconds")]
            use std::time::Instant;

            #[cfg(feature = "get_code_run_seconds")]
            let start = Instant::now();

            let result = run(
                full_code,
                REPL_FILE_NAME.to_string(),
                symbol_table.clone(),
                constants.clone(),
                globals.clone(),
            );

            match result {
                Ok(it) => {
                    if let Some(it) = it {
                        println!("{}", it.inspect())
                    }
                }
                Err(err) => {
                    let print_msg = |msg: String| eprintln!("{}", msg.red());

                    match err {
                        RunError::CompileError(msg) => print_msg(msg.to_string()),
                        RunError::RuntimeError(msg) => {
                            eprintln!("{}", msg.inspect().red());
                        }
                    }
                }
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
    }

    fn read_multi_line(&self, input: &mut String) -> String {
        let mut code = input.clone();

        // 检查是否需要继续读取多行
        while self.needs_more_input(&code) {
            print!("... ");

            // 刷新输出缓冲区
            if let Err(e) = io::stdout().flush() {
                eprintln!("{}", e.to_string());
                break;
            }

            let mut line = String::new();
            if let Err(e) = io::stdin().read_line(&mut line) {
                eprintln!("{}", e.to_string());
                break;
            }

            code.push_str(&line);
        }

        code
    }

    /// 检查代码是否需要更多输入
    fn needs_more_input(&self, code: &str) -> bool {
        let trimmed = code.trim();

        // 空行不需要更多输入
        if trimmed.is_empty() {
            return false;
        }

        // 检查未闭合的括号、引号等
        let mut paren_count = 0;
        let mut brace_count = 0;
        let mut bracket_count = 0;
        let mut in_string = false;
        let mut escape_next = false;

        for ch in code.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                continue;
            }

            if ch == '"' && !escape_next {
                in_string = !in_string;
                continue;
            }

            if in_string {
                continue;
            }

            match ch {
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                '[' => bracket_count += 1,
                ']' => bracket_count -= 1,
                _ => {}
            }
        }

        // 如果有未闭合的括号，需要更多输入
        if paren_count > 0 || brace_count > 0 || bracket_count > 0 {
            return true;
        }

        // 检查是否以需要继续的字符结尾
        let last_char = trimmed.chars().last();
        match last_char {
            Some(ch) => {
                // 如果以这些字符结尾，可能需要更多输入
                matches!(
                    ch,
                    ',' | '=' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' | '!' | '<' | '>'
                )
            }
            None => false,
        }
    }
}
