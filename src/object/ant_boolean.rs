use std::any::Any;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::id_counter::next_id;
use crate::object::object::{BOOLEAN, IAntObject, ObjectType};

#[derive(Clone)]
pub struct AntBoolean {
    id: usize,
    pub value: bool,
}

impl From<bool> for AntBoolean {
    fn from(value: bool) -> Self {
        Self {
            id: next_id(),
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

    fn get_id(&self) -> usize {
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
