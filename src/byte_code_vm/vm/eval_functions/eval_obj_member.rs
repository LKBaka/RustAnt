use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    builtin::builtin_types::BUILTIN_TYPE_MAP,
    byte_code_vm::vm::vm::Vm,
    function_caller::native_to_call_api::native_to_call,
    obj_enum::object::Object,
    object::{ant_string::AntString, object::IAntObject},
    rc_ref_cell,
};

pub fn eval_native_obj_member(
    vm: &mut Vm,
    obj: Rc<RefCell<Object>>,
    map: &HashMap<String, Object>,
    field: String,
) -> Result<(), String> {
    if let Some(__get__) = map.get("__get__") {
        match __get__ {
            Object::Method(method) => {
                let mut m = method.clone();

                m.me = Some(obj.clone());

                let o = rc_ref_cell!(Object::Method(m));

                native_to_call(
                    vm,
                    o.clone(),
                    vec![rc_ref_cell!(Object::AntString(AntString::new(field)))],
                )?;

                return Ok(());
            }

            _ => (),
        }
    }

    if let Err(msg) = vm.push(
        match match map.get(&field) {
            Some(it) => it,
            None => Err(format!(
                "object '{}' has no field '{field}'",
                obj.borrow().inspect()
            ))?,
        } {
            Object::Method(method) => {
                let mut m = method.clone();

                m.me = Some(obj.clone());

                rc_ref_cell!(Object::Method(m))
            }
            other => rc_ref_cell!(other.clone()),
        },
    ) {
        return Err(format!("error push field: {msg}"));
    } else {
        Ok(())
    }
}

pub fn eval_obj_member(vm: &mut Vm, obj: Rc<RefCell<Object>>, field: String) -> Result<(), String> {
    // 天哪那么多缩进我不会被拉去皮豆吧
    let o_borrow = obj.borrow();

    if let Object::AntClass(clazz) = &*o_borrow {
        if let Some(__get__) = clazz.map.get("__get__") {
            match __get__ {
                Object::Method(method) => {
                    let mut m = method.clone();

                    m.me = Some(obj.clone());

                    let o = rc_ref_cell!(Object::Method(m));

                    native_to_call(
                        vm,
                        o.clone(),
                        vec![rc_ref_cell!(Object::AntString(AntString::new(field)))],
                    )?;

                    return Ok(());
                }

                _ => (),
            }
        }

        let value = match match clazz.map.get(&field) {
            Some(it) => it,
            None => Err(format!(
                "object '{}' has no field '{}'",
                clazz.inspect(),
                field
            ))?,
        } {
            Object::Method(method) => {
                let mut m = method.clone();

                m.me = Some(obj.clone());

                Object::Method(m)
            }
            other => other.clone(),
        };

        if let Err(msg) = vm.push(rc_ref_cell!(value)) {
            return Err(format!("error push field: {msg}"));
        }

        return Ok(());
    }

    if let Some(m) = BUILTIN_TYPE_MAP.get(&o_borrow.get_type()) {
        return eval_native_obj_member(vm, obj.clone(), m, field);
    }

    return Err(format!(
        "expected an class to get field, got: {}",
        o_borrow.inspect()
    ));
}
