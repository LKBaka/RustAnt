use std::any::Any;
use uuid::Uuid;

use crate::constants::null_obj;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::{IAntObject, Object, ObjectType, STRING};
use crate::object::object::EnvGetter;


pub struct AntString {
    id: Uuid,
    env: Environment,
    pub(crate) value: String,
}

impl Clone for AntString {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            value: self.value.clone(),
        }
    }
}

impl AntString {
    pub fn new_with_native_value(mut value: Box<dyn Any>) -> Object {
        let cast_result = value.downcast_mut::<String>().cloned();

        match cast_result {
            None => {
                panic!("value is not String")
            }
            Some(s) => {
                let mut env = Environment::new();
                env.create("value", Data::new(null_obj.clone(), DataInfo::new(false)));

                Box::new(Self {
                    id: Uuid::new_v4(),
                    env,
                    value: s.clone()
                })
            }
        }

    }
}

impl IAntObject for AntString {
    fn get_type(&self) -> ObjectType {
        STRING.to_string()
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
        self.value.clone()
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == STRING {
            other.as_any().downcast_ref::<AntString>().unwrap().value == self.value
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntString);