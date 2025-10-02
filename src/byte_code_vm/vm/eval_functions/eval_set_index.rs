use std::{cell::RefCell, rc::Rc};

use bigdecimal::Signed;
use num_traits::ToPrimitive;

use crate::{
    big_dec, obj_enum::object::Object, object::object::{IAntObject, ARRAY, HASH_MAP, INT}, try_unwrap_ref
};

fn eval_set_index_array(
    value: Object,
    index: Rc<RefCell<Object>>,
    target: Rc<RefCell<Object>>,
) -> Result<(), String> {
    let casted_index = try_unwrap_ref!(index, Object::AntInt(index))
        .expect(&format!("expected an integer, but got: {:?}", index));

    let index = &casted_index.value;

    // 直接借用 target 并在匹配到 AntArray 时就地修改 items，保持直接可变性
    let mut target_borrow = target.borrow_mut();

    match &mut *target_borrow {
        Object::AntArray(arr) => {

            // index 检查
            if !index.is_integer() {
                return Err(format!("unsupported array index: {}", index));
            }

            // 计算绝对索引（处理负数索引）
            let absolute_index = if index.is_positive() || *index == big_dec!(0) {
                index.clone()
            } else {
                big_dec!(arr.items.len() as u128) + index.clone()
            };

            if absolute_index >= big_dec!(usize::MAX as u128) {
                return Err(format!("index too big! index: {}", index));
            }

            if absolute_index >= big_dec!(arr.items.len() as u128) || absolute_index < big_dec!(0) {
                return Err(format!(
                    "index out of range, index: {}, array length: {}",
                    index,
                    arr.items.len()
                ));
            }

            // 就地替换数组元素，保持直接可变性
            arr.items[absolute_index.to_usize().unwrap()] = value;

            return Ok(());
        }

        _ => panic!("expected an array, but got: {:?}", target),
    }
}

fn eval_set_value_hash_map(
    value: Object,
    index: Rc<RefCell<Object>>,
    target: Rc<RefCell<Object>>,
) -> Result<(), String> {
    let key = index.borrow().clone();

    // 直接借用 target 并在匹配到 AntArray 时就地修改 items，保持直接可变性
    let mut target_borrow = target.borrow_mut();

    match *target_borrow {
        Object::AntHashMap(ref mut map) => {
            map.map.insert(key, value);

            Ok(())
        }

        _ => panic!("expected an hash map, but got: {:?}", target),
    }
}

pub fn eval_set_index(
    value: Object,
    index: Rc<RefCell<Object>>,
    target: Rc<RefCell<Object>>,
) -> Result<(), String> {
    if target.borrow().get_type() == ARRAY && index.borrow().get_type() == INT {
        return eval_set_index_array(value, index, target);
    } else if target.borrow().get_type() == HASH_MAP {
        return eval_set_value_hash_map(value, index, target);
    } 

    Err(format!("cannot set index of object {:?}", target.borrow()))
}
