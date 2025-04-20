use std::any::Any;
use std::ops::Deref;
use uuid::Uuid;

use crate::constants::null_obj;
use crate::environment::environment::Environment;
use crate::impl_object_get_env_function;
use crate::object::object::{IAntObject, ObjectType, RETURN_VALUE};
use crate::object::object::GetEnv;

pub struct AntReturnValue {
    id: Uuid,
    env: Environment,
    pub value: Box<dyn IAntObject>
}

impl IAntObject for AntReturnValue {
    fn get_type(&self) -> ObjectType {
        RETURN_VALUE.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        self.value.get_value()
    }

    fn get_base(&self) -> Option<Box<dyn IAntObject>> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }

    fn new(_arg_env: Environment) -> Box<dyn IAntObject> {
        Box::new(Self {
            id: Uuid::new_v4(),
            env: Environment::new(),
            value: null_obj.clone(),
        })
    }

    fn new_with_native_value(value: Box<dyn Any>) -> Box<dyn IAntObject> {
        if value.downcast_ref::<Box<dyn IAntObject>>().is_none() {
            return Box::new(Self {
                id: Uuid::new_v4(),
                env: Environment::new(),
                value: null_obj.clone(),
            })
        }

        let obj = value.downcast_ref::<Box<dyn IAntObject>>().unwrap().clone();
        Box::new(Self {
            id: Uuid::new_v4(),
            env: Environment::new(),
            value: obj,
        })
    }

    fn eq(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == RETURN_VALUE {
            other.as_any().downcast_ref::<AntReturnValue>().unwrap().value.eq(self.value.clone().deref())
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for AntReturnValue {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            value: self.value.clone()
        }
    }
}

impl_object_get_env_function!(AntReturnValue);