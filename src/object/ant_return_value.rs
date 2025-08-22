use std::any::Any;
use uuid::Uuid;

use crate::impl_object;
use crate::object::object::{IAntObject, Object, ObjectType, RETURN_VALUE};

pub struct AntReturnValue {
    id: Uuid,
    pub value: Object,
}

impl IAntObject for AntReturnValue {
    fn get_type(&self) -> ObjectType {
        RETURN_VALUE.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        self.value.get_value()
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("<ReturnValue>({})", self.value.inspect())
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
            || if other.get_type() == RETURN_VALUE {
                other
                    .as_any()
                    .downcast_ref::<AntReturnValue>()
                    .unwrap()
                    .value
                    == self.value.clone()
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for AntReturnValue {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: self.value.clone(),
        }
    }
}

impl_object!(AntReturnValue);
