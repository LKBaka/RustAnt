use std::any::Any;
use uuid::Uuid;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::object::{IAntObject, ObjectType};

#[derive(Clone)]
pub struct AntClass {
    pub id: Uuid,
    pub name: String,
    pub base: Option<Box<Object>>,
}

impl IAntObject for AntClass {
    fn get_type(&self) -> ObjectType {
        "CLASS".to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        unimplemented!()
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("<Class {}>", self.id)
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntClass);
