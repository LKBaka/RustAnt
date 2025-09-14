use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::ant_closure::Closure;
use crate::object::ant_native_function::AntNativeFunction;
use crate::object::object::{IAntObject, ObjectType, METHOD};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MethodType {
    Closure(Closure),
    NativeFunction(AntNativeFunction)
}

impl MethodType {
    pub fn inspect(&self) -> String {
        match self {
            MethodType::Closure(cl) => cl.inspect(),
            MethodType::NativeFunction(f) => f.inspect()
        }
    }

    pub fn id(&self) -> usize {
        match self {
            MethodType::Closure(cl) => cl.get_id(),
            MethodType::NativeFunction(f) => f.get_id()
        }
    }
}

#[derive(Clone, Eq)]
pub struct Method {
    pub me: Option<Rc<RefCell<Object>>>,
    pub func: MethodType,
}

impl IAntObject for Method {
    fn get_type(&self) -> ObjectType {
        METHOD.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        unimplemented!()
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> usize {
        self.func.id()
    }

    fn inspect(&self) -> String {
        format!(
            "<Method self: {:?}, func: {}>",
            self.me,
            self.func.inspect(),
        )
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.get_id()
            || if let Some(it) = other.as_any().downcast_ref::<Method>() {
                it.me == self.me && it.func == self.func
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(Method);
