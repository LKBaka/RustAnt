use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    builtin::builtin_func::{
        builtin_clear, builtin_copy, builtin_create_method, builtin_force_exit, builtin_id, builtin_len, builtin_now, builtin_obj_info, builtin_print, builtin_shell
    },
    byte_code_vm::constants::NONE_OBJ,
    obj_enum::object::Object,
    object::{
        ant_class::AntClass, ant_native_function::create_ant_native_function, object::IAntObject,
    },
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
            Object::AntNativeFunction(create_ant_native_function(None, builtin_create_method))
        );

        m.insert(
            "Result".into(),
            Object::AntClass(AntClass::from({
                let native_func = |args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>| {
                    let o = args[0].borrow();

                    let me = match &*o {
                        Object::AntClass(clazz) => clazz,
                        _ => return Err(format!("expected an class (self) got {}", o.inspect())),
                    };

                    let err = match me.map.get("err") {
                        Some(it) => it,
                        None => return Err(format!("object '{}' has no field 'err'", me.inspect()))
                    };

                    if &*err == &*NONE_OBJ {
                        let value = me.map.get("value").ok_or_else(|| {
                            format!("object '{}' has no field 'value'", me.inspect(),)
                        })?;

                        return Ok(Some(value.clone()));
                    }

                    Err(format!("unwrap failed: {}", err.inspect()))
                };

                let mut m = HashMap::new();

                m.insert("value".into(), NONE_OBJ.clone());
                m.insert("err".into(), NONE_OBJ.clone());

                m.insert(
                    "unwrap".into(),
                    Object::AntNativeFunction(create_ant_native_function(None, native_func)),
                );

                m
            })),
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
            "Result".into(),
            "method".into(),
        ]
    };
}
