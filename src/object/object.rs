#![allow(unused_imports)]

use dyn_clone::{DynClone, clone_trait_object};
use enum_dispatch::enum_dispatch;
use std::any::Any;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::impl_object;
use crate::obj_enum::object::Object;

use crate::object::ant_array::AntArray;
use crate::object::ant_hash_map::AntHashMap;
use crate::object::ant_boolean::AntBoolean;
use crate::object::ant_class::AntClass;
use crate::object::ant_closure::Closure;
use crate::object::ant_compiled_function::CompiledFunction;
use crate::object::ant_double::AntDouble;
use crate::object::ant_error::AntError;
use crate::object::ant_int::AntInt;
use crate::object::ant_method::Method;
use crate::object::ant_native_function::AntNativeFunction;
use crate::object::ant_none::AntNone;
use crate::object::ant_string::AntString;
use crate::object::ant_uninit::AntUninit;
use crate::object::ant_i64::AntI64;

pub type ObjectType = String;

pub const ANY: &str = "Any";
pub const OBJECT: &str = "Object";
pub const INT: &str = "Int";
pub const I64: &str = "i64";
pub const DOUBLE: &str = "Double";
pub const BOOLEAN: &str = "Boolean";
pub const STRING: &str = "String";
pub const NULL: &str = "None";
pub const UNINIT: &str = "Uninit";
pub const ERROR: &str = "Error";
pub const NATIVE_FUNCTION: &str = "NativeFunction";
pub const COMPILED_FUNCTION: &str = "CompiledFunction";
pub const CLOSURE: &str = "Closure";
pub const METHOD: &str = "Method";
pub const ARRAY: &str = "Array";
pub const HASH_MAP: &str = "HashMap";
pub const CLASS: &str = "Class";

pub trait AsAnyMut {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[enum_dispatch]
pub trait IAntObject: DynClone + Sync + Send + Any + Debug + AsAnyMut {
    fn get_type(&self) -> ObjectType;
    fn get_value(&self) -> Box<dyn Any>;
    fn get_base(&self) -> Option<Object>;
    fn get_id(&self) -> usize;
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
        // 64-bit MurmurMixer 常量，可防连续低位
        let mut x = self.get_id() as u64;
        x = x.wrapping_mul(0x9e3779b97f4a7c15);
        x ^= x >> 32;
        x.hash(state);
    }
}

#[derive(Clone)]
pub struct AntObject {
    pub id: usize,
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

    fn get_id(&self) -> usize {
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
