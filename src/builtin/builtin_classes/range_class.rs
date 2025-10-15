use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    builtin::builtin_func::{ant_null, ant_some},
    byte_code_vm::{constants::NONE_OBJ, vm::vm::Vm},
    obj_enum::object::Object,
    object::{
        ant_class::AntClass,
        ant_int::AntInt,
        ant_method::{Method, MethodType},
        ant_native_function::create_ant_native_function,
        object::IAntObject,
    },
};

pub static RANGE: Lazy<AntClass> = Lazy::new(|| {
    AntClass::from({
        let next = |_vm: &mut Vm, args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>| {
            let mut o = args[0].borrow_mut();

            let me = match &mut *o {
                Object::AntClass(clazz) => clazz,
                _ => return Err(format!("expected an class (self) got: {}", o.inspect())),
            };

            let max_num = match match me.map.get("max_num") {
                Some(it) => it,
                None => return Err(format!("object '{}' has no field 'max_num'", me.inspect())),
            } {
                Object::AntInt(int) => int,
                it => return Err(format!("expected an integer object, got: {}", it.inspect())),
            };

            let next_num = match match me.map.get("next_num") {
                Some(it) => it,
                None => return Err(format!("object '{}' has no field 'next_num'", me.inspect())),
            } {
                Object::AntInt(int) => int,
                it => return Err(format!("expected an integer object, got: {}", it.inspect())),
            };

            if next_num.value < &max_num.value - 1 {
                let next_num = &next_num.value + 1;

                let next_num_obj = Object::AntInt(AntInt::from(next_num));

                me.map.insert("next_num".into(), next_num_obj.clone());
                Ok(Some(ant_some(next_num_obj)))
            } else {
                Ok(Some(ant_null()))
            }
        };

        let mut m = HashMap::new();

        m.insert("next_num".into(), NONE_OBJ.clone());
        m.insert("max_num".into(), NONE_OBJ.clone());

        m.insert(
            "next".into(),
            Object::Method(Method {
                me: None,
                func: MethodType::NativeFunction(create_ant_native_function(None, next)),
            }),
        );

        m
    })
});
