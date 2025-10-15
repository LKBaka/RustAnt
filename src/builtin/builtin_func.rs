use std::{
    cell::RefCell,
    cmp::Ordering,
    process::exit,
    rc::Rc,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use bigdecimal::BigDecimal;

use crate::{
    builtin::builtin_classes::{option_class::OPTION, range_class::RANGE, result_class::RESULT},
    byte_code_vm::{
        utils::native_boolean_to_object,
        vm::{
            eval_functions::eval_infix_operator::{eq_native_ref, gt_native_ref},
            vm::Vm,
        },
    },
    obj_enum::object::Object,
    object::{
        ant_double::AntDouble,
        ant_int::AntInt,
        ant_method::{Method, MethodType},
        ant_string::AntString,
        object::{DOUBLE, I64, IAntObject, INT, STRING},
    },
    utils::run_command,
};

pub fn builtin_print(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let obj = args[0].borrow();

    #[cfg(target_arch = "wasm32")]
    use crate::println;

    println!("{}", obj.inspect());

    Ok(None)
}

pub fn builtin_len(_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let obj = args[0].borrow();

    match &*obj {
        Object::AntArray(arr) => Ok(Some(Object::AntInt(AntInt::from(arr.items.len())))),

        Object::AntString(s) => Ok(Some(Object::AntInt(AntInt::from(s.value.chars().count())))),

        _ => Err(format!(
            "expected an array or string of function len, got: {}",
            obj.inspect()
        )),
    }
}

pub fn builtin_copy(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let obj = args[0].borrow().clone();

    Ok(Some(obj))
}

pub fn builtin_id(_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let obj = args[0].borrow();

    Ok(Some(Object::AntInt(AntInt::from(obj.get_id()))))
}

pub fn builtin_obj_info(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let obj = args[0].borrow();

    Ok(Some(Object::AntString(AntString::new(format!("{obj:#?}")))))
}

pub fn builtin_shell(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let obj = args[0].borrow();

    match &*obj {
        Object::AntString(s) => {
            let _ = run_command(&s.value);
            Ok(None)
        }

        _ => Err(format!(
            "expected an string to execute, got: {}",
            obj.inspect()
        )),
    }
}

pub fn builtin_clear(
    __vm: &mut Vm,
    _args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    #[cfg(windows)]
    let _ = run_command("cls");

    #[cfg(not(windows))]
    let _ = run_command("clear");

    Ok(None)
}

pub fn builtin_force_exit(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let obj = args[0].borrow();

    match &*obj {
        Object::AntInt(ret_val) => {
            use bigdecimal::ToPrimitive;

            exit(if let Some(val) = ret_val.value.to_i32() {
                val
            } else {
                -1
            })
        }

        _ => Err(format!(
            "expected an integer to return, got: {}",
            obj.inspect()
        )),
    }
}

pub fn builtin_now(
    __vm: &mut Vm,
    _args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    Ok(Some(Object::AntInt(AntInt::from(BigDecimal::from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )))))
}

pub fn builtin_create_method(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let to_be_method = args[0].clone();
    let borrowed = to_be_method.borrow();

    match &*borrowed {
        Object::Closure(cl) => Ok(Some(Object::Method(Method {
            me: None,
            func: MethodType::Closure(cl.clone()),
        }))),
        Object::AntNativeFunction(f) => Ok(Some(Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(f.clone()),
        }))),
        _ => Err(format!("cannot convert {} to method", borrowed.inspect())),
    }
}

pub fn builtin_range(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let max_num = args[0].borrow().clone();
    if max_num.get_type() != INT {
        return Err(format!("expected an integer, got: {}", max_num.inspect()));
    }

    let mut new_range = RANGE.clone();
    new_range
        .map
        .insert("next_num".into(), Object::AntInt(AntInt::from(-1)));

    new_range.map.insert("max_num".into(), max_num);

    Ok(Some(Object::AntClass(new_range)))
}

pub fn builtin_panic(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let err = args[0].borrow().clone();
    if err.get_type() != STRING {
        return Err(format!("expected an string, got: {}", err.inspect()));
    }

    Err(format!("panic: \"{}\"", err.inspect()))
}

