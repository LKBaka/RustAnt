use std::any::Any;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use uuid::Uuid;
use dyn_clone::{clone_trait_object, DynClone};

use crate::environment::environment::Environment;
use crate::impl_object;

pub type ObjectType = String;

pub const OBJECT: &str = "Object";
pub const INT: &str = "Int";
pub const DOUBLE: &str = "Double";
pub const BOOLEAN: &str = "Boolean";
pub const STRING: &str = "String";
pub const NULL: &str = "Null";
pub const UNINIT: &str = "Uninit";
pub const ERROR: &str = "Error";
pub const FUNCTION: &str = "Function";
pub const NATIVE_FUNCTION: &str = "NativeFunction";
pub const RETURN_VALUE: &str = "__Return_Value__";

pub trait GetEnv {
    fn get_env(&self) -> Environment;
    fn get_env_ref(&mut self) -> &mut Environment;
}

pub trait IAntObject: DynClone + Sync + Send + Any + GetEnv {
    fn get_type(&self) -> ObjectType;
    fn get_value(&self) -> Box<dyn Any>;
    fn get_base(&self) -> Option<Box<dyn IAntObject>>;
    fn get_id(&self) -> Uuid;
    fn inspect(&self) -> String;
    fn new(arg_env: Environment) -> Box<dyn IAntObject> where Self: Sized;
    fn new_with_native_value(value: Box<dyn Any>) -> Box<dyn IAntObject> where Self: Sized;
    fn equals(&self, other: &dyn IAntObject) -> bool;
    fn as_any(&self) -> &dyn Any;
}

clone_trait_object!(IAntObject);

impl PartialEq for Box<dyn IAntObject> {
    fn eq(&self, other: &Box<dyn IAntObject>) -> bool {
        self.equals(other.deref())
    }
}

impl Eq for Box<dyn IAntObject> {}

impl Hash for dyn IAntObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_id().hash(state);
    }
}


pub struct AntObject {
    id: Uuid,
    env: Environment,
}

impl Clone for AntObject {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
        }
    }
}

impl IAntObject for AntObject {
    fn get_type(&self) -> ObjectType {
        OBJECT.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(())
    }

    fn get_base(&self) -> Option<Box<dyn IAntObject>> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("AntObject(id: {})", self.id)
    }

    fn new(_arg_env: Environment) -> Box<dyn IAntObject> {
        Box::new(Self {
            id: Uuid::new_v4(),
            env: Environment::new(),
        })
    }

    fn new_with_native_value(_value: Box<dyn Any>) -> Box<dyn IAntObject>
    {
        Box::new(Self {
            id: Uuid::new_v4(),
            env: Environment::new(),
        })
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntObject);