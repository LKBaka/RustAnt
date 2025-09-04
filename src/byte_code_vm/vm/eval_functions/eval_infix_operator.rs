use bigdecimal::BigDecimal;

use crate::{
    byte_code_vm::{
        code::code::{OP_ADD, OP_DIVIDE, OP_EQ, OP_GT, OP_MULTIPLY, OP_NOTEQ, OP_SUBTRACT, OpCode},
        utils::native_boolean_to_object,
    },
    obj_enum::object::Object,
    object::{ant_double::AntDouble, ant_int::AntInt, ant_string::AntString, object::IAntObject},
};

// 保留原本注释与语义：对多种数值/字符串类型做互操作
fn add_native(left: Object, right: Object) -> Result<Object, String> {
    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => {
            Ok(Object::AntInt(AntInt::from(&l.value + &r.value)))
        }
        (Object::AntDouble(l), Object::AntDouble(r)) => {
            Ok(Object::AntDouble(AntDouble::from(&l.value + &r.value)))
        }
        (Object::AntInt(l), Object::AntDouble(r)) => {
            Ok(Object::AntDouble(AntDouble::from(&l.value + &r.value)))
        }
        (Object::AntDouble(l), Object::AntInt(r)) => {
            Ok(Object::AntDouble(AntDouble::from(&l.value + &r.value)))
        }
        (Object::AntString(l), Object::AntString(r)) => {
            Ok(Object::AntString(AntString::new(l.value + &r.value)))
        }

        (l, r) => Err(format!(
            "unimplemented for types: {} and {}",
            l.get_type(),
            r.get_type()
        )),
    }
}

fn subtract_native(left: Object, right: Object) -> Result<Object, String> {
    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => {
            Ok(Object::AntInt(AntInt::from(&l.value - &r.value)))
        }
        (Object::AntDouble(l), Object::AntDouble(r)) => {
            Ok(Object::AntDouble(AntDouble::from(&l.value - &r.value)))
        }
        (Object::AntInt(l), Object::AntDouble(r)) => {
            Ok(Object::AntDouble(AntDouble::from(&l.value - &r.value)))
        }
        (Object::AntDouble(l), Object::AntInt(r)) => {
            Ok(Object::AntDouble(AntDouble::from(&l.value - &r.value)))
        }

        (l, r) => Err(format!(
            "unimplemented for types: {} and {}",
            l.get_type(),
            r.get_type()
        )),
    }
}

fn multiply_native(left: Object, right: Object) -> Result<Object, String> {
    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => {
            Ok(Object::AntInt(AntInt::from(&l.value * &r.value)))
        }
        (Object::AntDouble(l), Object::AntDouble(r)) => {
            Ok(Object::AntDouble(AntDouble::from(&l.value * &r.value)))
        }
        (Object::AntInt(l), Object::AntDouble(r)) => {
            Ok(Object::AntDouble(AntDouble::from(&l.value * &r.value)))
        }
        (Object::AntDouble(l), Object::AntInt(r)) => {
            Ok(Object::AntDouble(AntDouble::from(&l.value * &r.value)))
        }

        (l, r) => Err(format!(
            "unimplemented for types: {} and {}",
            l.get_type(),
            r.get_type()
        )),
    }
}

fn divide_native(left: Object, right: Object) -> Result<Object, String> {
    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => {
            if r.value == BigDecimal::from(0) {
                return Err("division by zero".to_string());
            }

            let result = &l.value / &r.value;
            if result.is_integer() {
                Ok(Object::AntInt(AntInt::from(result)))
            } else {
                Ok(Object::AntDouble(AntDouble::from(result)))
            }
        }

        (Object::AntDouble(l), Object::AntDouble(r)) => {
            if r.value == BigDecimal::from(0) {
                return Err("division by zero".to_string());
            }

            let result = &l.value / &r.value;
            if result.is_integer() {
                Ok(Object::AntInt(AntInt::from(result)))
            } else {
                Ok(Object::AntDouble(AntDouble::from(result)))
            }
        }

        (Object::AntInt(l), Object::AntDouble(r)) => {
            if r.value == BigDecimal::from(0) {
                return Err("division by zero".to_string());
            }

            let result = &l.value / &r.value;
            Ok(Object::AntDouble(AntDouble::from(result)))
        }

        (Object::AntDouble(l), Object::AntInt(r)) => {
            if r.value == BigDecimal::from(0) {
                return Err("division by zero".to_string());
            }

            let result = &l.value / &r.value;
            Ok(Object::AntDouble(AntDouble::from(result)))
        }

        (l, r) => Err(format!(
            "unimplemented for types: {} and {}",
            l.get_type(),
            r.get_type()
        )),
    }
}

fn gt_native(left: Object, right: Object) -> Result<Object, String> {
    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => Ok(native_boolean_to_object(&l.value > &r.value)),
        (Object::AntDouble(l), Object::AntDouble(r)) => {
            Ok(native_boolean_to_object(&l.value > &r.value))
        }
        (Object::AntInt(l), Object::AntDouble(r)) => {
            Ok(native_boolean_to_object(&l.value > &r.value))
        }
        (Object::AntDouble(l), Object::AntInt(r)) => {
            Ok(native_boolean_to_object(&l.value > &r.value))
        }

        (l, r) => Err(format!(
            "unimplemented for types: {} and {}",
            l.get_type(),
            r.get_type()
        )),
    }
}

fn eq_native(left: Object, right: Object) -> Result<Object, String> {
    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => {
            Ok(native_boolean_to_object(&l.value == &r.value))
        }
        (Object::AntDouble(l), Object::AntDouble(r)) => {
            Ok(native_boolean_to_object(&l.value == &r.value))
        }
        (Object::AntInt(l), Object::AntDouble(r)) => {
            Ok(native_boolean_to_object(&l.value == &r.value))
        }
        (Object::AntDouble(l), Object::AntInt(r)) => {
            Ok(native_boolean_to_object(&l.value == &r.value))
        }
        (Object::AntBoolean(l), Object::AntBoolean(r)) => {
            Ok(native_boolean_to_object(l.value == r.value))
        }

        (l, r) => Err(format!(
            "unimplemented for types: {} and {}",
            l.get_type(),
            r.get_type()
        )),
    }
}

fn not_eq_native(left: Object, right: Object) -> Result<Object, String> {
    match eq_native(left, right) {
        Ok(obj) => {
            if let Object::AntBoolean(b) = obj {
                return Ok(native_boolean_to_object(!b.value));
            }

            Err(format!("expected a boolean object, got: {:?}", obj))
        }

        Err(e) => Err(e),
    }
}

pub fn eval_infix_operator(op: OpCode, left: Object, right: Object) -> Result<Object, String> {
    // 移除类型检查限制，允许不同类型的操作数进行互操作
    match op {
        OP_ADD => add_native(left, right),
        OP_SUBTRACT => subtract_native(left, right),
        OP_MULTIPLY => multiply_native(left, right),
        OP_DIVIDE => divide_native(left, right),
        OP_GT => gt_native(left, right),
        OP_EQ => eq_native(left, right),
        OP_NOTEQ => not_eq_native(left, right),

        _ => Err(format!("unknown operator: {}", op)),
    }
}
