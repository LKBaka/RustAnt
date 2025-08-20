use std::any::Any;
use uuid::Uuid;

use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::{IAntObject, NULL, Object, ObjectType};

pub struct AntNull {
    id: Uuid,
    env: Environment,
}

impl AntNull {
    pub fn new(_arg_env: Environment) -> Object {
        Box::new(Self {
            id: Uuid::new_v4(),
            env: Environment::new(),
        })
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
        other.get_id() == self.id
            || if other.get_type() == NULL {
                true
            } else {
                false
            }
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

impl_object!(AntNull);
