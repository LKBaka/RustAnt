use lazy_static::lazy_static;

use crate::{builtin::builtin_func::{builtin_clear, builtin_copy, builtin_force_exit, builtin_id, builtin_len, builtin_now, builtin_obj_info, builtin_print, builtin_shell}, object::ant_native_function::{create_ant_native_function, AntNativeFunction}};

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

        m.insert(
            "id".into(),
            create_ant_native_function(None, builtin_id)
        );

        m.insert(
            "obj_info".into(),
            create_ant_native_function(None, builtin_obj_info)
        );
        
        m.insert(
            "shell".into(),
            create_ant_native_function(None, builtin_shell)
        );
        
        m.insert(
            "clear".into(),
            create_ant_native_function(None, builtin_clear)
        );

        m.insert(
            "force_exit".into(),
            create_ant_native_function(None, builtin_force_exit)
        );

        m
    };

    pub static ref BUILTIN_MAP_INDEX: Vec<String> = {
        vec![
            "print".into(),
            "len".into(),
            "copy".into(),
            "now".into(),
            "id".into(),
            "obj_info".into(),
            "shell".into(),
            "clear".into(),
            "force_exit".into(),
        ]
    };
}   