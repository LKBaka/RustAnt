use std::path::PathBuf;
use std::{path::Path, vec};

use uuid::Uuid;

use crate::object::object::AntObject;
use crate::object::utils::is_error;

use crate::{environment::utils::create_env, object::object::Object};

pub struct ModuleImporter {
    pub module_name: String,
    pub module_paths: Vec<String>,
}

impl ModuleImporter {
    pub fn new(module_name: String, module_paths: Vec<String>) -> Self {
        Self {
            module_name,
            module_paths,
        }
    }

    pub fn import(&mut self) -> Result<Object, String> {
        if self.module_paths.is_empty() {
            let current_dir = std::env::current_dir()
                .unwrap_or_else(|_| Path::new(".").to_path_buf());

            self.module_paths = vec![
                current_dir.to_str().expect(
                    "failed to convert current directory to string"
                ).to_string()
            ];
        }

        let module_file = self.find_module_file();
        match module_file {
            Ok(file) => {
                self.import_module(file)
            }
            Err(e) => return Err(e),
        }
    }       

    fn import_module(&self, module_file: String) -> Result<Object, String> {
        // 读取模块文件内容
        let code = std::fs::read_to_string(&module_file)
            .map_err(|e| format!("failed to read module file '{}': {}", module_file, e));
        
        if code.is_err() {
            return Err(code.err().unwrap());
        }

        // 创建一个新的环境来保存模块里的内容
        let env = create_env(vec![]);

        panic!("unimplented import_modules.rs");

        let o: Object = Box::new(crate::byte_code_vm::constants::TRUE.clone());

        if let Some(it) = Some(o) {
            if is_error(&it) {
                return Err(it.inspect());
            }
        }
        
        Ok(Box::new(AntObject {
            id: Uuid::new_v4(),
            env,
        }))
    }

    fn find_module_file(&self) -> Result<String, String> {
        for path in &self.module_paths {
            let mut module_file = PathBuf::new();

            module_file.push(path);
            module_file.push(format!("{}.ant", self.module_name));
            module_file.set_extension("ant");

            if Path::new(&module_file).exists() {
                return Ok(
                    module_file.to_str()
                        .expect("failed to convert module file path to string")
                        .to_string()
                );
            }
        }

        Err(format!("module '{}' not found", self.module_name))
    }
}