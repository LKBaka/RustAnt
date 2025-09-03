use lazy_static::lazy_static;

use crate::{builtin::builtin_func::{builtin_copy, builtin_len, builtin_now, builtin_print}, object::ant_native_function::{create_ant_native_function, AntNativeFunction}};

lazy_static! {
    pub static ref BUILTIN_MAP: hashbrown::HashMap<String, AntNativeFunction> = {
        let mut m = hashbrown::HashMap::new();

        m.insert(
            "print".into(),
            create_ant_native_function(None, builtin_print)
        );

        m.insert(
            "len".into(),
            create_ant_native_function(None, builtin_len)
        );

        m.insert(
            "copy".into(),
            create_ant_native_function(None, builtin_copy)
        );

        m.insert(
            "now".into(),
            create_ant_native_function(None, builtin_now)
        );

        m
    };

    pub static ref BUILTIN_MAP_INDEX: Vec<String> = {
        vec![
            "print".into(),
            "len".into(),
            "copy".into(),
            "now".into(),
        ]
    };
}   