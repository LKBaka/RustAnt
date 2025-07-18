use std::any::Any;
use std::vec;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::object::object::{EnvGetter, DOUBLE};
use crate::constants::{ant_false, ant_true, null_obj, uninit_obj};
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::environment::utils::create_env;
use crate::evaluator::utils::native_boolean_to_boolean_obj;
use crate::type_defs::RcRefCellEnv;
use crate::{impl_gt_func, impl_lt_func, impl_minus_func, impl_multiply_func, impl_object, impl_plus_func};
use crate::object::ant_double::AntDouble;
use crate::object::ant_native_function::{create_ant_native_function, NativeFunction};
use crate::object::object::{IAntObject, Object, ObjectType, INT};
use crate::object::utils::{create_error, create_error_with_name, is_truthy};
use crate::extract_arg;
use crate::{type_hint, type_hint_map};

use super::type_hint::{TypeHint, TypeHintMap};

pub struct AntInt {
    id: Uuid,
    env: Environment,
    pub value: BigDecimal,
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

impl AntInt {
    pub fn new_with_native_value(mut value: Box<dyn Any>) -> Object {
        let cast_result = value.downcast_mut::<BigDecimal>().cloned();

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

        panic!("value is not BigDecimal. value's type id: {:?}", value.type_id());
    }
}

impl IAntObject for AntInt {
    fn get_type(&self) -> ObjectType {
        INT.to_string()
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
        other.get_id() == self.id || if other.get_type() == INT {
            other.as_any().downcast_ref::<AntInt>().unwrap().value == self.value
        } else if let Some(double_obj) = other.as_any().downcast_ref::<AntDouble>() {
            &double_obj.value == &self.value
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntInt);

fn init_env(int_obj: &mut AntInt) {
    // 初始化运算符
    {
        fn plus(arg_env: &mut Environment) -> Option<Object> {
            impl_plus_func!(plus_int, AntInt, AntInt, AntInt);
            impl_plus_func!(plus_double, AntInt, AntDouble, AntDouble);

            let me = extract_arg!(arg_env, "me" => AntInt);

            if me.is_none() {
                return Some(create_error_with_name("TypeError", format!("type mismatch for 'me'")))
            } else if let Some(me) = me {
                if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return plus_int(me, value)}
                if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return plus_double(me, value)}
            }

            None
        }

        fn minus(arg_env: &mut Environment) -> Option<Object> {
            impl_minus_func!(minus_int, AntInt, AntInt, AntInt);
            impl_minus_func!(minus_double, AntInt, AntDouble, AntDouble);

            let me = extract_arg!(arg_env, "me" => AntInt);

            if me.is_none() {
                return Some(create_error_with_name("TypeError", format!("type mismatch for 'me'")))
            } else if let Some(me) = me {
                if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return minus_int(me, value)}
                if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return minus_double(me, value)}
            }

            None
        }

