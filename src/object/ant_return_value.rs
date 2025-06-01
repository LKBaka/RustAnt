use std::any::Any;
use uuid::Uuid;

use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::{IAntObject, Object, ObjectType, RETURN_VALUE};
use crate::object::object::EnvGetter;

use super::utils::create_error;

pub struct AntReturnValue {
    id: Uuid,
    env: Environment,
    pub value: Object
}

impl AntReturnValue {
    pub fn new_with_native_value(value: Box<dyn Any>) -> Object {
        if value.downcast_ref::<Box<dyn IAntObject + 'static>>().is_none() {
            return create_error(
                "return value is None".to_string()
            )
        }

        let obj = value.downcast_ref::<Object>().unwrap().clone();
        Box::new(Self {
            id: Uuid::new_v4(),
            env: Environment::new(),
            value: obj,
        })
    }
}

impl IAntObject for AntReturnValue {
    fn get_type(&self) -> ObjectType {
        RETURN_VALUE.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        self.value.get_value()
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == RETURN_VALUE {
            other.as_any().downcast_ref::<AntReturnValue>().unwrap().value == self.value.clone()
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

impl_object!(AntReturnValue);