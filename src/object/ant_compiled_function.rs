use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use uuid::Uuid;

use crate::byte_code_vm::code::code::{instruction_to_str, Instructions};
use crate::impl_object;
use crate::object::object::{IAntObject, Object, ObjectType, COMPILED_FUNCTION};

pub struct CompiledFunction {
    pub instructions: Rc<RefCell<Instructions>>,
    pub locals_count: usize,
}

impl CompiledFunction {
    fn from(instructions: Rc<RefCell<Instructions>>, locals_count: usize) -> Self {
        Self { instructions, locals_count }
    }
}

impl Clone for CompiledFunction {
    fn clone(&self) -> Self {
        Self {
            instructions: self.instructions.clone(),
            locals_count: self.locals_count
        }
    }
}

impl IAntObject for CompiledFunction {
    fn get_type(&self) -> ObjectType {
        COMPILED_FUNCTION.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(
            self.instructions.clone()
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
            "<CompiledFunction locals_count: {} {}>", 
            self.locals_count, instruction_to_str(&self.instructions.borrow().clone())
        )
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.get_id() || if let Some(compiled_func) = other
            .as_any()
            .downcast_ref::<CompiledFunction>() 
        {
            compiled_func.instructions == self.instructions
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(CompiledFunction);
