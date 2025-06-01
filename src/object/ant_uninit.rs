use std::any::Any;
use uuid::Uuid;

use crate::environment::environment::Environment;
use crate::object::object::{IAntObject, Object, ObjectType, UNINIT};
use crate::object::object::EnvGetter;
use crate::impl_object;

pub struct AntUninit {
    id: Uuid,
    env: Environment,
}

impl AntUninit {
    pub fn new(_arg_env: Environment) -> Object {
        let obj = Box::new(Self {
            id: Uuid::new_v4(),
            env: Environment::new(),
        });

        obj
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

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        "uninit".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    
    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == UNINIT {true} else {false}
    }
}

impl Clone for AntUninit {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
        }
    }
}

impl_object!(AntUninit);