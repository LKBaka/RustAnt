use std::any::Any;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use uuid::Uuid;
use dyn_clone::{clone_trait_object, DynClone};

use crate::environment::environment::Environment;
use crate::impl_object;

pub type ObjectType = String;
pub type Object = Box<dyn IAntObject>;

pub const ANY: &str = "Any";
pub const OBJECT: &str = "Object";
pub const INT: &str = "Int";
pub const DOUBLE: &str = "Double";
pub const BOOLEAN: &str = "Boolean";
pub const STRING: &str = "String";
pub const NULL: &str = "Null";
pub const UNINIT: &str = "Uninit";
pub const ERROR: &str = "Error";
pub const FUNCTION: &str = "Function";
pub const ENVIRONMENT: &str = "Environment";
pub const NATIVE_FUNCTION: &str = "NativeFunction";
pub const RETURN_VALUE: &str = "__Return_Value__";

pub trait EnvGetter {
    fn get_env(&self) -> Environment;
    fn get_env_ref(&mut self) -> &mut Environment;
}

pub trait IAntObject: DynClone + Sync + Send + Any + EnvGetter + Debug {
    fn get_type(&self) -> ObjectType;
    fn get_value(&self) -> Box<dyn Any>;
    fn get_base(&self) -> Option<Object>;
    fn get_id(&self) -> Uuid;
    fn inspect(&self) -> String;
    fn equals(&self, other: &dyn IAntObject) -> bool;
    fn as_any(&self) -> &dyn Any;
}

clone_trait_object!(IAntObject);

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        self.equals(other.as_ref())
    }
}

impl Eq for Object {}

impl Hash for dyn IAntObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_id().hash(state);
    }
}


pub struct AntObject {
    pub id: Uuid,
    pub env: Environment,
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

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("<AntObject {}>", self.id)
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntObject);