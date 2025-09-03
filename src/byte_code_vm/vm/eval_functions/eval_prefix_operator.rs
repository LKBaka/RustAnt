use bigdecimal::BigDecimal;

use crate::{
    byte_code_vm::{code::code::{OpCode, OP_BANG, OP_MINUS}, utils::native_boolean_to_object}, obj_enum::object::Object, object::{ant_double::AntDouble, ant_int::AntInt}
};

pub fn bang(right: Object) -> Result<Object, String> {
    match right {
        Object::AntBoolean(right) => Ok(native_boolean_to_object(!right.value)),
        Object::AntInt(right) => Ok(native_boolean_to_object(right.value == BigDecimal::from(0))),
        Object::AntDouble(right) => Ok(native_boolean_to_object(right.value == BigDecimal::from(0))),

        _ => Err(format!(
            "unimplemented for type: {:?}",
            right
        ))
    }
}

pub fn minus(right: Object) -> Result<Object, String> {
    match right {
        Object::AntBoolean(right) => Ok(Object::AntInt(AntInt::from(-(right.value as i32)))),
        Object::AntInt(right) => Ok(Object::AntInt(AntInt::from(-&right.value))),
        Object::AntDouble(right) => Ok(Object::AntDouble(AntDouble::from(-&right.value))),

        _ => Err(format!(
            "unimplemented for type: {:?}",
            right
        ))
    }
}

pub fn eval_prefix_operator(op: OpCode, right: Object) -> Result<Object, String> {
    match op {
        OP_BANG => bang(right),
        OP_MINUS => minus(right),

        _ => Err(format!("unknown prefix operator: {}", op)),
    }
}
