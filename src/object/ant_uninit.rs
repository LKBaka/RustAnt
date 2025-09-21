use std::any::Any;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::id_counter::next_id;
use crate::object::object::{IAntObject, ObjectType, UNINIT};

#[derive(Clone)]
pub struct AntUninit {
    id: usize,
}

impl AntUninit {
    pub fn new() -> Object {
        Object::AntUninit(Self { id: next_id() })
    }

    pub fn create() -> Self {
        Self { id: next_id() }
    }
}

impl IAntObject for AntUninit {
    fn get_type(&self) -> ObjectType {
        UNINIT.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(())
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn inspect(&self) -> String {
        "uninit".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || other.get_type() == UNINIT
    }
}

impl_object!(AntUninit);