pub fn builtin_str(_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let s = args[0].borrow().clone();

    Ok(Some(Object::AntString(AntString::new(s.inspect()))))
}

pub fn builtin_double(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let n = args[0].borrow();

    let expected_types: [&str; 4] = [INT, I64, DOUBLE, STRING];

    match &*n {
        Object::AntI64(i) => Ok(Some(Object::AntDouble(AntDouble::from(i.value)))),
        Object::AntInt(i) => Ok(Some(Object::AntDouble(AntDouble::from(i.value.clone())))),
        Object::AntString(s) => Ok(Some(Object::AntDouble(AntDouble::from({
            match BigDecimal::from_str(&s.value) {
                Ok(it) => it,
                Err(err) => return Err(err.to_string()),
            }
        })))),
        Object::AntDouble(d) => Ok(Some(Object::AntDouble(d.clone()))),

        it => Err(format!(
            "expected type {:#?}, got: {}",
            expected_types,
            it.inspect()
        )),
    }
}

pub fn builtin_int(_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let n = args[0].borrow();

    let expected_types: [&str; 4] = [INT, I64, DOUBLE, STRING];

    match &*n {
        Object::AntI64(i) => Ok(Some(Object::AntInt(AntInt::from(i.value)))),
        Object::AntInt(i) => Ok(Some(Object::AntInt(AntInt::from(i.value.clone())))),
        Object::AntDouble(d) => Ok(Some(Object::AntInt(AntInt::from(d.value.with_scale(0))))),
        Object::AntString(s) => Ok(Some(Object::AntInt(AntInt::from({
            match BigDecimal::from_str(&s.value) {
                Ok(it) => it.with_scale(0),
                Err(err) => return Err(err.to_string()),
            }
        })))),

        it => Err(format!(
            "expected type {:#?}, got: {}",
            expected_types,
            it.inspect()
        )),
    }
}

pub fn builtin_ok(_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let value = args[0].borrow().clone();

    Ok(Some(ant_ok(value)))
}

pub fn builtin_err(_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let err = args[0].borrow().clone();

    Ok(Some(ant_err(err)))
}

pub fn builtin_some(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let value = args[0].borrow().clone();

    Ok(Some(ant_some(value)))
}

pub fn ant_ok(value: Object) -> Object {
    let mut new_result = RESULT.clone();
    new_result.map.insert("value".into(), value);

    Object::AntClass(new_result)
}

pub fn ant_err(err: Object) -> Object {
    let mut new_result = RESULT.clone();
    new_result.map.insert("err".into(), err);

    Object::AntClass(new_result)
}

#[inline(always)]
pub fn ant_null() -> Object {
    Object::AntClass(OPTION.clone()) // Option Class normal state is Null
}

pub fn ant_some(val: Object) -> Object {
    let mut new_option = OPTION.clone();
    new_option
        .map
        .insert("is_null".into(), native_boolean_to_object(false));
    new_option.map.insert("value".into(), val);

    Object::AntClass(new_option)
}

pub fn builtin_sorted(
    _vm: &mut Vm,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let mut arr = match args[0].borrow().clone() {
        Object::AntArray(arr) => arr,
        it => return Err(format!("expected an array to sort, got: {}", it.inspect())),
    };

    let mut err = None;

    arr.items.sort_by(
        // 调换 l, r 实现小于
        |l, r| match gt_native_ref(r, l) {
            Ok(less_than) => {
                if less_than {
                    return Ordering::Less;
                }

                match eq_native_ref(l, r) {
                    Ok(eq) => {
                        if eq {
                            Ordering::Equal
                        } else {
                            Ordering::Greater
                        }
                    }
                    Err(eq_err) => {
                        err = Some(eq_err);
                        Ordering::Equal
                    }
                }
            }
            Err(it) => {
                err = Some(it);
                Ordering::Equal
            }
        },
    );

    match err {
        None => Ok(Some(Object::AntArray(arr))),
        Some(it) => Err(it),
    }
}
