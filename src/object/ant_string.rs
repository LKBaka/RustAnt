use std::any::Any;
use uuid::Uuid;

use crate::impl_object;
use crate::object::object::{IAntObject, Object, ObjectType, STRING};

pub struct AntString {
    id: Uuid,
    pub(crate) value: String,
}

impl Clone for AntString {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: self.value.clone(),
        }
    }
}

impl AntString {
    pub fn new(s: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            value: s,
        }
    }
}

impl IAntObject for AntString {
    fn get_type(&self) -> ObjectType {
        STRING.to_string()
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
        self.value.clone()
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
            || if other.get_type() == STRING {
                other.as_any().downcast_ref::<AntString>().unwrap().value == self.value
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntString);
