use std::path::Path;

use crate::{environment::{data::Data, data_info::DataInfo, environment::Environment}, module_system::import_module::ModuleImporter};

pub fn import_all_modules(env: &mut Environment) {
    let mut module_importer = ModuleImporter::new(
        "result".to_string(),
        vec![
            {
                let mut path_buf = std::env::current_exe()
                    .unwrap_or_else(|_| Path::new(".").to_path_buf())
                    .parent()   
                    .expect("failed to get parent directory of current executable")
                    .to_path_buf();

                path_buf.push("modules");

                path_buf                    
                    .to_str()
                    .expect("failed to convert current directory to string")
                    .to_string()
            }
        ]
    );

    let result_module = module_importer.import();
    if result_module.is_err() {
        eprintln!(
            "error importing module: {}, module_paths: {}", 
            result_module.err().unwrap(), 
            module_importer.module_paths.join(", ")
        );

        return;
    }

    env.create("result", Data::new(result_module.unwrap(), DataInfo::new(false)));
}