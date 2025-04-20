use num_bigint::BigInt;
use crate::constants::ant_true;
use crate::object::ant_function::AntFunction;
use crate::object::ant_int::AntInt;
use crate::object::object::{IAntObject, ERROR, INT, NULL, UNINIT};

pub fn object_type_to_string(string: &str) -> String {
    String::from(string)
}

pub fn is_eq_functions(
    left_func_name: String, right_func_name: String,
    left: Box<dyn IAntObject>, right: Box<dyn IAntObject>
) -> bool {
    if left.as_any().downcast_ref::<AntFunction>().is_none() {
        return false
    }

    if right.as_any().downcast_ref::<AntFunction>().is_none() {
        return false
    }

    let left_function = left.as_any().downcast_ref::<AntFunction>().unwrap();
    let right_function = left.as_any().downcast_ref::<AntFunction>().unwrap();

    left_func_name == right_func_name &&
    left_function.env == right_function.env &&
    left_function.param_env == right_function.param_env
}

pub fn is_native_error(obj: Box<dyn IAntObject>) -> bool {
    obj.get_type() == ERROR
}

pub fn is_error(obj: Box<dyn IAntObject>) -> bool {
    is_native_error(obj)
}

pub fn is_truthy(obj: Box<dyn IAntObject>) -> bool {
    obj.eq(&*ant_true.clone()) || (
        if obj.get_type() == INT.to_string(){
            if let Some(it) = obj.as_any().downcast_ref::<AntInt>().cloned() {
                it.value != BigInt::from(0)
            } else {false}
        } else {false} && obj.get_type() != NULL.to_string() && obj.get_type() != UNINIT.to_string()
    )
}