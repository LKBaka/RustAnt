use std::any::Any;
use std::vec;
use uuid::Uuid;

use crate::constants::{ant_false, ant_true, uninit_obj};
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::environment::utils::create_env;
use crate::evaluator::utils::native_boolean_to_boolean_obj;
use crate::object::ant_native_function::{create_ant_native_function, NativeFunction};
use crate::object::utils::{create_error, is_truthy};
use crate::{extract_arg, impl_object};
use crate::object::object::{IAntObject, Object, ObjectType, NULL};
use crate::object::object::EnvGetter;

pub struct AntNull {
    id: Uuid,
    env: Environment,
}

impl AntNull {
    pub fn new(_arg_env: Environment) -> Object {
        let mut obj = Box::new(Self {
            id: Uuid::new_v4(),
            env: Environment::new(),
        });

        init_env(&mut obj);

        obj
    }
}

impl IAntObject for AntNull {
    fn get_type(&self) -> ObjectType {
        NULL.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(())
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }


    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == NULL {true} else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for AntNull {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
        }
    }
}

fn init_env(null_obj: &mut AntNull) {
    // 初始化运算符
    {
        fn eq(arg_env: &mut Environment) -> Option<Object> {
            fn eq_null(_me: AntNull, _other: AntNull) -> Option<Object> {
                Some(ant_true.clone())
            }

            let me = extract_arg!(arg_env, "me" => AntNull);

            if me.is_none() {
                return Some(create_error(format!("type mismatch for \"me\"")))
            } else if let Some(me) = me {
                if let Some(value) = extract_arg!(arg_env, "value" => AntNull) {return eq_null(me, value)}
            }

            None
        }

        fn not_eq(arg_env: &mut Environment) -> Option<Object> {
            let me = extract_arg!(arg_env, "me" => AntNull);

            if me.is_none() {
                return Some(create_error(format!("type mismatch for \"me\"")))
            } else if let Some(_me) = me {
                if let Some(_value) = extract_arg!(arg_env, "value" => AntNull) {
                    return Some(native_boolean_to_boolean_obj(!is_truthy(eq(arg_env).expect(""))))
                }
            }

            None
        }
       
        let func_param_env = create_env(
            vec![
                ("me".to_string(), Box::new(null_obj.clone())),
                ("value".to_string(), uninit_obj.clone())
            ]
        );

        let operator_functions = vec![
            ("eq", eq as NativeFunction),
            ("not_eq", not_eq),
        ];

        for (op, func) in operator_functions {
            let native_func_object = create_ant_native_function(func_param_env.clone(), func); 

            null_obj.env.create(op, Data::new(native_func_object, DataInfo::new(false)));
        }
    }

    // 初始化类函数
    {
        let __bool__ = {
            fn __bool__(_arg_env: &mut Environment) -> Option<Object> {
                Some(ant_false.clone())
            }

            create_ant_native_function(
                create_env(vec![("me".into(), Box::new(null_obj.clone()))]),
                __bool__
            )
        };

        null_obj.env.create("__bool__", Data::new(__bool__, DataInfo::new(false)));
    }

}

impl_object!(AntNull);