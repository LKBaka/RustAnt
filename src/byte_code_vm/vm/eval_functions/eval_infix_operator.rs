use bigdecimal::BigDecimal;

use crate::{
    byte_code_vm::{
        code::code::{OP_ADD, OP_DIVIDE, OP_EQ, OP_GT, OP_MULTIPLY, OP_NOTEQ, OP_SUBTRACT, OpCode},
        utils::native_boolean_to_object,
    },
    object::{
        ant_boolean::AntBoolean, ant_double::AntDouble, ant_int::AntInt, ant_string::AntString,
        object::Object,
    },
};

fn add_native(left: Object, right: Object) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    // AntInt + AntInt
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        return Ok(Box::new(AntInt::from(&left.value + &right.value)));
    }

    // AntDouble + AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        return Ok(Box::new(AntDouble::from(&left.value + &right.value)));
    }

    // AntInt + AntDouble -> 转换为 AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        return Ok(Box::new(AntDouble::from(&left.value + &right.value)));
    }

    // AntDouble + AntInt -> 转换为 AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        return Ok(Box::new(AntDouble::from(&left.value + &right.value)));
    }

    // AntString + AntString
    if let Some(left) = left_as_anyed.downcast_ref::<AntString>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntString>()
    {
        return Ok(Box::new(AntString::new(left.value.clone() + &right.value)));
    }

    return Err(format!(
        "unimplemented for types: {} and {}",
        left.get_type(),
        right.get_type()
    ));
}

fn subtract_native(left: Object, right: Object) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    // AntInt - AntInt
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        return Ok(Box::new(AntInt::from(&left.value - &right.value)));
    }

    // AntDouble - AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        return Ok(Box::new(AntDouble::from(&left.value - &right.value)));
    }

    // AntInt - AntDouble -> 转换为 AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        return Ok(Box::new(AntDouble::from(&left.value - &right.value)));
    }

    // AntDouble - AntInt -> 转换为 AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        return Ok(Box::new(AntDouble::from(&left.value - &right.value)));
    }

    return Err(format!(
        "unimplemented for types: {} and {}",
        left.get_type(),
        right.get_type()
    ));
}

fn multiply_native(left: Object, right: Object) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    // AntInt * AntInt
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        return Ok(Box::new(AntInt::from(&left.value * &right.value)));
    }

    // AntDouble * AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        return Ok(Box::new(AntDouble::from(&left.value * &right.value)));
    }

    // AntInt * AntDouble -> 转换为 AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        return Ok(Box::new(AntDouble::from(&left.value * &right.value)));
    }

    // AntDouble * AntInt -> 转换为 AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        return Ok(Box::new(AntDouble::from(&left.value * &right.value)));
    }

    return Err(format!(
        "unimplemented for types: {} and {}",
        left.get_type(),
        right.get_type()
    ));
}

fn divide_native(left: Object, right: Object) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    // AntInt / AntInt
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        if right.value == BigDecimal::from(0) {
            return Err("division by zero".to_string());
        }

        let result = &left.value / &right.value;

        return if result.is_integer() {
            Ok(Box::new(AntInt::from(result)))
        } else {
            Ok(Box::new(AntDouble::from(result)))
        };
    }

    // AntDouble / AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        if right.value == BigDecimal::from(0) {
            return Err("division by zero".to_string());
        }

        let result = &left.value / &right.value;

        return if result.is_integer() {
            Ok(Box::new(AntInt::from(result)))
        } else {
            Ok(Box::new(AntDouble::from(result)))
        };
    }

    // AntInt / AntDouble -> 转换为 AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        if right.value == BigDecimal::from(0) {
            return Err("division by zero".to_string());
        }

        let result = &left.value / &right.value;
        return Ok(Box::new(AntDouble::from(result)));
    }

    // AntDouble / AntInt -> 转换为 AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        if right.value == BigDecimal::from(0) {
            return Err("division by zero".to_string());
        }

        let result = &left.value / &right.value;
        return Ok(Box::new(AntDouble::from(result)));
    }

    return Err(format!(
        "unimplemented for types: {} and {}",
        left.get_type(),
        right.get_type()
    ));
}

fn gt_native(left: Object, right: Object) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    // AntInt > AntInt
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        let result = &left.value > &right.value;
        return Ok(native_boolean_to_object(result));
    }

    // AntDouble > AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        let result = &left.value > &right.value;
        return Ok(native_boolean_to_object(result));
    }

    // AntInt > AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        let result = &left.value > &right.value;
        return Ok(native_boolean_to_object(result));
    }

    // AntDouble > AntInt
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        let result = &left.value > &right.value;
        return Ok(native_boolean_to_object(result));
    }

    return Err(format!(
        "unimplemented for types: {} and {}",
        left.get_type(),
        right.get_type()
    ));
}

fn eq_native(left: Object, right: Object) -> Result<Object, String> {
    let left_as_anyed = left.as_any();
    let right_as_anyed = right.as_any();

    // AntInt == AntInt
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        let result = &left.value == &right.value;
        return Ok(native_boolean_to_object(result));
    }

    // AntDouble == AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        let result = &left.value == &right.value;
        return Ok(native_boolean_to_object(result));
    }

    // AntInt == AntDouble
    if let Some(left) = left_as_anyed.downcast_ref::<AntInt>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntDouble>()
    {
        let result = &left.value == &right.value;
        return Ok(native_boolean_to_object(result));
    }

    // AntDouble == AntInt
    if let Some(left) = left_as_anyed.downcast_ref::<AntDouble>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntInt>()
    {
        let result = &left.value == &right.value;
        return Ok(native_boolean_to_object(result));
    }

    // AntBoolean == AntBoolean
    if let Some(left) = left_as_anyed.downcast_ref::<AntBoolean>()
        && let Some(right) = right_as_anyed.downcast_ref::<AntBoolean>()
    {
        let result = &left.value == &right.value;
        return Ok(native_boolean_to_object(result));
    }

    return Err(format!(
        "unimplemented for types: {} and {}",
        left.get_type(),
        right.get_type()
    ));
}

fn not_eq_native(left: Object, right: Object) -> Result<Object, String> {
    let result = eq_native(left, right);

    match result {
        Ok(obj) => {
            if let Some(boolean) = obj.as_any().downcast_ref::<AntBoolean>() {
                return Ok(native_boolean_to_object(!boolean.value));
            }

            return Err(format!("expected a boolean object, got: {:?}", obj));
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
