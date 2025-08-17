use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use uuid::Uuid;

use crate::byte_code_vm::code::code::{Instructions, instruction_to_str};
use crate::impl_object;
use crate::object::object::{COMPILED_FUNCTION, IAntObject, Object, ObjectType};

pub struct CompiledFunction {
    pub instructions: Rc<RefCell<Instructions>>,
    pub local_count: usize,
    pub param_count: usize,
}

impl Clone for CompiledFunction {
    fn clone(&self) -> Self {
        Self {
            instructions: self.instructions.clone(),
            local_count: self.local_count,
            param_count: self.param_count,
        }
    }
}

impl IAntObject for CompiledFunction {
    fn get_type(&self) -> ObjectType {
        COMPILED_FUNCTION.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.instructions.clone())
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        uuid::Uuid::new_v4()
    }

    fn inspect(&self) -> String {
        format!(
            "<CompiledFunction locals_count: {} param_count: {} {}>",
            self.local_count,
            self.param_count,
            instruction_to_str(&self.instructions.borrow().clone())
        )
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.get_id()
            || if let Some(compiled_func) = other.as_any().downcast_ref::<CompiledFunction>() {
                compiled_func.instructions == self.instructions
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(CompiledFunction);
