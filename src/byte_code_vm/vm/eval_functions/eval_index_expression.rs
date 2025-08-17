use bigdecimal::BigDecimal;

use num_traits::Signed;
use num_traits::cast::ToPrimitive;

use crate::{
    big_dec,
    object::{ant_array::AntArray, ant_int::AntInt, object::INT, object::Object},
};

fn eval_array_index_expression(arr: &AntArray, index: &AntInt) -> Result<Object, String> {
    if !index.value.is_integer() {
        return Err(format!("unsupported array index: {}", index.value));
    }

    let absolute_index = if index.value.is_positive() || index.value == big_dec!(0) {
        &index.value
    } else {
        &(big_dec!(arr.items.len() as u128) + &index.value)
    };

    if absolute_index >= &big_dec!(usize::MAX as u128) {
        return Err(format!("index too big! index: {}", absolute_index));
    }

    if absolute_index >= &big_dec!(arr.items.len() as u128) || index.value < big_dec!(0) {
        return Err(format!(
            "index out of range, index: {}, array length: {}",
            absolute_index,
            arr.items.len()
        ));
    }

    Ok(arr.items[absolute_index.to_usize().unwrap()].clone())
}

pub fn eval_index_expression(obj: Object, index: Object) -> Result<Object, String> {
    if let Some(arr) = obj.as_any().downcast_ref::<AntArray>() {
        return if let Some(index) = index.as_any().downcast_ref::<AntInt>() {
            eval_array_index_expression(arr, index)
        } else {
            Err(format!(
                "list indices must be {INT}, not {}",
                obj.get_type()
            ))
        };
    } else {
        Err(format!("object {:?} is not a subscriptable", obj))
    }
}
