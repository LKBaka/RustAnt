use uuid::Uuid;
use std::any::Any;
use enum_dispatch::enum_dispatch;

use crate::object::ant_array::AntArray;
use crate::object::ant_boolean::AntBoolean;
use crate::object::ant_class::AntClass;
use crate::object::ant_closure::Closure;
use crate::object::ant_compiled_function::CompiledFunction;
use crate::object::ant_double::AntDouble;
use crate::object::ant_error::AntError;
use crate::object::ant_int::AntInt;
use crate::object::ant_native_function::AntNativeFunction;
use crate::object::ant_none::AntNone;
use crate::object::ant_string::AntString;
use crate::object::ant_uninit::AntUninit;
use crate::object::object::AsAnyMut;
use crate::object::object::ObjectType;
use crate::object::object::IAntObject;


#[enum_dispatch(IAntObject)]
#[derive(Debug, Clone)]
pub enum Object {
    AntArray,
    AntBoolean,
    AntClass,
    Closure,
    CompiledFunction,
    AntDouble,
    AntError,
    AntInt,
    AntNativeFunction,
    AntNone,
    AntString,
    AntUninit
}

impl AsAnyMut for Object {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}