        fn multiply(arg_env: &mut Environment) -> Option<Object> {
            impl_multiply_func!(multiply_int, AntInt, AntInt, AntInt);
            impl_multiply_func!(multiply_double, AntInt, AntDouble, AntDouble);

            let me = extract_arg!(arg_env, "me" => AntInt);

            if me.is_none() {
                return Some(create_error_with_name("TypeError", format!("type mismatch for 'me'")))
            } else if let Some(me) = me {
                if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return multiply_int(me, value)}
                if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return multiply_double(me, value)}
            }

            None
        }

        fn divide(arg_env: &mut Environment) -> Option<Object> {
            fn divide_int(me: AntInt, other: AntInt) -> Option<Object> {
                if other.value == BigDecimal::from(0) {
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
                return Some(create_error(format!("type mismatch for 'me'")))
            } else if let Some(me) = me {
                if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return divide_int(me, value)}
            }

            None
        }

        fn greater_than(arg_env: &mut Environment) -> Option<Object> {
            impl_gt_func!(gt_int, AntInt, AntInt);
            impl_gt_func!(gt_double, AntInt, AntDouble);

            let me = extract_arg!(arg_env, "me" => AntInt);

            if me.is_none() {
                return Some(create_error(format!("type mismatch for 'me'")))
            } else if let Some(me) = me {
                if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return gt_int(me, value)}
                if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return gt_double(me, value)}
            }

            None
        }

        fn less_than(arg_env: &mut Environment) -> Option<Object> {
            impl_lt_func!(lt_int, AntInt, AntInt);
            impl_lt_func!(lt_double, AntInt, AntDouble);

            let me = extract_arg!(arg_env, "me" => AntInt);

            if me.is_none() {
                return Some(create_error(format!("type mismatch for 'me'")))
            } else if let Some(me) = me {
                if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return lt_int(me, value)}
                if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return lt_double(me, value)}
            }

            None
        }

        fn eq(arg_env: &mut Environment) -> Option<Object> {
            fn eq_int(me: AntInt, other: AntInt) -> Option<Object> {
                Some(native_boolean_to_boolean_obj( me == other))
            }

            fn eq_double(me: AntInt, other: AntDouble) -> Option<Object> {
                Some(native_boolean_to_boolean_obj( me.value == other.value))
            }

            let me = extract_arg!(arg_env, "me" => AntInt);

            if me.is_none() {
                return Some(create_error(format!("type mismatch for 'me'")))
            } else if let Some(me) = me {
                if let Some(value) = extract_arg!(arg_env, "value" => AntInt) {return eq_int(me, value)}
                if let Some(value) = extract_arg!(arg_env, "value" => AntDouble) {return eq_double(me, value)}
            }

            None
        }

        fn not_eq(arg_env: &mut Environment) -> Option<Object> {
            let me = extract_arg!(arg_env, "me" => AntInt);

            if me.is_none() {
                return Some(create_error(format!("type mismatch for 'me'")))
            } else if let Some(_me) = me {
                if let Some(_value) = extract_arg!(arg_env, "value" => AntInt) {
                    return Some(native_boolean_to_boolean_obj(!is_truthy(eq(arg_env).expect(""))))
                }

                if let Some(_value) = extract_arg!(arg_env, "value" => AntDouble) {
                    return Some(native_boolean_to_boolean_obj(!is_truthy(eq(arg_env).expect(""))))
                }
            }

            None
        }

        let func_param_env = create_env(
            vec![
                ("me".to_string(), Box::new(int_obj.clone())),
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
            ("eq", eq),
            ("not_eq", not_eq),
        ];

        for (op, func) in operator_functions {
            let native_func_object = create_ant_native_function(
                func_param_env.clone(), Some(type_hint_map.clone()), func
            ); 

            int_obj.env.create(op, Data::new(native_func_object, DataInfo::new(false)));
        }
    }

    // 初始化类函数
    {
        let __bool__ = {
            fn __bool__(arg_env: &mut Environment) -> Option<Object> {
                let me = extract_arg!(arg_env, "me" => AntInt);

                if let Some(me) = me {
                    if me.value != BigDecimal::from(0) {
                        return Some(ant_true.clone());
                    } else {
                        return Some(ant_false.clone());
                    }
                }

                return Some(create_error(format!("type mismatch for 'me'")))
            }

            create_ant_native_function(
                create_env(vec![("me".into(), Box::new(int_obj.clone()))]),
                Some(type_hint_map!()),
                __bool__,
            )
        };

        int_obj.env.create("__bool__", Data::new(__bool__, DataInfo::new(false)));
    }
}

impl From<i32> for AntInt {
    fn from(value: i32) -> Self {
        AntInt { 
            id: Uuid::new_v4(), 
            env: Environment::new(), 
            value: BigDecimal::from(value) 
        }
    }
}

pub fn create_ant_int(value: BigDecimal, outer: RcRefCellEnv) -> Object {
    let mut env = create_env(
    vec![
            ("value".to_string(), null_obj.clone())
        ]
    );

    env.outer = Some(outer);

    let mut obj = AntInt {
        id: Uuid::new_v4(),
        env,
        value,
    };

    init_env(&mut obj);

    Box::new(obj)
}