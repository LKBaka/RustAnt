use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use uuid::Uuid;

use crate::impl_object;
use crate::object::ant_compiled_function::CompiledFunction;
use crate::object::object::{IAntObject, Object, ObjectType, CLOSURE};

#[derive(Clone)]
pub struct Closure {
    pub func: Rc<RefCell<CompiledFunction>>,
    pub free: Rc<RefCell<Vec<Object>>>
}

impl IAntObject for Closure {
    fn get_type(&self) -> ObjectType {
        CLOSURE.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(
            self.func.borrow().instructions.clone()
        )
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        uuid::Uuid::new_v4()
    }

    fn inspect(&self) -> String {
        format!(
            "<Closure compiled_function: {}, free: {:?}>",
            self.func.borrow().inspect(),
            &self.free
        )
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.get_id() || if let Some(it) = other
            .as_any()
            .downcast_ref::<Closure>() 
        {
            it.func == self.func && it.free == self.free
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(Closure);
