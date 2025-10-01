use std::cell::RefCell;
use std::rc::Rc;

use bigdecimal::BigDecimal;

use crate::constants::{ant_false, ant_true};

use crate::obj_enum::object::Object;
use crate::object::ant_error::AntError;
use crate::object::id_counter::next_id;
use crate::object::object::{IAntObject, ERROR};

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
    } else if let Object::AntInt(obj) = obj {
        !(obj.value == BigDecimal::from(0))
    } else if let Object::AntDouble(obj) = obj {
        !(obj.value == BigDecimal::from(0))
    } else if let Object::AntNone(_) = obj {
        false
    } else if let Object::AntString(s) = obj {
        !s.value.is_empty()
    } else if let Object::AntArray(arr) = obj {
        !arr.items.is_empty()
    } else if let Object::AntHashMap(map) = obj {
        !map.map.is_empty()
    } else {
        false
    }
}

#[inline(always)]
pub fn rrc_is_truthy(obj: &Rc<RefCell<Object>>) -> bool {
    let obj = &*obj.borrow();

    if obj == &*ant_true {
        true
    } else if obj == &*ant_false {
        false
    } else if let Object::AntInt(obj) = obj {
        obj.value != BigDecimal::from(0)
    } else if let Object::AntDouble(obj) = obj {
        obj.value != BigDecimal::from(0)
    } else if let Object::AntNone(_) = obj {
        false
    } else if let Object::AntString(s) = obj {
        !s.value.is_empty()
    } else if let Object::AntArray(arr) = obj {
        !arr.items.is_empty()
    } else if let Object::AntHashMap(map) = obj {
        !map.map.is_empty()
    } else {
        false
    }
}


pub fn create_error(message: String) -> Object {
    Object::AntError(AntError {
        id: next_id(),
        error_name: "Error".to_string(),
        message,
    })
}

pub fn create_error_with_name(error_name: &'static str, message: String) -> Object {
    Object::AntError(AntError {
        id: next_id(),
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
macro_rules! convert_type_ref {
    ($t:ty, $value:expr) => {{
        let value = $value as &dyn std::any::Any;

        let converted = value.downcast_ref::<$t>().expect(&format!(
            "cannot convert '{:?}' to type '{}'",
            $value,
            std::any::type_name::<$t>()
        ));

        converted.clone()
    }};
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

#[macro_export]
macro_rules! try_unwrap {
    ($o:expr, Object::$variant:ident($binding:pat)) => {{
        match $o {
            Object::$variant(inner) => Some(inner),
            _ => None,
        }
    }};
}

#[macro_export]
macro_rules! try_unwrap_ref {
    ($rrc:expr, Object::$variant:ident($binding:pat)) => {{
        let o = $rrc.borrow().clone();

        match o {
            Object::$variant(inner) => Some(inner),
            _ => None,
        }
    }};
}