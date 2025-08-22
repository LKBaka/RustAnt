use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::constants::{ant_false, ant_true};

use crate::object::ant_double::AntDouble;
use crate::object::ant_error::AntError;
use crate::object::ant_int::AntInt;
use crate::object::object::ERROR;

use super::object::Object;

pub fn is_native_error(obj: &Object) -> bool {
    obj.get_type() == ERROR
}

pub fn is_error(obj: &Object) -> bool {
    is_native_error(obj)
}

pub fn is_truthy(obj: &Object) -> bool {
    if obj == &*ant_true {
        true
    } else if obj == &*ant_false {
        false
    } else if let Some(obj) = obj.as_any().downcast_ref::<AntInt>() {
        !(obj.value == BigDecimal::from(0))
    } else if let Some(obj) = obj.as_any().downcast_ref::<AntDouble>() {
        !(obj.value == BigDecimal::from(0))
    } else {
        false
    }
}

pub fn create_error(message: String) -> Object {
    Box::new(AntError {
        id: Uuid::new_v4(),
        error_name: "Error".to_string(),
        message,
    })
}

pub fn create_error_with_name(error_name: &'static str, message: String) -> Object {
    Box::new(AntError {
        id: Uuid::new_v4(),
        error_name: error_name.to_string(),
        message,
    })
}

pub fn unsupported_operand_type_err(
    op: &'static str,
    left_type: String,
    right_type: String,
) -> Object {
    create_error_with_name(
        "TypeError",
        format!(
            "TypeError: unsupported operand type(s) for {}: '{}' and '{}'",
            op, left_type, right_type
        ),
    )
}

#[macro_export]
macro_rules! convert_type_use_box {
    ($t:ty, $value:expr) => {{
        let value = Box::new($value) as Box<dyn Any>;

        let converted = value.downcast_ref::<$t>().expect(&format!(
            "cannot convert '{:?}' to type '{}'",
            $value,
            std::any::type_name::<$t>()
        ));

        converted.clone()
    }};
}

#[macro_export]
macro_rules! convert_type_to_owned {
    ($t:ty, $value:expr) => {{
        let value_format = format!("{:?}", $value);

        let value = $value as Box<dyn std::any::Any>;

        let converted = value.downcast::<$t>().expect(&format!(
            "cannot convert '{}' to type '{}'",
            value_format,
            std::any::type_name::<$t>()
        ));

        *converted
    }};
}

#[macro_export]
macro_rules! convert_type {
    ($t:ty, $value:expr) => {{
        use std::any::Any;

        let value = $value.as_ref() as &dyn Any;

        let converted = value.downcast_ref::<$t>().expect(&format!(
            "cannot convert '{:?}' to type '{}'",
            $value,
            std::any::type_name::<$t>()
        ));

        converted.clone()
    }};
}

#[macro_export]
macro_rules! big_dec {
    ($value:expr) => {
        bigdecimal::BigDecimal::from($value)
    };
}

#[macro_export]
macro_rules! big_dec_from_str {
    ($value:expr) => {{
        use std::str::FromStr;
        
        bigdecimal::BigDecimal::from_str(stringify!($value)).unwrap()
    }};
}
