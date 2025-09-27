use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::hash::{Hash, Hasher};

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::ant_compiled_function::CompiledFunction;
use crate::object::object::{CLOSURE, IAntObject, ObjectType};

#[derive(Clone)]
pub struct Closure {
    pub func: CompiledFunction,
    pub free: Rc<RefCell<Vec<Object>>>,
}

impl Hash for Closure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.func.hash(state);
        self.free.borrow().hash(state);
    }
}

impl IAntObject for Closure {
    fn get_type(&self) -> ObjectType {
        CLOSURE.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.func.instructions.clone())
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> usize {
        self.func.get_id()
    }

    fn inspect(&self) -> String {
        format!(
            "<Closure compiled_function: {}, free: {:?}>",
            self.func.inspect(),
            &self.free
        )
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.get_id()
            || if let Some(it) = other.as_any().downcast_ref::<Closure>() {
                it.func == self.func && it.free == self.free
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Eq for Closure {}

impl_object!(Closure);
