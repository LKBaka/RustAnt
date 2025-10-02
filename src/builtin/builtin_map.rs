use lazy_static::lazy_static;

use crate::{
    builtin::builtin_func::{
        builtin_clear, builtin_copy, builtin_create_method, builtin_double, builtin_err,
        builtin_force_exit, builtin_id, builtin_int, builtin_len, builtin_now, builtin_obj_info,
        builtin_ok, builtin_panic, builtin_print, builtin_range, builtin_shell, builtin_sorted,
        builtin_str,
    },
    obj_enum::object::Object,
    object::ant_native_function::create_ant_native_function,
};

lazy_static! {
    pub static ref BUILTIN_MAP: hashbrown::HashMap<String, Object> = {
        let mut m = hashbrown::HashMap::new();

        m.insert(
            "print".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_print)),
        );

        m.insert(
            "len".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_len)),
        );

        m.insert(
            "copy".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_copy)),
        );

        m.insert(
            "now".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_now)),
        );

        m.insert(
            "id".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_id)),
        );

        m.insert(
            "obj_info".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_obj_info)),
        );

        m.insert(
            "shell".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_shell)),
        );

        m.insert(
            "clear".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_clear)),
        );

        m.insert(
            "force_exit".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_force_exit)),
        );

        m.insert(
            "method".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_create_method)),
        );

        m.insert(
            "range".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_range)),
        );

        m.insert(
            "panic".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_panic)),
        );

        m.insert(
            "str".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_str)),
        );

        m.insert(
            "double".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_double)),
        );

        m.insert(
            "int".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_int)),
        );

        m.insert(
            "sorted".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_sorted)),
        );

        m.insert(
            "Ok".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_ok)),
        );

        m.insert(
            "Err".into(),
            Object::AntNativeFunction(create_ant_native_function(None, builtin_err)),
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
            "method".into(),
            "range".into(),
            "panic".into(),
            "str".into(),
            "int".into(),
            "double".into(),
            "sorted".into(),
            "Ok".into(),
            "Err".into(),
        ]
    };
}
