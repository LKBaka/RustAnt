use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    byte_code_vm::{constants::NONE_OBJ, vm::vm::Vm},
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

pub static RESULT: Lazy<AntClass> = Lazy::new(|| {
    AntClass::from(("Result", {
        let unwrap_func = |_vm: &mut Vm, args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>| {
            let o = args[0].borrow();

            let me = match &*o {
                Object::AntClass(clazz) => clazz,
                _ => return Err(format!("expected an class (self) got: {}", o.inspect())),
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

        let when_ok_func = |vm: &mut Vm,
                            args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>|
         -> Result<Option<Object>, String> {
            let o = args[0].borrow();

            let me = match &*o {
                Object::AntClass(clazz) => clazz,
                _ => return Err(format!("expected an class (self) got: {}", o.inspect())),
            };

            let err = match me.map.get("err") {
                Some(it) => it,
                None => return Err(format!("object '{}' has no field 'err'", me.inspect())),
            };

            if &*err == &*NONE_OBJ {
                let value = me
                    .map
                    .get("value")
                    .ok_or_else(|| format!("object '{}' has no field 'value'", me.inspect()))?;

                if args.len() > 1 {
                    let callback = args[1].clone();

                    native_to_call(vm, callback, vec![rc_ref_cell!(value.clone())])?;

                    return Ok(match vm.pop() {
                        Some(obj) => Some(obj.borrow().clone()),
                        None => None,
                    });
                }
            }

            Ok(None)
        };

        let when_err_func = |vm: &mut Vm,
                             args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>|
         -> Result<Option<Object>, String> {
            let o = args[0].borrow();

            let me = match &*o {
                Object::AntClass(clazz) => clazz,
                _ => return Err(format!("expected an class (self) got: {}", o.inspect())),
            };

            let err = match me.map.get("err") {
                Some(it) => it,
                None => return Err(format!("object '{}' has no field 'err'", me.inspect())),
            };

            if &*err != &*NONE_OBJ {
                if args.len() > 1 {
                    let callback = args[1].clone();

                    native_to_call(vm, callback, vec![rc_ref_cell!(err.clone())])?;

                    return Ok(match vm.pop() {
                        Some(obj) => Some(obj.borrow().clone()),
                        None => None,
                    });
                }
            }

            Ok(None)
        };

        let mut m = HashMap::new();

        m.insert("value".into(), NONE_OBJ.clone());
        m.insert("err".into(), NONE_OBJ.clone());

        m.insert(
            "unwrap".into(),
            Object::Method(Method {
                me: None,
                func: MethodType::NativeFunction(create_ant_native_function(None, unwrap_func)),
            }),
        );

        m.insert(
            "when_ok".into(),
            Object::Method(Method {
                me: None,
                func: MethodType::NativeFunction(create_ant_native_function(None, when_ok_func)),
            }),
        );

        m.insert(
            "when_err".into(),
            Object::Method(Method {
                me: None,
                func: MethodType::NativeFunction(create_ant_native_function(None, when_err_func)),
            }),
        );

        m
    }))
});
