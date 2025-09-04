use std::any::Any;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::object::{ERROR, IAntObject, ObjectType};

#[derive(Clone)]
pub struct AntError {
    pub id: usize,
    pub error_name: String,
    pub message: String,
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

    fn get_id(&self) -> usize {
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
