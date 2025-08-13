use std::any::Any;
use std::vec;
use uuid::Uuid;

use crate::byte_code_vm::utils::native_boolean_to_object;
use crate::constants::uninit_obj;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::environment::utils::create_env;
use crate::object::ant_native_function::{create_ant_native_function, NativeFunction};
use crate::object::object::{IAntObject, Object, ObjectType, ENVIRONMENT};
use crate::object::utils::{create_error, is_truthy, unsupported_operand_type_err};
use crate::{extract_arg, type_hint_map, type_hint};
use crate::impl_object;

use super::type_hint::{TypeHint, TypeHintMap};

pub struct AntEnv {
    id: Uuid,
    pub env: Environment,
    pub obj_env: Environment,
}

impl Clone for AntEnv {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            obj_env: self.obj_env.clone(),
        }
    }
}

impl AntEnv {
    pub fn new_with_native_value(mut value: Box<dyn Any>) -> Object {
        let cast_result = value.downcast_mut::<Environment>().cloned();

        if let Some(it) = cast_result {
            let mut obj = Self {
                id: Uuid::new_v4(),
                env: Environment::new(),
                obj_env: it.clone(),
            };

            init_env(&mut obj);

            return Box::new(obj);
        }

        panic!("value is not Environment")
    }
}

impl IAntObject for AntEnv {
    fn get_type(&self) -> ObjectType {
        ENVIRONMENT.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.env.clone())
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("{}", self.obj_env.to_string())
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == ENVIRONMENT {
            other.as_any().downcast_ref::<AntEnv>().unwrap().env == self.env
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntEnv);

fn init_env(int_obj: &mut AntEnv) {
    fn eq(arg_env: &mut Environment) -> Option<Object> {
        fn eq_int(me: AntEnv, other: AntEnv) -> Option<Object> {
           Some(native_boolean_to_object( me == other))
        }

        let me = extract_arg!(arg_env, "me" => AntEnv);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(value) = extract_arg!(arg_env, "value" => AntEnv) {return eq_int(me, value)}
        
            let right_type = arg_env
                .get("value")
                .expect(&format!("cannot find 'value'. arg_env: {}", arg_env.to_string()))
                .get_type();

            return Some(unsupported_operand_type_err("==", me.get_type(), right_type))
        }

        None
    }

    fn not_eq(arg_env: &mut Environment) -> Option<Object> {
        let me = extract_arg!(arg_env, "me" => AntEnv);

        if me.is_none() {
            return Some(create_error(format!("type mismatch for 'me'")))
        } else if let Some(me) = me {
            if let Some(_value) = extract_arg!(arg_env, "value" => AntEnv) {
                return Some(native_boolean_to_object(!is_truthy(&eq(arg_env).expect(""))))
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

    let type_hint_map = type_hint_map!("value" => type_hint!(ENVIRONMENT));

    let operator_functions = vec![
        ("eq", eq as NativeFunction),
        ("not_eq", not_eq),
    ];

    for (op, func) in operator_functions {
        let native_func_object = create_ant_native_function(
            func_param_env.clone(), Some(type_hint_map.clone()), func
        ); 

        int_obj.env.create(op, Data::new(native_func_object, DataInfo::new(false)));
    }
}

pub fn create_ant_env(obj_env: Environment) -> Box<dyn IAntObject + 'static> {
    AntEnv::new_with_native_value(
        Box::new(obj_env)
    )
}