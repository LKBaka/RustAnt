use std::{cell::RefCell, process::exit, rc::Rc, time::{SystemTime, UNIX_EPOCH}};

use bigdecimal::BigDecimal;

use crate::{obj_enum::object::Object, object::{ant_int::AntInt, ant_string::AntString, object::IAntObject}, utils::run_command};

pub fn builtin_print(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    let obj = args[0]
        .borrow();

    println!("{}", obj.inspect());

    None
}

pub fn builtin_len(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    let obj = args[0]
        .borrow();

    match &*obj {
        Object::AntArray(arr) => {
            Some(Object::AntInt(AntInt::from(arr.items.len())))
        }

        Object::AntString(s) => {
            Some(Object::AntInt(AntInt::from(s.value.chars().count())))
        }

        _ => panic!("expected an array or string of function len")
    }
}

pub fn builtin_copy(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    let obj = args[0]
        .borrow()
        .clone();

    Some(obj)
}

pub fn builtin_id(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    let obj = args[0]
        .borrow();

    Some(Object::AntInt(AntInt::from(obj.get_id())))
}

pub fn builtin_obj_info(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    let obj = args[0]
        .borrow();

    Some(Object::AntString(AntString::new(format!("{obj:#?}"))))
}

pub fn builtin_shell(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    let obj = args[0]
        .borrow();

    match &*obj {
        Object::AntString(s) => {
            let _ = run_command(&s.value);
        }

        _ => panic!("expected an string to execute") 
    }

    None
}

pub fn builtin_clear(_args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    #[cfg(windows)]
    let _ = run_command("cls");

    #[cfg(not(windows))]
    let _ = run_command("clear");

    None
}

pub fn builtin_force_exit(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
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

        _ => panic!("expected an integer to return") 
    }
}

pub fn builtin_now(_args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    Some(Object::AntInt(AntInt::from(
        BigDecimal::from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        )
    )))
}
