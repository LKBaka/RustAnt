use std::any::Any;
use uuid::Uuid;

use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::{ERROR, IAntObject, Object, ObjectType};

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

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("{}: {}", self.error_name, self.message)
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntError);
