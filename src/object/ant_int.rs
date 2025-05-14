use std::any::Any;
use std::vec;
use bigdecimal::BigDecimal;
use uuid::Uuid;
use num_bigint::BigInt;

use crate::object::object::GetEnv;
use crate::constants::{null_obj, uninit_obj};
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::environment::utils::create_env;
use crate::evaluator::utils::native_boolean_to_boolean_obj;
use crate::{impl_minus_func, impl_multiply_func, impl_object, impl_plus_func};
use crate::object::ant_double::AntDouble;
use crate::object::ant_native_function::{create_ant_native_function, NativeFunction};
use crate::object::object::{IAntObject, ObjectType, INT};
use crate::object::utils::{create_error, create_error_with_name, is_truthy, unsupported_operand_type_err};
use crate::extract_arg;

pub struct AntInt {
    id: Uuid,
    env: Environment,
    pub value: BigInt,
}

impl Clone for AntInt {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            value: self.value.clone(),
        }
    }
}

impl IAntObject for AntInt {
    fn get_type(&self) -> ObjectType {
        INT.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value.clone())
    }

    fn get_base(&self) -> Option<Box<dyn IAntObject>> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("{}", self.value.to_string())
    }

    fn new(arg_env: Environment) -> Box<dyn IAntObject> {
        let mut value = BigInt::from(0);

        let mut new = |obj: Box<dyn IAntObject>| {
            let cast_obj =  obj.as_any().downcast_ref::<AntInt>().cloned();
            if let Some(obj) = cast_obj {
                value = obj.value; return null_obj.clone()
            }

            create_error(format!("value is not {}", INT))
        };

        let mut env = create_env(
            vec![
                ("value".to_string(), null_obj.clone())
            ]
        );

        env.in_place_fusion(arg_env);

        new(env.get("value").unwrap());

        let mut obj = Self {
            id: Uuid::new_v4(),
            env: env.clone(),
            value: BigInt::from(value),
        };

        init_env(&mut obj);

        Box::new(obj)
    }

    fn new_with_native_value(mut value: Box<dyn Any>) -> Box<dyn IAntObject> {
        let cast_result = value.downcast_mut::<BigInt>().cloned();

        if let Some(it) = cast_result {
            let env = create_env(
                vec![
                    ("value".to_string(), null_obj.clone())
                ]
            );

            let mut obj = Self {
                id: Uuid::new_v4(),
                env,
                value: it.clone(),
            };

            init_env(&mut obj);

            return Box::new(obj);
        }

        panic!("value is not BigInt")
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == INT {
            other.as_any().downcast_ref::<AntInt>().unwrap().value == self.value
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntInt);

fn init_env(int_obj: &mut AntInt) {
    fn plus(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        impl_plus_func!(plus_int, AntInt, AntInt, AntInt);
        impl_plus_func!(plus_double, AntInt, AntDouble, AntDouble);

        let me = extract_arg!(arg_env, "me" => AntInt);

        if me.is_none() {
            return Some(create_error_with_name("TypeError", format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return plus_int(me, value)}
            if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return plus_double(me, value)}
            
            let right_type = arg_env
                .get("value")
                .expect(&format!("cannot find 'value'. arg_env: {}", arg_env.to_string()))
                .get_type();

            return Some(unsupported_operand_type_err("+", me.get_type(), right_type))
        }

        None
    }

    fn minus(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        impl_minus_func!(minus_int, AntInt, AntInt, AntInt);
        impl_minus_func!(minus_double, AntInt, AntDouble, AntDouble);

        let me = extract_arg!(arg_env, "me" => AntInt);

        if me.is_none() {
            return Some(create_error_with_name("TypeError", format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return minus_int(me, value)}
            if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return minus_double(me, value)}
            
            let right_type = arg_env
                .get("value")
                .expect(&format!("cannot find 'value'. arg_env: {}", arg_env.to_string()))
                .get_type();

            return Some(unsupported_operand_type_err("-", me.get_type(), right_type))
        }

        None
    }

    fn multiply(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        impl_multiply_func!(multiply_int, AntInt, AntInt, AntInt);
        impl_multiply_func!(multiply_double, AntInt, AntDouble, AntDouble);

        let me = extract_arg!(arg_env, "me" => AntInt);

        if me.is_none() {
            return Some(create_error_with_name("TypeError", format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return multiply_int(me, value)}
            if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return multiply_double(me, value)}
            
            let right_type = arg_env
                .get("value")
                .expect(&format!("cannot find 'value'. arg_env: {}", arg_env.to_string()))
                .get_type();

            return Some(unsupported_operand_type_err("*", me.get_type(), right_type))
        }

        None
    }

    fn divide(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        fn divide_int(me: AntInt, other: AntInt) -> Option<Box<dyn IAntObject>> {
            if other.value == BigInt::from(0) {
                return Some(
                    create_error("division by zero".to_string())
                )
            }

            Some(
                AntDouble::new_with_native_value(Box::new(BigDecimal::from(me.value) / BigDecimal::from(other.value)))
            )
        }

        let me = extract_arg!(arg_env, "me" => AntInt);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for \"me\"")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return divide_int(me, value)}

            let right_type = arg_env
                .get("value")
                .expect(&format!("cannot find 'value'. arg_env: {}", arg_env.to_string()))
                .get_type();

            return Some(unsupported_operand_type_err("/", me.get_type(), right_type))
        }

        None
    }

    fn greater_than(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        fn greater_than_int(me: AntInt, other: AntInt) -> Option<Box<dyn IAntObject>> {
            Some(
                native_boolean_to_boolean_obj(me.value > other.value)
            )
        }

        let me = extract_arg!(arg_env, "me" => AntInt);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for \"me\"")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return greater_than_int(me, value)}
        
            let right_type = arg_env
                .get("value")
                .expect(&format!("cannot find 'value'. arg_env: {}", arg_env.to_string()))
                .get_type();

            return Some(unsupported_operand_type_err(">", me.get_type(), right_type))
        }

        None
    }

    fn less_than(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        fn less_than_int(me: AntInt, other: AntInt) -> Option<Box<dyn IAntObject>> {
            Some(
                native_boolean_to_boolean_obj(me.value < other.value)
            )
        }

        let me = extract_arg!(arg_env, "me" => AntInt);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for \"me\"")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return less_than_int(me, value)}
        
            let right_type = arg_env
                .get("value")
                .expect(&format!("cannot find 'value'. arg_env: {}", arg_env.to_string()))
                .get_type();

            return Some(unsupported_operand_type_err("<", me.get_type(), right_type))
        }

        None
    }

    fn eq(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        fn eq_int(me: AntInt, other: AntInt) -> Option<Box<dyn IAntObject>> {
           Some(native_boolean_to_boolean_obj( me == other))
        }

        let me = extract_arg!(arg_env, "me" => AntInt);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for \"me\"")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return eq_int(me, value)}
        
            let right_type = arg_env
                .get("value")
                .expect(&format!("cannot find 'value'. arg_env: {}", arg_env.to_string()))
                .get_type();

            return Some(unsupported_operand_type_err("==", me.get_type(), right_type))
        }

        None
    }

    fn not_eq(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        let me = extract_arg!(arg_env, "me" => AntInt);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for \"me\"")))
        } else if let Some(me) = me {
            if let Some(_value) = extract_arg!(arg_env, "value" => AntInt) {
                return Some(native_boolean_to_boolean_obj(!is_truthy(eq(arg_env).expect(""))))
            }

        
            let right_type = arg_env
                .get("value")
                .expect(&format!("cannot find 'value'. arg_env: {}", arg_env.to_string()))
                .get_type();

            return Some(unsupported_operand_type_err("!=", me.get_type(), right_type))
        }

        None
    }

    let func_param_env = create_env(
        vec![
            ("me".to_string(), Box::new(int_obj.clone())),
            ("value".to_string(), uninit_obj.clone())
        ]
    );

    let operator_functions = vec![
        ("plus", plus as NativeFunction),
        ("minus", minus),
        ("multiply", multiply),
        ("divide", divide),
        ("lt", less_than),
        ("gt", greater_than),
        ("eq", eq),
        ("not_eq", not_eq),
    ];

    for (op, func) in operator_functions {
        let native_func_object = create_ant_native_function(func_param_env.clone(), func); 

        int_obj.env.create(op, Data::new(native_func_object, DataInfo::new(false)));
    }
}