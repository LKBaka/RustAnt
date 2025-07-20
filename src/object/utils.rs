use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::constants::{ant_true, ant_false};
use crate::environment::utils::create_env;
use crate::evaluator::evaluator::Evaluator;
use crate::function_caller::function_caller::call_function_with_name;
use crate::object::ant_double::AntDouble;
use crate::object::ant_error::AntError;
use crate::object::ant_int::AntInt;
use crate::object::object::{IAntObject, ERROR};

use super::ant_class::AntClass;
use super::ant_string::AntString;
use super::object::Object;

pub fn is_native_error(obj: &Object) -> bool {
    obj.get_type() == ERROR
}

pub fn is_error(obj: &Object) -> bool {
    is_native_error(obj)
}

pub fn is_truthy(obj: Object) -> bool {
    // 明确处理 ant_true/ant_false
    if &obj == &*ant_true {
        true
    } else if &obj == &*ant_false {
        false   
    } else if let Some(obj) = obj.as_any().downcast_ref::<AntInt>() {
        // 针对 AntInt 类型的特别优化
        !(obj.value == BigDecimal::from(0))
    } else if let Some(obj) = obj.as_any().downcast_ref::<AntDouble>() {
        // 针对 AntDouble 类型的特别优化
        !(obj.value == BigDecimal::from(0))
    } else {
        let result = call_function_with_name("__bool__".into(), &vec![], &mut Evaluator::new(), obj.clone().get_env_ref());
        if let Err(_err) = result {
            return false
        }

        if let Ok(result) = result { 
            result.is_none() || {
                let result = result.expect(
                    &format!("__bool__ method should return a value. obj: {}", obj.inspect())
                );

                &result == &*ant_true
            }
        } else {false}
    }
}

pub fn create_error(message: String) -> Object {
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

pub fn create_error_with_name(error_name: &'static str, message: String) -> Object {
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

pub fn unsupported_operand_type_err(op: &'static str, left_type: String, right_type: String) -> Object {
    create_error_with_name(
        "TypeError", 
        format!("TypeError: unsupported operand type(s) for {}: '{}' and '{}'", op , left_type, right_type)
    )
}

pub fn type_error(msg: &str) -> Object {
    create_error_with_name(
        "TypeError", 
        msg.into()
    )
}

pub fn type_eq(left: Object, right: Object) -> bool {
    if left.get_id() == right.get_id() {
        return true;
    }

    if left.get_type() == right.get_type() {
        return true;
    }

    if !left.as_any().is::<AntClass>() {
        return false;
    }

    if !right.as_any().is::<AntClass>() {
        return false;
    }

    let left_obj = left.as_any().downcast_ref::<AntClass>().unwrap();
    let right_obj = right.as_any().downcast_ref::<AntClass>().unwrap();

    if left_obj.get_type() == right_obj.get_type() {
        return true;
    }

    if let Some(base) = &left_obj.base {
        if right_obj.base.is_none() {return false;}

        return base.get_type() == right_obj.base.as_ref().unwrap().get_type()
    }

    false
}

pub fn check_error_name(error: &Object, error_name: &'static str) -> bool {
    if let Some(err) = error.as_any().downcast_ref::<AntError>() {
        return err.error_name == error_name
    }

    false
}

pub fn not_implemented_error(message: &str) -> Object {
    create_error_with_name(
        "NotImplementedError", 
        message.into()
    )
}

pub fn not_implemented_error_with_name(name: &str) -> Object {
    create_error_with_name(
        "NotImplementedError", 
        format!("{} is not implemented", name)
    )
}
