use dyn_clone::{DynClone, clone_trait_object};
use enum_dispatch::enum_dispatch;
use std::any::Any;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

use crate::impl_object;
use crate::obj_enum::object::Object;

pub type ObjectType = String;

pub const ANY: &str = "Any";
pub const OBJECT: &str = "Object";
pub const INT: &str = "Int";
pub const DOUBLE: &str = "Double";
pub const BOOLEAN: &str = "Boolean";
pub const STRING: &str = "String";
pub const NULL: &str = "None";
pub const UNINIT: &str = "Uninit";
pub const ERROR: &str = "Error";
pub const NATIVE_FUNCTION: &str = "NativeFunction";
pub const COMPILED_FUNCTION: &str = "CompiledFunction";
pub const CLOSURE: &str = "Closure";
pub const ARRAY: &str = "Array";

pub trait AsAnyMut {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[enum_dispatch]
pub trait IAntObject: DynClone + Sync + Send + Any + Debug + AsAnyMut {
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
        self.equals(other)
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
}

impl Clone for AntObject {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
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
