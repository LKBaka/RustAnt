use std::{cell::RefCell, rc::Rc, time::{SystemTime, UNIX_EPOCH}};

use bigdecimal::BigDecimal;

use crate::{obj_enum::object::Object, object::{ant_int::AntInt, object::IAntObject}};

pub fn builtin_print(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    let obj = args[0]
        .borrow();

    println!("{}", obj.inspect());

    None
}

pub fn builtin_len(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    let obj = args[0]
        .borrow();

    let arr = if let Object::AntArray(ref arr) = *obj {
        arr
    } else {
        panic!("expected an array of function len")
    };

    Some(Object::AntInt(AntInt::from(arr.items.len())))
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
