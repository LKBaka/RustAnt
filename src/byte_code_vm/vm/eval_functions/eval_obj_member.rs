use std::{cell::RefCell, rc::Rc};

use crate::{
    function_caller::native_to_call_api::native_to_call,
    byte_code_vm::vm::vm::Vm,
    obj_enum::object::Object,
    object::{ant_string::AntString, object::IAntObject},
    rc_ref_cell,
};

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

    return Err(format!(
        "expected an class to get field, got: {}",
        o_borrow.inspect()
    ));
}
