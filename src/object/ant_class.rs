use std::any::Any;
use std::collections::HashMap;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::id_counter::next_id;
use crate::object::object::{IAntObject, ObjectType, CLASS};

#[derive(Clone)]
pub struct AntClass {
    pub id: usize,
    pub map: HashMap<String, Object>,
}

impl IAntObject for AntClass {
    fn get_type(&self) -> ObjectType {
        CLASS.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        unimplemented!()
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn inspect(&self) -> String {
        format!("<Class Id: {}, IdentMap: {:#?}>", self.id, self.map)
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        match (other as &dyn Any).downcast_ref::<AntClass>() {
            Some(it) => it.map == self.map,
            None => false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntClass);

impl From<HashMap<String, Object>> for AntClass {
    fn from(value: HashMap<String, Object>) -> Self {
        Self {
            id: next_id(),
            map: value
        }
    }
}