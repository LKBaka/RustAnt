use num_bigint::BigInt;
use uuid::Uuid;

use crate::constants::{ant_true, ant_false};
use crate::environment::utils::create_env;
use crate::object::ant_error::AntError;
use crate::object::ant_function::AntFunction;
use crate::object::ant_int::AntInt;
use crate::object::object::{IAntObject, ERROR};

use super::ant_string::AntString;
use super::object::{INT, NULL, UNINIT};

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

pub fn is_native_error(obj: &Box<dyn IAntObject>) -> bool {
    obj.get_type() == ERROR
}

pub fn is_error(obj: &Box<dyn IAntObject>) -> bool {
    is_native_error(obj)
}

pub fn is_truthy(obj: Box<dyn IAntObject>) -> bool {
    // 明确处理 ant_true/ant_false
    if obj == ant_true.clone() {
        true
    } else if obj == ant_false.clone() {
        false
    } else {
        // 处理其他类型
        match obj.get_type().as_str() {
            INT => {
                if let Some(it) = obj.as_any().downcast_ref::<AntInt>() {
                    it.value != BigInt::from(0)
                } else {
                    false
                }
            },
            NULL | UNINIT => false,
            _ => true // 其他非空对象为 true
        }
    }
}

pub fn create_error(message: String) -> Box<dyn IAntObject> {
    Box::new(
        AntError {
            id: Uuid::new_v4(),
            env: create_env(vec![
                ("error_name".to_string(), AntString::new_with_native_value(Box::new("Error".to_string()))),
                ("message".to_string(), AntString::new_with_native_value(Box::new(message.to_string()))),
            ]),
            error_name: "Error".to_string(),
            message
        }
    )
}

pub fn create_error_with_name(error_name: &'static str, message: String) -> Box<dyn IAntObject> {
    Box::new(
        AntError {
            id: Uuid::new_v4(),
            env: create_env(vec![
                ("error_name".to_string(), AntString::new_with_native_value(Box::new(error_name.to_string()))),
                ("message".to_string(), AntString::new_with_native_value(Box::new(message.clone()))),
            ]),
            error_name: error_name.to_string(),
            message
        }
    )
}

pub fn unsupported_operand_type_err(op: &'static str, left_type: String, right_type: String) -> Box<dyn IAntObject> {
    create_error_with_name(
        "TypeError", 
        format!("TypeError: unsupported operand type(s) for {}: '{}' and '{}'", op , left_type, right_type)
    )
}