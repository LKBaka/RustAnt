use std::any::Any;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::object::ant_int::AntInt;
use crate::object::object::{EnvGetter, INT};
use crate::constants::{null_obj, uninit_obj};
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::environment::utils::create_env;
use crate::{impl_gt_func, impl_lt_func, impl_object, type_hint, type_hint_map};
use crate::object::ant_native_function::create_ant_native_function;
use crate::object::object::{IAntObject, Object, ObjectType, DOUBLE};
use crate::extract_arg;
use crate::{impl_minus_func, impl_multiply_func, impl_plus_func};
use crate::object::utils::create_error;
use crate::object::ant_native_function::NativeFunction;
use crate::evaluator::utils::native_boolean_to_boolean_obj;

use super::type_hint::{TypeHint, TypeHintMap};


pub struct AntDouble {
    id: Uuid,
    env: Environment,
    pub value: BigDecimal,
}

impl Clone for AntDouble {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            value: self.value.clone(),
        }
    }
}

fn init_env(double_obj: &mut AntDouble) {
    fn plus(arg_env: &mut Environment) -> Option<Object> {
        impl_plus_func!(plus_double, AntDouble, AntDouble, AntDouble);
        impl_plus_func!(plus_int, AntDouble, AntInt, AntDouble);

        let me = extract_arg!(arg_env, "me" => AntDouble);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return plus_int(me, value)}
            if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return plus_double(me, value)}
        }

        None
    }

    fn minus(arg_env: &mut Environment) -> Option<Object> {
        impl_minus_func!(minus_double, AntDouble, AntDouble, AntDouble);
        impl_minus_func!(minus_int, AntDouble, AntInt, AntDouble);

        let me = extract_arg!(arg_env, "me" => AntDouble);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return minus_int(me, value)}
            if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return minus_double(me, value)}
        }

        None
    }

    fn multiply(arg_env: &mut Environment) -> Option<Object> {
        impl_multiply_func!(multiply_double, AntDouble, AntDouble, AntDouble);
        impl_multiply_func!(multiply_int, AntDouble, AntInt, AntDouble);

        let me = extract_arg!(arg_env, "me" => AntDouble);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return multiply_int(me, value)}
            if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return multiply_double(me, value)}
        }

        None
    }

    fn divide(arg_env: &mut Environment) -> Option<Object> {
        fn divide_double(me: AntDouble, other: AntDouble) -> Option<Object> {
            if other.value == BigDecimal::from(0) {
                return Some(create_error(format!("division by zero")))
            }

            Some(
                AntDouble::new_with_native_value(Box::new(me.value / other.value))
            )
        }

        let me = extract_arg!(arg_env, "me" => AntDouble);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return divide_double(me, value)}
        }

        None
    }

    fn greater_than(arg_env: &mut Environment) -> Option<Object> {
        impl_gt_func!(gt_double, AntDouble, AntDouble);
        impl_gt_func!(gt_int, AntDouble, AntInt);

        let me = extract_arg!(arg_env, "me" => AntDouble);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return gt_double(me, value)}
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return gt_int(me, value)}
        }

        None
    }

    fn less_than(arg_env: &mut Environment) -> Option<Object> {
        impl_lt_func!(lt_double, AntDouble, AntDouble);
        impl_lt_func!(lt_int, AntDouble, AntInt);

        let me = extract_arg!(arg_env, "me" => AntDouble);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return lt_double(me, value)}
            if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return lt_int(me, value)}
        }

        None
    }

    let func_param_env = create_env(
        vec![
            ("me".to_string(), Box::new(double_obj.clone())),
            ("value".to_string(), uninit_obj.clone())
        ]
    );

    let type_hint_map = type_hint_map!("value" => type_hint!(INT, DOUBLE));

    let operator_functions = vec![
        ("plus", plus as NativeFunction),
        ("minus", minus),
        ("multiply", multiply),
        ("divide", divide),
        ("lt", less_than),
        ("gt", greater_than),
    ];

    for (op, func) in operator_functions {
        let native_func_object = create_ant_native_function(
            func_param_env.clone(), Some(type_hint_map.clone()), func 
        ); 

        double_obj.env.create(op, Data::new(native_func_object, DataInfo::new(false)));
    }
}

impl AntDouble {
    pub fn new_with_native_value(mut value: Box<dyn Any>) -> Object {
        let cast_result = value.downcast_mut::<BigDecimal>().cloned();

        match cast_result {
            None => {
                panic!("value is not BigDecimal")
            }
            Some(big_decimal) => {
                let mut env = Environment::new();
                env.create("value", Data::new(null_obj.clone(), DataInfo::new(false)));

                let mut obj = Self {
                    id: Uuid::new_v4(),
                    env,
                    value: big_decimal.clone(),
                };

                init_env(&mut obj);

                Box::new(obj)
            }
        }
    }
}

impl IAntObject for AntDouble {
    fn get_type(&self) -> ObjectType {
        DOUBLE.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value.clone())
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("{}", self.value.to_string())
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == DOUBLE {
            other.as_any().downcast_ref::<AntDouble>().unwrap().value == self.value
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntDouble);
