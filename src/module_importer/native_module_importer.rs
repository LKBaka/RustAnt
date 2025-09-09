use libloading::{Library, Symbol};

use crate::object::ant_class::AntClass;

pub struct NativeModuleImporter {
    pub file: String
}

impl NativeModuleImporter {
    pub fn import(&self) -> Result<AntClass, String> {
        unsafe {
            let lib = match Library::new(&self.file) {
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