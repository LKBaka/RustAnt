
use std::sync::{Arc, Mutex};

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

            let get_all_exports: Symbol<fn() -> AntClass> = match lib.get(b"get_all_exports") {
                Ok(it) => it,
                Err(err) => return Err(err.to_string())
            };

            Ok(get_all_exports())
        }
    }
}

pub struct LoadedLibrary {
    pub path: String,
    pub library: Library,
}

pub static LOADED_LIBRARIES: Lazy<Arc<Mutex<Vec<LoadedLibrary>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

fn ensure_library_loaded(path: &str) -> Result<&'static Library, Error> {
    let mut libraries = LOADED_LIBRARIES.lock().unwrap();

    // 检查是否已经加载
    for loaded_lib in libraries.iter() {
        if loaded_lib.path == path {
            return unsafe { Ok(std::mem::transmute(&loaded_lib.library)) };
        }
    }

    // 加载新库并保持
    let library = unsafe {
        let _ = Library::new(path);
        Library::new(path)?
    };

    libraries.push(LoadedLibrary {
        path: path.to_string(),
        library,
    });
    
    // 返回静态引用（安全因为Vec不会被移动）
    unsafe { Ok(std::mem::transmute(libraries.last().unwrap())) }
}