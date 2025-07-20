use std::fs;

use crate::environment::utils::create_top_env;
use crate::runner::eval::eval;
use crate::arg_structure::arg_structure::Args;

use super::utils::import_all_modules;

pub struct FileRunner {
    file_path: String,
    args: Args
}

impl FileRunner {
    pub fn new(file_path: String, args: Args) -> Self {
        FileRunner{
            file_path, args
        }
    }

    pub fn run(&self) {
        // 读取文件
        let contents = fs::read_to_string(&self.file_path);
        match contents {
            Ok(contents) => {
                let mut env = create_top_env();
                
                import_all_modules(&mut env);
                
                #[cfg(feature = "get_code_run_seconds")]
                use std::time::Instant;

                #[cfg(feature = "get_code_run_seconds")]
                let start = Instant::now();

                eval(contents, format!("main - file: {}", self.file_path).to_string(), &mut env, &self.args);

                #[cfg(feature = "get_code_run_seconds")]
                println!(
                    "{}", format!(
                        "Code run time: {} seconds, {} milliseconds, {} nanoseconds", 
                        start.elapsed().as_secs_f64(), start.elapsed().as_millis(), start.elapsed().as_nanos()
                    )
                );
            }
            Err(e) => {eprintln!("{}", e.to_string()); return;}
        }
    }
}