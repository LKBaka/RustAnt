use std::fs;
use std::path::Path;

use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::utils::create_top_env;
use crate::runner::eval::eval;
use crate::module_system::import_module::ModuleImporter;

use super::utils::import_all_modules;

pub struct FileRunner {
    file_path: String,
}

impl FileRunner {
    pub fn new(file_path: String) -> Self {
        FileRunner{
            file_path,
        }
    }

    pub fn run(&self) {
        // 读取文件
        let contents = fs::read_to_string(&self.file_path);
        match contents {
            Ok(contents) => {
                let mut env = create_top_env();
                
                import_all_modules(&mut env);
                
                eval(contents, format!("main - file: {}", self.file_path).to_string(), &mut env);
            }
            Err(e) => {eprintln!("{}", e.to_string()); return;}
        }
    }
}