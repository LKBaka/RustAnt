use std::{cell::RefCell, process::exit, rc::Rc, time::{SystemTime, UNIX_EPOCH}};

use bigdecimal::BigDecimal;

use crate::{obj_enum::object::Object, object::{ant_int::AntInt, ant_method::{Method, MethodType}, ant_string::AntString, object::IAntObject}, utils::run_command};

pub fn builtin_print(args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let obj = args[0]
        .borrow();

    println!("{}", obj.inspect());

    Ok(None)
}

pub fn builtin_len(args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let obj = args[0]
        .borrow();

    match &*obj {
        Object::AntArray(arr) => {
            Ok(Some(Object::AntInt(AntInt::from(arr.items.len()))))
        }

        Object::AntString(s) => {
            Ok(Some(Object::AntInt(AntInt::from(s.value.chars().count()))))
        }

        _ => Err(format!("expected an array or string of function len, got: {}", obj.inspect()))
    }
}

pub fn builtin_copy(args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let obj = args[0]
        .borrow()
        .clone();

    Ok(Some(obj))
}

pub fn builtin_id(args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let obj = args[0]
        .borrow();

    Ok(Some(Object::AntInt(AntInt::from(obj.get_id()))))
}

pub fn builtin_obj_info(args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let obj = args[0]
        .borrow();

    Ok(Some(Object::AntString(AntString::new(format!("{obj:#?}")))))
}

pub fn builtin_shell(args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let obj = args[0]
        .borrow();

    match &*obj {
        Object::AntString(s) => {
            let _ = run_command(&s.value);
            Ok(None)
        }

        _ => Err(format!("expected an string to execute, got: {}", obj.inspect())) 
    }
}

pub fn builtin_clear(_args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    #[cfg(windows)]
    let _ = run_command("cls");

    #[cfg(not(windows))]
    let _ = run_command("clear");

    Ok(None)
}

pub fn builtin_force_exit(args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let obj = args[0]
        .borrow();

    match &*obj {
        Object::AntInt(ret_val) => {
            use bigdecimal::ToPrimitive;

            exit(
                if let Some(val) = ret_val.value.to_i32() {
                    val
                } else {
                    -1
                }
            )
        }

        _ => Err(format!("expected an integer to return, got: {}", obj.inspect())) 
    }
}

pub fn builtin_now(_args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    Ok(Some(Object::AntInt(AntInt::from(
        BigDecimal::from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        )
    ))))
}

pub fn builtin_create_method(args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let to_be_method = args[0].clone();
    let borrowed = to_be_method.borrow();

    match &*borrowed {
        Object::Closure(cl) => Ok(Some(Object::Method(Method {
            me: None,
            func: MethodType::Closure(cl.clone())
        }))),
        Object::AntNativeFunction(f) => Ok(Some(Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(f.clone())
        }))),
        _ => Err(format!("cannot convert {} to method", borrowed.inspect()))
    }
}
