use std::io;
use std::io::Write;
use crate::environment::utils::create_top_env;
use crate::runner::eval::eval;

use super::utils::import_all_modules;

pub struct REPLRunner {

}

impl REPLRunner {
    pub fn new() -> Self {
        REPLRunner{}
    }

    pub fn run(&self) {
        let mut env = create_top_env();
        
        import_all_modules(&mut env);

        loop {
            let mut code: String = String::new();

            print!(">>> ");

            let flush_result = io::stdout().flush(); // 刷新缓冲区（重要！）
            match flush_result {
                Ok(_) => {},
                Err(e) => {eprintln!("{}", e.to_string()); continue;}
            }

            let read_line_result = io::stdin().read_line(&mut code);
            match read_line_result {
                Err(e) => {eprintln!("{}", e.to_string()); continue;},
                Ok(_) => {}
            }

            let result = eval(code, "repl".to_string(), &mut env);
            if let Some(it) = result {
                println!("{}", it.inspect())
            }
        }
    }
}