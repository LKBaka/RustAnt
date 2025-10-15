use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    byte_code_vm::{constants::NONE_OBJ, utils::native_boolean_to_object, vm::vm::Vm},
    function_caller::native_to_call_api::native_to_call,
    obj_enum::object::Object,
    object::{
        ant_class::AntClass,
        ant_method::{Method, MethodType},
        ant_native_function::create_ant_native_function,
        object::IAntObject,
    },
    rc_ref_cell,
};

pub static OPTION: Lazy<AntClass> = Lazy::new(|| {
    AntClass::from({
        let unwrap_func = |
            _vm: &mut Vm, args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>
        | {
            let o = args[0].borrow();

            let me = match &*o {
                Object::AntClass(clazz) => clazz,
                _ => return Err(format!("expected an class (self) got: {}", o.inspect())),
            };

            let is_none = match me
                .map
                .get("is_null")
                .ok_or_else(|| format!("object '{}' has no field 'is_null'", me.inspect()))?
            {
                Object::AntBoolean(boolean) => boolean.value,
                it => Err(format!(
                    "expected field 'is_null' a boolean, got: {}",
                    it.inspect()
                ))?,
            };

            if is_none {
                return Err(format!("unwrap failed: unwrap an null value"));
            }

            let value = me
                .map
                .get("value")
                .ok_or_else(|| format!("object '{}' has no field 'value'", me.inspect()))?;

            return Ok(Some(value.clone()));
        };

        let when_some_func = |
            vm: &mut Vm,
            args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>
        | -> Result<Option<Object>, String> {
            let o = args[0].borrow();

            let me = match &*o {
                Object::AntClass(clazz) => clazz,
                _ => return Err(format!("expected an class (self) got: {}", o.inspect())),
            };

            let is_null = match me
                .map
                .get("is_null")
                .ok_or_else(|| format!("object '{}' has no field 'is_null'", me.inspect()))?
            {
                Object::AntBoolean(boolean) => boolean.value,
                it => Err(format!(
                    "expected field 'is_null' a boolean, got: {}",
                    it.inspect()
                ))?,
            };

            if is_null {
                return Ok(None);
            }

            let value = me
                .map
                .get("is_null")
                .ok_or_else(|| format!("object '{}' has no field 'is_null'", me.inspect()))?;

            if args.len() > 1 {
                let callback = args[1].clone();

                native_to_call(vm, callback, vec![rc_ref_cell!(value.clone())])?;

                return Ok(match vm.pop() {
                    Some(obj) => Some(obj.borrow().clone()),
                    None => None,
                });
            }

            Ok(None)
        };

        let when_null_func = |
            vm: &mut Vm,
            args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>
        | -> Result<Option<Object>, String> {
            let o = args[0].borrow();

            let me = match &*o {
                Object::AntClass(clazz) => clazz,
                _ => return Err(format!("expected an class (self) got: {}", o.inspect())),
            };

            let is_null = match me
                .map
                .get("is_null")
                .ok_or_else(|| format!("object '{}' has no field 'is_null'", me.inspect()))?
            {
                Object::AntBoolean(boolean) => boolean.value,
                it => Err(format!(
                    "expected field 'is_null' a boolean, got: {}",
                    it.inspect()
                ))?,
            };

            if !is_null {
                return Ok(None);
            }

            if args.len() > 1 {
                let callback = args[1].clone();

                native_to_call(vm, callback, vec![])?;

                return Ok(match vm.pop() {
                    Some(obj) => Some(obj.borrow().clone()),
                    None => None,
                });
            }

            Ok(None)
        };

        let mut m = HashMap::new();

        m.insert("value".into(), NONE_OBJ.clone());
        m.insert("is_null".into(), native_boolean_to_object(true));

        m.insert(
            "unwrap".into(),
            Object::Method(Method {
                me: None,
                func: MethodType::NativeFunction(create_ant_native_function(None, unwrap_func)),
            }),
        );

        m.insert(
            "when_some".into(),
            Object::Method(Method {
                me: None,
                func: MethodType::NativeFunction(create_ant_native_function(
                    None, when_some_func
                )),
            }),
        );

        m.insert(
            "when_null".into(),
            Object::Method(Method {
                me: None,
                func: MethodType::NativeFunction(create_ant_native_function(
                    None, when_null_func
                )),
            }),
        );

        m
    })
});
