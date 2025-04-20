use std::fs;
use crate::environment::utils::create_top_env;
use crate::runner::eval::eval;

pub struct FileRunner {
    file_path: String,
}

impl FileRunner {
    pub(crate) fn new(file_path: String) -> Self {
        FileRunner{
            file_path,
        }
    }

    pub(crate) fn run(&self) {
        // 读取文件
        let contents = fs::read_to_string(&self.file_path);
        match contents {
            Ok(contents) => {
                eval(contents, format!("main - file: {}", self.file_path).to_string(), &mut create_top_env());
            }
            Err(e) => {eprintln!("{}", e.to_string()); return;}
        }
    }
}