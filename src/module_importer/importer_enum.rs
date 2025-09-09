use std::{path::{Path, PathBuf}, str::FromStr};

use crate::{constants::MODULE_PATHS, module_importer::ant_module_importer::AntModuleImporter, obj_enum::object::Object, object::ant_class::AntClass};

pub struct ModuleImporter;

impl ModuleImporter {
    pub fn import(imports: Vec<&str>) -> Vec<Result<AntClass, String>> {
        let mut results = vec![];

        let mut module_paths: Vec<String> = vec![];

        for module_path in &MODULE_PATHS
            .lock()
            .unwrap()
            .items 
        {
            if let Object::AntString(s) = module_path {
                module_paths.push(s.value.clone());
            }
        }

        for module_path in module_paths  {
            for import in &imports {
                let path_buf = PathBuf::from_str(&module_path)
                    .expect(&format!("invaild path: {module_path}"));

                let mut try_ant_mod = path_buf.clone();
                try_ant_mod.push(format!("{import}.ant"));

                // let mut try_native_mod = path_buf.clone();
                // try_native_mod.push(format!("{import}.dll"));

                if try_ant_mod.exists() {
                    results.push(Self::import_ant_module(&try_ant_mod));
                    continue;
                }
            }
        }

        results
    }

    fn import_ant_module(file: &Path) -> Result<AntClass, String> {
        let ant_mod_importer = AntModuleImporter {
            file: file
                .to_str()
                .unwrap()
                .to_string()
        };

        ant_mod_importer.import()
    }
}