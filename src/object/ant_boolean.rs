use std::any::Any;
use uuid::Uuid;

use crate::constants::null_obj;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::{IAntObject, ObjectType, BOOLEAN};
use crate::object::object::GetEnv;
use crate::object::utils::{create_error, is_error};

pub struct AntBoolean {
    id: Uuid,
    env: Environment,
    value: bool,
}

impl Clone for AntBoolean {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            value: self.value.clone(),
        }
    }
}

impl IAntObject for AntBoolean {
    fn get_type(&self) -> ObjectType {
        BOOLEAN.to_string()
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
        let mut value = false;

        let mut new = |obj: Box<dyn IAntObject>| {
            let cast_obj =  obj.as_any().downcast_ref::<AntBoolean>();
            if let Some(obj) = cast_obj {
                value = obj.value; return null_obj.clone()
            }

            create_error(format!("value is not {}", BOOLEAN))
        };

        let mut env = Environment::new();
        env.create("value", Data::new(null_obj.clone(), DataInfo::new(false)));

        env = env.fusion(arg_env);


        if env.get("value").unwrap() == null_obj.clone() {
            panic!()
        }

        let create_result = new(env.get("value").unwrap());
        if is_error(&create_result) {return create_result}

        Box::new(Self {
            id: Uuid::new_v4(),
            env: env.clone(),
            value,
        })
    }

    fn new_with_native_value(mut value: Box<dyn Any>) -> Box<dyn IAntObject> {
        let cast_result = value.downcast_mut::<bool>().cloned();

        match cast_result {
            None => {
                panic!("value is not boolean")
            }
            Some(boolean) => {
                let mut env = Environment::new();
                env.create("value", Data::new(null_obj.clone(), DataInfo::new(false)));

                Box::new(Self {
                    id: Uuid::new_v4(),
                    env,
                    value: boolean.clone()
                })
            }
        }
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == BOOLEAN {
            other.as_any().downcast_ref::<AntBoolean>().unwrap().value == self.value
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntBoolean);