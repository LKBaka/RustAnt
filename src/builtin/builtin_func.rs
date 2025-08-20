use std::{cell::RefCell, rc::Rc};

use crate::{convert_type, object::{ant_array::AntArray, ant_int::AntInt, object::Object}};

pub fn builtin_print(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    let obj = args[0]
        .borrow()
        .clone();

    println!("{}", obj.inspect());

    None
}

pub fn builtin_len(args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    let obj = args[0]
        .borrow()
        .clone();

    let arr = convert_type!(AntArray, obj);

    Some(Box::new(AntInt::from(arr.items.len())))
}