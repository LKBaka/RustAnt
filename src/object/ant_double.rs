use bigdecimal::BigDecimal;
use std::any::Any;
use uuid::Uuid;

use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::{DOUBLE, IAntObject, Object, ObjectType};

pub struct AntDouble {
    id: Uuid,
    env: Environment,
    pub value: BigDecimal,
}

impl Clone for AntDouble {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            value: self.value.clone(),
        }
    }
}

impl From<BigDecimal> for AntDouble {
    fn from(value: BigDecimal) -> Self {
        AntDouble {
            id: Uuid::new_v4(),
            env: Environment::new(),
            value,
        }
    }
}

impl From<i32> for AntDouble {
    fn from(value: i32) -> Self {
        AntDouble {
            id: Uuid::new_v4(),
            env: Environment::new(),
            value: BigDecimal::from(value),
        }
    }
}

impl IAntObject for AntDouble {
    fn get_type(&self) -> ObjectType {
        DOUBLE.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value.clone())
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("{}", self.value.to_string())
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
            || if other.get_type() == DOUBLE {
                other.as_any().downcast_ref::<AntDouble>().unwrap().value == self.value
            } else if other.get_type() == "INT" {
                // 支持与 AntInt 的比较
                if let Some(int_obj) = other
                    .as_any()
                    .downcast_ref::<crate::object::ant_int::AntInt>()
                {
                    &int_obj.value == &self.value
                } else {
                    false
                }
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntDouble);
