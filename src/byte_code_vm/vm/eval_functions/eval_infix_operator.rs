use std::{cell::RefCell, rc::Rc};

use bigdecimal::BigDecimal;

use crate::{
    byte_code_vm::{
        code::code::{OpCode, OP_ADD, OP_DIVIDE, OP_EQ, OP_GT, OP_MULTIPLY, OP_NOTEQ, OP_SUBTRACT}, constants::{FALSE_OBJ, TRUE_OBJ}, utils::native_boolean_to_object
    },
    obj_enum::object::Object,
    object::{ant_double::AntDouble, ant_int::AntInt, ant_string::AntString, object::{IAntObject, NULL}},
};

// 保留原本注释与语义：对多种数值/字符串类型做互操作
fn add_native(left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
    let left = &*left.borrow();
    let right = &*right.borrow();

    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => Ok(Object::AntInt(AntInt::from(&l.value + &r.value))),
        (Object::AntDouble(l), Object::AntDouble(r)) => Ok(Object::AntDouble(AntDouble::from(&l.value + &r.value))),
        (Object::AntInt(l), Object::AntDouble(r)) => Ok(Object::AntDouble(AntDouble::from(&l.value + &r.value))),
        (Object::AntDouble(l), Object::AntInt(r)) => Ok(Object::AntDouble(AntDouble::from(&l.value + &r.value))),
        (Object::AntString(l), Object::AntString(r)) => Ok(Object::AntString(AntString::new(l.value.clone() + &r.value))),

        (l, r) => Err(format!("unimplemented for types: {} and {}", l.get_type(), r.get_type())),
    }
}

fn subtract_native(left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
    let left = &*left.borrow();
    let right = &*right.borrow();

    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => Ok(Object::AntInt(AntInt::from(&l.value - &r.value))),
        (Object::AntDouble(l), Object::AntDouble(r)) => Ok(Object::AntDouble(AntDouble::from(&l.value - &r.value))),
        (Object::AntInt(l), Object::AntDouble(r)) => Ok(Object::AntDouble(AntDouble::from(&l.value - &r.value))),
        (Object::AntDouble(l), Object::AntInt(r)) => Ok(Object::AntDouble(AntDouble::from(&l.value - &r.value))),

        (l, r) => Err(format!("unimplemented for types: {} and {}", l.get_type(), r.get_type())),
    }
}

fn multiply_native(left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
    let left = &*left.borrow();
    let right = &*right.borrow();

    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => Ok(Object::AntInt(AntInt::from(&l.value * &r.value))),
        (Object::AntDouble(l), Object::AntDouble(r)) => Ok(Object::AntDouble(AntDouble::from(&l.value * &r.value))),
        (Object::AntInt(l), Object::AntDouble(r)) => Ok(Object::AntDouble(AntDouble::from(&l.value * &r.value))),
        (Object::AntDouble(l), Object::AntInt(r)) => Ok(Object::AntDouble(AntDouble::from(&l.value * &r.value))),

        (l, r) => Err(format!("unimplemented for types: {} and {}", l.get_type(), r.get_type())),
    }
}

fn divide_native(left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
    let left = &*left.borrow();
    let right = &*right.borrow();

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

        (l, r) => Err(format!("unimplemented for types: {} and {}", l.get_type(), r.get_type())),
    }
}

fn gt_native(left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
    let left = &*left.borrow();
    let right = &*right.borrow();

    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => Ok(native_boolean_to_object(&l.value > &r.value)),
        (Object::AntDouble(l), Object::AntDouble(r)) => Ok(native_boolean_to_object(&l.value > &r.value)),
        (Object::AntInt(l), Object::AntDouble(r)) => Ok(native_boolean_to_object(&l.value > &r.value)),
        (Object::AntDouble(l), Object::AntInt(r)) => Ok(native_boolean_to_object(&l.value > &r.value)),

        (l, r) => Err(format!("unimplemented for types: {} and {}", l.get_type(), r.get_type())),
    }
}

fn eq_native(left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
    let left = &*left.borrow();
    let right = &*right.borrow();

    if left.get_type() == NULL && right.get_type() != NULL {
        return Ok(FALSE_OBJ.clone());
    } else if right.get_type() == NULL && left.get_type() != NULL {
        return Ok(FALSE_OBJ.clone());
    }

    if left.get_type() == NULL && right.get_type() == NULL {
        return Ok(TRUE_OBJ.clone());
    }

    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => Ok(native_boolean_to_object(&l.value == &r.value)),
        (Object::AntDouble(l), Object::AntDouble(r)) => Ok(native_boolean_to_object(&l.value == &r.value)),
        (Object::AntInt(l), Object::AntDouble(r)) => Ok(native_boolean_to_object(&l.value == &r.value)),
        (Object::AntDouble(l), Object::AntInt(r)) => Ok(native_boolean_to_object(&l.value == &r.value)),
        (Object::AntBoolean(l), Object::AntBoolean(r)) => Ok(native_boolean_to_object(l.value == r.value)),

        (l, r) => Err(format!("unimplemented for types: {} and {}", l.get_type(), r.get_type())),
    }
}

fn not_eq_native(left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
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

pub fn eval_infix_operator(op: OpCode, left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
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
