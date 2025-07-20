use std::any::Any;
use uuid::Uuid;

use crate::object::object::EnvGetter;
use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::{IAntObject, Object, ObjectType};

pub struct AntClass {
    pub id: Uuid,
    pub name: String,
    pub base: Option<Object>,
    pub env: Environment,
}

impl Clone for AntClass {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            base: self.base.clone(),
            env: self.env.clone(),
        }
    }
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
