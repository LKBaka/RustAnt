use bigdecimal::BigDecimal;
use std::any::Any;
use uuid::Uuid;

use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::ant_double::AntDouble;
use crate::object::object::{IAntObject, INT, Object, ObjectType};

pub struct AntInt {
    id: Uuid,
    env: Environment,
    pub value: BigDecimal,
}

impl Clone for AntInt {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            value: self.value.clone(),
        }
    }
}

impl IAntObject for AntInt {
    fn get_type(&self) -> ObjectType {
        INT.to_string()
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
            || if other.get_type() == INT {
                other.as_any().downcast_ref::<AntInt>().unwrap().value == self.value
            } else if let Some(double_obj) = other.as_any().downcast_ref::<AntDouble>() {
                &double_obj.value == &self.value
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntInt);

impl From<i32> for AntInt {
    fn from(value: i32) -> Self {
        AntInt {
            id: Uuid::new_v4(),
            env: Environment::new(),
            value: BigDecimal::from(value),
        }
    }
}

impl From<BigDecimal> for AntInt {
    fn from(value: BigDecimal) -> Self {
        AntInt {
            id: Uuid::new_v4(),
            env: Environment::new(),
            value,
        }
    }
}
