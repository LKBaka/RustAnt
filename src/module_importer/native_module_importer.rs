use std::{collections::HashMap, sync::Mutex};

use libloading::Error;

#[cfg(not(target_family = "wasm"))]
use libloading::{Library, Symbol};

use once_cell::sync::Lazy;

use crate::object::ant_class::AntClass;

pub struct NativeModuleImporter {
    pub file: String
}

#[cfg(not(target_family = "wasm"))]
impl NativeModuleImporter {
    pub fn import(&self) -> Result<AntClass, String> {
        use std::path::PathBuf;

        use crate::{constants::MODULE_PATHS, obj_enum::object::Object, object::ant_string::AntString};

        let o = Object::AntString(AntString::new(
            PathBuf::from(&self.file)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        ));

        MODULE_PATHS.lock().unwrap().items.push(o);

        unsafe {
            let lib = match ensure_library_loaded(&self.file) {
                Ok(it) => it,
                Err(err) => return Err(err.to_string())
            };

            let get_all_exports: Symbol<fn() -> AntClass> = match lib.get(b"get_all_exports\0") {
                Ok(it) => it,
                Err(err) => return Err(err.to_string())
            };

            Ok(get_all_exports())
        }
    }
}

#[cfg(not(target_family = "wasm"))]
static LOADED: Lazy<Mutex<HashMap<String, &'static Library>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[cfg(not(target_family = "wasm"))]
pub fn ensure_library_loaded(path: &str) -> Result<&'static Library, Error> {
    let mut map = LOADED.lock().unwrap();

    if let Some(&lib) = map.get(path) {
        return Ok(lib);
    }

    let lib = Box::leak(Box::new(unsafe { Library::new(path) }?));
    map.insert(path.to_owned(), lib);
    Ok(lib)
}