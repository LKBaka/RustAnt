use std::any::Any;
use uuid::Uuid;

use crate::impl_object;
use crate::object::object::{BOOLEAN, IAntObject, ObjectType};

use super::object::Object;

pub struct AntBoolean {
    id: Uuid,
    pub value: bool,
}

impl Clone for AntBoolean {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: self.value,
        }
    }
}

impl From<bool> for AntBoolean {
    fn from(value: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            value,
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
        other.get_id() == self.id
            || if other.get_type() == BOOLEAN {
                other.as_any().downcast_ref::<AntBoolean>().unwrap().value == self.value
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntBoolean);
