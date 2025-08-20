use std::{cell::RefCell, rc::Rc};

use bigdecimal::Signed;
use num_traits::ToPrimitive;

use crate::{
    big_dec,
    object::{
        ant_array::AntArray,
        ant_int::AntInt,
        object::{ARRAY, INT, Object},
    },
};

fn eval_set_index_array(
    value: Object,
    index: Rc<RefCell<Object>>,
    target: Rc<RefCell<Object>>,
) -> Result<(), String> {
    let index_borrow = index.borrow_mut();

    let casted_index = index_borrow
        .as_any()
        .downcast_ref::<AntInt>()
        .expect(&format!("expected an integer, but got: {:?}", index));

    let mut target_borrow_mut = target.borrow_mut();

    let casted_target = target_borrow_mut
        .as_any_mut()
        .downcast_mut::<AntArray>()
        .expect(&format!("expected an array, but got: {:?}", index));

    let index = &casted_index.value;

    // index 检查
    if !index.is_integer() {
        return Err(format!("unsupported array index: {}", index));
    }

    let absolute_index = if index.is_positive() || *index == big_dec!(0) {
        &index
    } else {
        &(big_dec!(casted_target.items.len() as u128) + index)
    };

    if absolute_index >= &big_dec!(usize::MAX as u128) {
        return Err(format!("index too big! index: {}", absolute_index));
    }

    if absolute_index >= &big_dec!(casted_target.items.len() as u128) || *index < big_dec!(0) {
        return Err(format!(
            "index out of range, index: {}, array length: {}",
            absolute_index,
            casted_target.items.len()
        ));
    }

    casted_target.items[absolute_index.to_usize().unwrap()] = value;

    Ok(())
}

pub fn eval_set_index(
    value: Object,
    index: Rc<RefCell<Object>>,
    target: Rc<RefCell<Object>>,
) -> Result<(), String> {
    if target.borrow().get_type() == ARRAY && index.borrow().get_type() == INT {
        return eval_set_index_array(value, index, target);
    }

    Err(format!("cannot set index of object {:?}", target.borrow()))
}
