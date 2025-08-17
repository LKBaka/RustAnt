use bigdecimal::BigDecimal;

use crate::{
    byte_code_vm::code::code::{OP_BANG, OP_MINUS, OpCode},
    object::{ant_boolean::AntBoolean, ant_double::AntDouble, ant_int::AntInt, object::Object},
};

pub fn bang(right: Object) -> Result<Object, String> {
    let right_as_anyed = right.as_any();

    if let Some(right) = right_as_anyed.downcast_ref::<AntBoolean>() {
        return Ok(Box::new(AntBoolean::from(!right.value)));
    } else if let Some(right) = right_as_anyed.downcast_ref::<AntInt>() {
        return Ok(Box::new(AntBoolean::from(
            right.value == BigDecimal::from(0),
        )));
    } else if let Some(right) = right_as_anyed.downcast_ref::<AntDouble>() {
        return Ok(Box::new(AntBoolean::from(
            right.value == BigDecimal::from(0),
        )));
    }

    Err(format!(
        "unimplemented for type: {:?}",
        right_as_anyed.type_id()
    ))
}

pub fn minus(right: Object) -> Result<Object, String> {
    let right_as_anyed = right.as_any();

    if let Some(right) = right_as_anyed.downcast_ref::<AntBoolean>() {
        return Ok(Box::new(AntInt::from(-(right.value as i32))));
    } else if let Some(right) = right_as_anyed.downcast_ref::<AntInt>() {
        return Ok(Box::new(AntInt::from(-&right.value)));
    } else if let Some(right) = right_as_anyed.downcast_ref::<AntDouble>() {
        return Ok(Box::new(AntDouble::from(-&right.value)));
    }

    Err(format!(
        "unimplemented for type: {:?}",
        right_as_anyed.type_id()
    ))
}

pub fn eval_prefix_operator(op: OpCode, right: Object) -> Result<Object, String> {
    match op {
        OP_BANG => bang(right),
        OP_MINUS => minus(right),

        _ => Err(format!("unknown prefix operator: {}", op)),
    }
}
