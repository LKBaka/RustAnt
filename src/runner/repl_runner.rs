use std::io;
use std::io::Write;

use crate::arg_structure::arg_structure::Args;
use crate::environment::utils::create_top_env;
use crate::runner::eval::eval;

use super::utils::import_all_modules;

pub struct REPLRunner {
    args: Args
}

impl REPLRunner {
    pub fn new(args: Args) -> Self {
        REPLRunner {args}
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

            // 使用多行输入功能
            let full_code = self.read_multi_line(&mut code);
            
            // 如果用户输入为空，跳过执行
            if full_code.trim().is_empty() {
                continue;
            }

            let result = eval(full_code, "repl".to_string(), &mut env, &self.args);
            if let Some(it) = result {
                println!("{}", it.inspect())
            }
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
                matches!(ch, ',' | '=' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' | '!' | '<' | '>')
            }
            None => false
        }
    }
}