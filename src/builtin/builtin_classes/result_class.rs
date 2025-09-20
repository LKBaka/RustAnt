use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    byte_code_vm::constants::NONE_OBJ,
    obj_enum::object::Object,
    object::{
        ant_class::AntClass,
        ant_method::{Method, MethodType},
        ant_native_function::create_ant_native_function,
        object::IAntObject,
    },
};

pub static RESULT: Lazy<AntClass> = Lazy::new(|| {
    AntClass::from({
        let native_func = |args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>| {
            let o = args[0].borrow();

            let me = match &*o {
                Object::AntClass(clazz) => clazz,
                _ => return Err(format!("expected an class (self) got {}", o.inspect())),
            };

            let err = match me.map.get("err") {
                Some(it) => it,
                None => return Err(format!("object '{}' has no field 'err'", me.inspect())),
            };

            if &*err == &*NONE_OBJ {
                let value = me
                    .map
                    .get("value")
                    .ok_or_else(|| format!("object '{}' has no field 'value'", me.inspect(),))?;

                return Ok(Some(value.clone()));
            }

            Err(format!("unwrap failed: {}", err.inspect()))
        };

        let mut m = HashMap::new();

        m.insert("value".into(), NONE_OBJ.clone());
        m.insert("err".into(), NONE_OBJ.clone());

        m.insert(
            "unwrap".into(),
            Object::Method(Method {
                me: None,
                func: MethodType::NativeFunction(create_ant_native_function(None, native_func)),
            }),
        );

        m
    })
});
