use std::any::Any;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::id_counter::next_id;
use crate::object::object::{IAntObject, ObjectType, STRING};

#[derive(Clone)]
pub struct AntString {
    pub id: usize,
    pub value: String,
}

impl AntString {
    pub fn new(s: String) -> Self {
        Self {
            id: next_id(),
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

    fn get_id(&self) -> usize {
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

impl From<&str> for AntString {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl From<String> for AntString {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl_object!(AntString);
