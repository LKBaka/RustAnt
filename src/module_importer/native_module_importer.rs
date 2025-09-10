
use std::{collections::HashMap, sync::Mutex};

use libloading::{Error, Library, Symbol};
use once_cell::sync::Lazy;

use crate::object::ant_class::AntClass;

pub struct NativeModuleImporter {
    pub file: String
}

impl NativeModuleImporter {
    pub fn import(&self) -> Result<AntClass, String> {
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

static LOADED: Lazy<Mutex<HashMap<String, &'static Library>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn ensure_library_loaded(path: &str) -> Result<&'static Library, Error> {
    let mut map = LOADED.lock().unwrap();

    if let Some(&lib) = map.get(path) {
        return Ok(lib);
    }

    let lib = Box::leak(Box::new(unsafe { Library::new(path) }?));
    map.insert(path.to_owned(), lib);
    Ok(lib)
}