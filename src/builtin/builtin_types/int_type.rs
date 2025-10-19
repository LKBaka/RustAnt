use std::{collections::HashMap, str::FromStr};

use bigdecimal::{BigDecimal, ParseBigDecimalError};
use once_cell::sync::Lazy;

use crate::{
    builtin::builtin_func::{ant_err, ant_ok},
    byte_code_vm::{constants::NONE_OBJ, vm::vm::Vm},
    obj_enum::object::Object,
    object::{
        ant_class::AntClass,
        ant_int::AntInt,
        ant_method::{Method, MethodType},
        ant_native_function::create_ant_native_function,
        ant_string::AntString,
        object::IAntObject,
    },
};

#[derive(Debug)]
enum ParseIntErrorType {
    InvalidDigit,
    Empty,
    Other,
}

fn to_string(
    _vm: &mut Vm,
    args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let o = args[0].borrow();

    let me = match &*o {
        Object::AntInt(int) => int,
        _ => return Err(format!("expected an integer (self) got: {}", o.inspect())),
    };

    Ok(Some(Object::AntString(AntString::from(
        me.value.to_string(),
    ))))
}

fn create_parse_int_error(ty: ParseIntErrorType, msg: String) -> AntClass {
    let mut new_err = PARSE_INT_ERROR.clone();

    new_err.map.insert(
        "type".to_string(),
        Object::AntString(AntString::from(format!("{:?}", ty))),
    );
    new_err.map.insert(
        "message".to_string(),
        Object::AntString(AntString::from(msg)),
    );

    new_err
}

fn parse_from_str(
    _vm: &mut Vm,
    args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let o = args[0].borrow();

    let s = match &*o {
        Object::AntString(s) => s,
        _ => return Err(format!("expected an string, got: {}", o.inspect())),
    };

    if s.value.is_empty() {
        return Ok(Some(ant_err(Object::AntClass(create_parse_int_error(
            ParseIntErrorType::Empty,
            "empty string".to_string(),
        )))));
    }

    let result = BigDecimal::from_str(&s.value);

    match result {
        Ok(it) => Ok(Some(ant_ok(Object::AntInt(AntInt::from(it.with_scale(0)))))),
        Err(it) => Ok(Some(ant_err(Object::AntClass(create_parse_int_error(
            match &it {
                ParseBigDecimalError::Other(_) => ParseIntErrorType::Other,
                _ => ParseIntErrorType::InvalidDigit,
            },
            it.to_string(),
        ))))),
    }
}

pub static INT_MEMBERS: Lazy<HashMap<String, Object>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(
        "to_string".to_string(),
        Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(create_ant_native_function(None, to_string)),
        }),
    );

    m.insert(
        "parse".to_string(),
        Object::AntNativeFunction(create_ant_native_function(None, parse_from_str)),
    );

    m
});

pub static PARSE_INT_ERROR: Lazy<AntClass> = Lazy::new(|| {
    let mut m = HashMap::new();

    fn to_string(
        _vm: &mut Vm,
        args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>,
    ) -> Result<Option<Object>, String> {
        let o = args[0].borrow();

        let me = match &*o {
            Object::AntClass(clazz) => clazz,
            _ => return Err(format!("expected an class (self) got: {}", o.inspect())),
        };

        let ty = match me.map.get("type") {
            Some(it) => it,
            None => return Err(format!("object '{}' has no field 'type'", me.inspect())),
        };

        let msg = match me.map.get("message") {
            Some(it) => it,
            None => return Err(format!("object '{}' has no field 'message'", me.inspect())),
        };

        Ok(Some(Object::AntString(AntString::from(format!(
            "ParseIntError {{ type: {}, message: \"{}\" }}",
            ty.inspect(),
            msg.inspect()
        )))))
    }

    m.insert("type".to_string(), NONE_OBJ.clone());
    m.insert("message".to_string(), NONE_OBJ.clone());

    m.insert(
        "to_string".to_string(),
        Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(create_ant_native_function(None, to_string)),
        }),
    );

    AntClass::from(m)
});
