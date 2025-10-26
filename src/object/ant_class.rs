use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::id_counter::next_id;
use crate::object::object::{IAntObject, ObjectType, CLASS};

#[derive(Clone)]
pub struct AntClass {
    pub id: usize,
    pub name: Rc<str>,
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
        format!(
            "class {} {}",
            self.name, if self.map.is_empty() { "{}" } else { "{ ... }" }
        )
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

impl From<(&str, HashMap<String, Object>)> for AntClass {
    fn from(value: (&str, HashMap<String, Object>)) -> Self {
        let (name, map) = value;

        Self {
            id: next_id(),
            name: name.into(),
            map,
        }
    }
}