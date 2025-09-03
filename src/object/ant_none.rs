use std::any::Any;
use uuid::Uuid;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::object::{IAntObject, NULL, ObjectType};

#[derive(Clone)]
pub struct AntNone {
    id: Uuid,
}

impl AntNone {
    pub fn new() -> Object {
        Object::AntNone(Self {
            id: Uuid::new_v4(),
        })
    }
}

impl IAntObject for AntNone {
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
        "None".to_string()
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

impl_object!(AntNone);
