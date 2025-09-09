use std::any::Any;

use hashbrown::HashMap;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::id_counter::next_id;
use crate::object::object::{IAntObject, ObjectType, CLASS};

#[derive(Clone)]
pub struct AntClass {
    pub id: usize,
    pub map: HashMap<Object, Object>,
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
        other.get_id() == self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntClass);

impl From<HashMap<Object, Object>> for AntClass {
    fn from(value: HashMap<Object, Object>) -> Self {
        Self {
            id: next_id(),
            map: value
        }
    }
}