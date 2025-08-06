use bigdecimal::BigDecimal;

use crate::{byte_code_vm::{code::code::{OpCode, OP_ADD, OP_DIVIDE, OP_EQ, OP_GT, OP_MULTIPLY, OP_NOTEQ, OP_SUBTRACT}, utils::native_boolean_to_object}, object::{ant_boolean::AntBoolean, ant_double::AntDouble, ant_int::AntInt, object::Object}};

fn add_native(
    left: Object,
    right: Object,
) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>() && 
       let Some(right) = right_as_anyed.downcast_ref::<AntInt>() 
    {
        return Ok(Box::new(AntInt::from(&left.value + &right.value)));
    }

    return Err(format!("Unimplemented for types: {:?} and {:?}", left_as_anyed.type_id(), right_as_anyed.type_id()))
}

fn subtract_native(
    left: Object,
    right: Object,
) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>() && 
       let Some(right) = right_as_anyed.downcast_ref::<AntInt>() 
    {
        return Ok(Box::new(AntInt::from(&left.value - &right.value)));
    }

    return Err(format!("Unimplemented for types: {:?} and {:?}", left_as_anyed.type_id(), right_as_anyed.type_id()))
}

fn multiply_native(
    left: Object,
    right: Object,
) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>() && 
       let Some(right) = right_as_anyed.downcast_ref::<AntInt>() 
    {
        return Ok(Box::new(AntInt::from(&left.value * &right.value)));
    }

    return Err(format!("Unimplemented for types: {:?} and {:?}", left_as_anyed.type_id(), right_as_anyed.type_id()))
}

fn divide_native(
    left: Object,
    right: Object,
) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>() && 
       let Some(right) = right_as_anyed.downcast_ref::<AntInt>() 
    {
        if right.value == BigDecimal::from(0) {
            return Err("Division by zero".to_string());
        }

        let result = &left.value / &right.value;

        return if result.is_integer() {
            Ok(Box::new(AntInt::from(result)))
        } else {
            Ok(Box::new(AntDouble::from(result)))
        }
    }

    return Err(format!("Unimplemented for types: {:?} and {:?}", left_as_anyed.type_id(), right_as_anyed.type_id()))
}

fn gt_native(
    left: Object,
    right: Object,
) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>() && 
       let Some(right) = right_as_anyed.downcast_ref::<AntInt>() 
    {
        let result = &left.value > &right.value;

        return Ok(native_boolean_to_object(result));
    }

    return Err(format!("Unimplemented for types: {:?} and {:?}", left_as_anyed.type_id(), right_as_anyed.type_id()))
}

fn eq_native(
    left: Object,
    right: Object,
) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>() && 
       let Some(right) = right_as_anyed.downcast_ref::<AntInt>() 
    {
        let result = &left.value == &right.value;

        return Ok(native_boolean_to_object(result));
    }

    if let Some(left) = left_as_anyed.downcast_ref::<AntBoolean>() && 
       let Some(right) = right_as_anyed.downcast_ref::<AntBoolean>() 
    {
        let result = &left.value == &right.value;

        return Ok(native_boolean_to_object(result));
    }

    return Err(format!("Unimplemented for types: {:?} and {:?}", left_as_anyed.type_id(), right_as_anyed.type_id()))
}

fn not_eq_native(
    left: Object,
    right: Object,
) -> Result<Object, String> {
    let result = eq_native(left, right);

    match result {
        Ok(obj) => {
            if let Some(boolean) = obj.as_any().downcast_ref::<AntBoolean>() {
                return Ok(native_boolean_to_object(!boolean.value));
            }
            
            return Err(format!(
                "Expected a boolean object, got: {:?}",
                obj
            ));
        }

        Err(e) => Err(e),
    }
}

pub fn eval_infix_operator(
    op: OpCode,
    left: Object,
    right: Object,
) -> Result<Object, String> {
    if left.get_type() != right.get_type() {
        panic!("Unimplemented")
    }

    match op {
        OP_ADD => add_native(left, right),
        OP_SUBTRACT => subtract_native(left, right),
        OP_MULTIPLY => multiply_native(left, right),
        OP_DIVIDE => divide_native(left, right),
        OP_GT => gt_native(left, right),
        OP_EQ => eq_native(left, right),
        OP_NOTEQ => not_eq_native(left, right),

        _ => {
            Err(format!("Unknown operator: {}", op))
        }
    }
}