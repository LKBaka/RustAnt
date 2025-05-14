use std::any::Any;
use std::ops::Deref;
use uuid::Uuid;

use crate::object::object::GetEnv;
use crate::constants::null_obj;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::ant_string::AntString;
use crate::object::object::{IAntObject, ObjectType, ERROR};

pub struct AntError {
    pub id: Uuid,
    pub env: Environment,
    pub error_name: String,
    pub message: String,
}

impl Clone for AntError {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            error_name: self.error_name.clone(),
            message: self.message.clone(),
        }
    }
}

impl IAntObject for AntError {
    fn get_type(&self) -> ObjectType {
        ERROR.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.message.clone())
    }

    fn get_base(&self) -> Option<Box<dyn IAntObject>> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("{}: {}", self.error_name, self.message)
    }

    fn new(arg_env: Environment) -> Box<dyn IAntObject> {
        let mut value = String::from("");

        let mut new = |obj: Box<dyn IAntObject>| {
            let cast_obj =  obj.as_any().downcast_ref::<AntString>();
            match cast_obj {
                None => {
                    panic!()
                }
                Some(str_obj) => {
                    value = str_obj.value.to_owned()
                }
            }
        };

        let mut env = Environment::new();
        env.create("value", Data::new(null_obj.clone(), DataInfo::new(false)));

        env.fusion(arg_env);

        if env.get("value").unwrap().equals(null_obj.clone().deref()) {
            panic!()
        }

        new(env.get("value").unwrap());

        Box::new(Self {
            id: Uuid::new_v4(),
            env: env.clone(),
            error_name: "error".to_string(),
            message: value,
        })
    }

    fn new_with_native_value(mut value: Box<dyn Any>) -> Box<dyn IAntObject> {
        let cast_result = value.downcast_mut::<String>().cloned();

        match cast_result {
            None => {
                panic!("message is not String")
            }
            Some(s) => {
                let mut env = Environment::new();
                env.create("value", Data::new(
                    AntString::new_with_native_value(Box::new(s.clone())),
                    DataInfo::new(false))
                );

                Box::new(Self {
                    id: Uuid::new_v4(),
                    env,
                    error_name: "error".to_string(),
                    message: s
                })
            }
        }

    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntError);
