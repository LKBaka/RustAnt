use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    byte_code_vm::vm::vm::Vm, constants::MODULE_PATHS, module_importer::{
        ant_module_importer::AntModuleImporter, native_module_importer::NativeModuleImporter,
    }, obj_enum::object::Object, object::ant_class::AntClass
};

pub struct ModuleImporter<'a> {
    pub vm: &'a mut Vm
}

impl<'a> ModuleImporter<'a> {
    pub fn import(&mut self, imports: Vec<&str>) -> Vec<Result<AntClass, String>> {
        let mut results = vec![];

        let mut module_paths: Vec<String> = vec![];

        for module_path in &MODULE_PATHS.lock().unwrap().items {
            if let Object::AntString(s) = module_path {
                module_paths.push(s.value.clone());
            }
        }

        let mut loaded = vec![];

        for module_path in module_paths {
            for import in &imports {
                if loaded.contains(import) {
                    continue;
                }

                let path_buf =
                    PathBuf::from_str(&module_path).expect(&format!("invaild path: {module_path}"));

                let mut try_folder = path_buf.clone();
                try_folder.push(import);

                let mut try_ant_mod = path_buf.clone();
                try_ant_mod.push(format!("{import}.ant"));

                let mut try_native_mod = path_buf.clone();

                #[cfg(target_os = "windows")]
                try_native_mod.push(format!("{import}.dll"));

                #[cfg(target_os = "linux")]
                try_native_mod.push(format!("{import}.so"));

                if try_ant_mod.exists() {
                    results.push(self.import_ant_module(&try_ant_mod));
                    loaded.push(&import);
                    continue;
                }

                #[cfg(not(target_family = "wasm"))]
                if try_native_mod.exists() {
                    results.push(Self::import_native_module(&try_native_mod));
                    loaded.push(&import);
                    continue;
                }

                if try_folder.exists() {
                    results.push(self.import_folder(&try_folder));
                    loaded.push(&import);
                    continue;
                }
            }
        }

        results
    }

    fn import_ant_module(&mut self, file: &Path) -> Result<AntClass, String> {
        let mut ant_mod_importer = AntModuleImporter {
            file: file.to_str().unwrap().to_string(),
            vm: self.vm
        };

        ant_mod_importer.import()
    }

    #[cfg(not(target_family = "wasm"))]
    fn import_native_module(file: &Path) -> Result<AntClass, String> {
        let native_mod_importer = NativeModuleImporter {
            file: file.to_str().unwrap().to_string(),
        };

        native_mod_importer.import()
    }

    /*
    原谅我 这坨函数写的跟屎一样的多层缩进
    God forgive me for writing this function with multiple layers of indentation like shit
    */
    fn import_folder(&mut self, path: &Path) -> Result<AntClass, String> {
        let mut m = HashMap::new();

        fn walk(
            importer: &mut ModuleImporter, 
            dir: &Path, 
            map: &mut HashMap<String, Object>
        ) -> Result<(), String> {
            let entries = fs::read_dir(dir).map_err(|e| format!("read_dir failed: {e}"))?;

            for entry in entries {
                let entry = entry.map_err(|e| format!("dir entry failed: {e}"))?;
                let path = entry.path();

                let file_name = path.file_stem().unwrap().to_str().unwrap().to_string();

                if path.is_dir() {
                    let mut dir_map = HashMap::new();

                    // 递归子目录
                    walk(importer, &path, &mut dir_map)?;

                    let dir_obj = AntClass::from(dir_map);

                    map.insert(file_name, Object::AntClass(dir_obj));
                } else if path
                    .extension()
                    .and_then(|s| Some(s.to_ascii_lowercase().to_str().unwrap().to_string()))
                    == Some(String::from("ant"))
                {
                    map.insert(
                        file_name,
                        Object::AntClass(importer.import_ant_module(&path)?),
                    );
                } else if path
                    .extension()
                    .and_then(|s| Some(s.to_ascii_lowercase().to_str().unwrap().to_string()))
                    == Some(String::from("dll"))
                    || path
                        .extension()
                        .and_then(|s| Some(s.to_ascii_lowercase().to_str().unwrap().to_string()))
                        == Some(String::from("so"))
                    || path
                        .extension()
                        .and_then(|s| Some(s.to_ascii_lowercase().to_str().unwrap().to_string()))
                        == Some(String::from("dylib"))
                {
                    #[cfg(not(target_family = "wasm"))]
                    map.insert(
                        file_name,
                        Object::AntClass(ModuleImporter::import_native_module(&path)?),
                    );
                }
            }

            Ok(())
        }

        walk(self, path, &mut m)?;

        Ok(AntClass::from(m))
    }
}
