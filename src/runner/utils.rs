use std::path::Path;

use crate::{
    environment::{data::Data, data_info::DataInfo, environment::Environment},
    module_system::import_module::ModuleImporter,
};

pub fn import_all_modules(env: &mut Environment) {
    let module_path = vec![{
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
    }];

    let modules = vec!["result", "option"];

    for module in modules {
        let mut module_importer = ModuleImporter::new(module.to_string(), module_path.clone());

        let m = module_importer.import();
        if m.is_err() {
            eprintln!(
                "error importing module: {}, module_paths: {}",
                m.err().unwrap(),
                module_importer.module_paths.join(", ")
            );

            continue;
        }

        env.create(module, Data::new(m.unwrap(), DataInfo::new(false)));
    }
}
