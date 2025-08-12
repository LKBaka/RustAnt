use std::any::Any;
use uuid::Uuid;

use crate::constants::null_obj;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::{IAntObject, ObjectType, BOOLEAN};

use super::object::Object;

pub struct AntBoolean {
    id: Uuid,
    env: Environment,
    pub value: bool,
}

impl Clone for AntBoolean {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            value: self.value,
        }
    }
}

impl From<bool> for AntBoolean {
    fn from(value: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            env: Environment::new(),
            value,
        }
    }
}

impl AntBoolean {
    pub fn new_with_native_value(mut value: Box<dyn Any>) -> Object {
        let cast_result = value.downcast_mut::<bool>().cloned();

        match cast_result {
            None => {
                panic!("value is not boolean")
            }
            Some(boolean) => {
                let mut env = Environment::new();
                env.create("value", Data::new(null_obj.clone(), DataInfo::new(false)));

                Box::new(Self {
                    id: Uuid::new_v4(),
                    env,
                    value: boolean.clone()
                })
            }
        }
    }
}

impl IAntObject for AntBoolean {
    fn get_type(&self) -> ObjectType {
        BOOLEAN.to_string()
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
        other.get_id() == self.id || if other.get_type() == BOOLEAN {
            other.as_any().downcast_ref::<AntBoolean>().unwrap().value == self.value
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntBoolean);