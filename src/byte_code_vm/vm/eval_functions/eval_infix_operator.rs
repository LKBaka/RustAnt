use std::{cell::RefCell, rc::Rc};

use bigdecimal::BigDecimal;

use crate::{
    byte_code_vm::{
        code::code::{OpCode, OP_ADD, OP_DIVIDE, OP_EQ, OP_GT, OP_MULTIPLY, OP_NOTEQ, OP_SUBTRACT},
        constants::{FALSE_OBJ, TRUE_OBJ},
        utils::native_boolean_to_object,
    },
    obj_enum::object::Object,
    object::{
        ant_array::AntArray, ant_double::AntDouble, ant_i64::AntI64, ant_int::AntInt, ant_string::AntString, object::{IAntObject, NULL}
    },
};

// 保留原本注释与语义：对多种数值/字符串类型做互操作
fn add_native(left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
    let left = &*left.borrow();
    let right = &*right.borrow();

    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => {
            Ok(Object::AntInt(AntInt::from(&l.value + &r.value)))
        }
        (Object::AntI64(l), Object::AntI64(r)) => {
            Ok(Object::AntI64(AntI64::from(&l.value + &r.value)))
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
        (Object::AntString(l), Object::AntString(r)) => Ok(Object::AntString(AntString::new(
            l.value.clone() + &r.value,
        ))),
        (Object::AntArray(l), Object::AntArray(r)) => Ok(Object::AntArray(AntArray::from({
            let mut v = l.items.clone();
            v.append(&mut r.items.clone());

            v
        }))),

        (l, r) => Err(format!(
            "unimplemented for types: {} and {}",
            l.get_type(),
            r.get_type()
        )),
    }
}

fn subtract_native(
    left: Rc<RefCell<Object>>,
    right: Rc<RefCell<Object>>,
) -> Result<Object, String> {
    let left = &*left.borrow();
    let right = &*right.borrow();

    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => {
            Ok(Object::AntInt(AntInt::from(&l.value - &r.value)))
        }
        (Object::AntI64(l), Object::AntI64(r)) => {
            Ok(Object::AntI64(AntI64::from(&l.value - &r.value)))
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

fn multiply_native(
    left: Rc<RefCell<Object>>,
    right: Rc<RefCell<Object>>,
) -> Result<Object, String> {
    let left = &*left.borrow();
    let right = &*right.borrow();

    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => {
            Ok(Object::AntInt(AntInt::from(&l.value * &r.value)))
        }
        (Object::AntI64(l), Object::AntI64(r)) => {
            Ok(Object::AntI64(AntI64::from(&l.value * &r.value)))
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
        (Object::AntString(l), Object::AntInt(r)) => {
            if l.value == "" {
                return Ok(left.clone())
            }

            use num_traits::ToPrimitive;

            let repeat_count = r.value.to_usize().ok_or_else(|| format!(
                "expected repeat count"
            ))?;

            Ok(Object::AntString(AntString::new(l.value.repeat(repeat_count))))
        }
        (Object::AntInt(l), Object::AntString(r)) => {
            if r.value == "" {
                return Ok(left.clone())
            }

            use num_traits::ToPrimitive;

            let repeat_count = l.value.to_usize().ok_or_else(|| format!(
                "expected repeat count"
            ))?;

            Ok(Object::AntString(AntString::new(r.value.repeat(repeat_count))))
        }
        (Object::AntArray(l), Object::AntInt(r)) => {
            if l.items.is_empty() {
                return Ok(left.clone())
            }

            use num_traits::ToPrimitive;

            let repeat_count = r.value.to_usize().ok_or_else(|| format!(
                "expected repeat count"
            ))?;

            Ok(Object::AntArray(AntArray::from(
                l.items
                    .iter()
                    .cloned()
                    .cycle()
                    .take(repeat_count)
                    .collect::<Vec<_>>()
            )))
        }
        (Object::AntInt(l), Object::AntArray(r)) => {
            if r.items.is_empty() {
                return Ok(left.clone())
            }

            use num_traits::ToPrimitive;

            let repeat_count = l.value.to_usize().ok_or_else(|| format!(
                "expected repeat count"
            ))?;

            Ok(Object::AntArray(AntArray::from(
                r.items
                    .iter()
                    .cloned()
                    .cycle()
                    .take(repeat_count)
                    .collect::<Vec<_>>()
            )))
        }

        (l, r) => Err(format!(
            "unimplemented for types: {} and {}",
            l.get_type(),
            r.get_type()
        )),
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

        (Object::AntI64(l), Object::AntI64(r)) => {
            if r.value == 0 {
                return Err("division by zero".to_string());
            }

            let result = BigDecimal::from(&l.value) / BigDecimal::from(&r.value);
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

fn gt_native(left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
    let left = &*left.borrow();
    let right = &*right.borrow();

    Ok(native_boolean_to_object(gt_native_ref(left, right)?))
}

#[inline(always)]
pub fn gt_native_ref(left: &Object, right: &Object) -> Result<bool, String> {
    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => Ok(&l.value > &r.value),
        (Object::AntI64(l), Object::AntI64(r)) => Ok(&l.value > &r.value),
        (Object::AntDouble(l), Object::AntDouble(r)) => {
            Ok(&l.value > &r.value)
        }
        (Object::AntInt(l), Object::AntDouble(r)) => {
            Ok(&l.value > &r.value)
        }
        (Object::AntDouble(l), Object::AntInt(r)) => {
            Ok(&l.value > &r.value)
        }

        (l, r) => Err(format!(
            "unimplemented for types: {} and {}",
            l.get_type(),
            r.get_type()
        )),
    }
}

#[inline(always)]
pub fn eq_native_ref(left: &Object, right: &Object) -> Result<bool, String> {
    if left.get_type() == NULL && right.get_type() != NULL {
        return Ok(false);
    } else if right.get_type() == NULL && left.get_type() != NULL {
        return Ok(false);
    }

    if left.get_type() == NULL && right.get_type() == NULL {
        return Ok(false);
    }

    match (left, right) {
        (Object::AntInt(l), Object::AntInt(r)) => {
            Ok(&l.value == &r.value)
        }
        (Object::AntI64(l), Object::AntI64(r)) => {
            Ok(&l.value == &r.value)
        }
        (Object::AntDouble(l), Object::AntDouble(r)) => {
            Ok(&l.value == &r.value)
        }
        (Object::AntInt(l), Object::AntDouble(r)) => {
            Ok(&l.value == &r.value)
        }
        (Object::AntDouble(l), Object::AntInt(r)) => {
            Ok(&l.value == &r.value)
        }
        (Object::AntBoolean(l), Object::AntBoolean(r)) => {
            Ok(l.value == r.value)
        }
        (Object::AntString(l), Object::AntString(r)) => {
            Ok(l.value == r.value)
        }

        (l, r) => Err(format!(
            "unimplemented for types: {} and {}",
            l.get_type(),
            r.get_type()
        )),
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

    Ok(native_boolean_to_object(eq_native_ref(left, right)?))
}

fn not_eq_native(left: Rc<RefCell<Object>>, right: Rc<RefCell<Object>>) -> Result<Object, String> {
    match eq_native_ref(&*left.borrow(), &*right.borrow()) {
        Ok(eq) => Ok(native_boolean_to_object(eq)),
        Err(e) => Err(e),
    }
}

pub fn eval_infix_operator(
    op: OpCode,
    left: Rc<RefCell<Object>>,
    right: Rc<RefCell<Object>>,
) -> Result<Object, String> {
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
