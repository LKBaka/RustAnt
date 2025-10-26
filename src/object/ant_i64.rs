use std::any::Any;

use bigdecimal::BigDecimal;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::ant_double::AntDouble;
use crate::object::id_counter::next_id;
use crate::object::object::{IAntObject, ObjectType, I64};

#[derive(Clone, Copy)]
pub struct AntI64 {
    pub id: usize,
    pub value: i64,
}

impl IAntObject for AntI64 {
    fn get_type(&self) -> ObjectType {
        I64.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value)
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
            || if other.get_type() == I64 {
                other.as_any().downcast_ref::<AntI64>().unwrap().value == self.value
            } else if let Some(double_obj) = other.as_any().downcast_ref::<AntDouble>() {
                &double_obj.value == &BigDecimal::from(self.value)
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntI64);

impl From<i32> for AntI64 {
    fn from(value: i32) -> Self {
        AntI64 {
            id: next_id(),
            value: value as i64,
        }
    }
}

impl From<i64> for AntI64 {
    fn from(value: i64) -> Self {
        AntI64 {
            id: next_id(),
            value,
        }
    }
}