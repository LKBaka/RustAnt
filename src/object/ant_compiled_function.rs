use std::any::Any;
use std::rc::Rc;

use crate::byte_code_vm::code::code::instruction_to_str;
use crate::byte_code_vm::scope_info::ScopeInfo;
use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::object::{COMPILED_FUNCTION, IAntObject, ObjectType};

#[derive(Eq, Hash)]
pub struct CompiledFunction {
    #[cfg(feature = "debug")]
    pub id: usize,
    pub instructions: Rc<[u8]>,
    pub local_count: usize,
    pub param_count: usize,
    pub scope_info: ScopeInfo,
}

impl Clone for CompiledFunction {
    fn clone(&self) -> Self {
        Self {
            #[cfg(feature = "debug")]
            id: self.id,
            instructions: self.instructions.clone(),
            local_count: self.local_count,
            param_count: self.param_count,
            scope_info: self.scope_info.clone(),
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

    fn get_id(&self) -> usize {
        #[cfg(feature = "debug")]
        return self.id;

        #[cfg(not(feature = "debug"))]
        0x33550336
    }

    fn inspect(&self) -> String {
        #[cfg(feature = "debug")]
        return format!(
            "<CompiledFunction id: {} locals_count: {} param_count: {} {}>",
            self.id,
            self.local_count,
            self.param_count,
            instruction_to_str(&self.instructions.to_vec())
        );

        #[cfg(not(feature = "debug"))]
        format!(
            "<CompiledFunction locals_count: {} param_count: {} {}>",
            self.local_count,
            self.param_count,
            instruction_to_str(&self.instructions.to_vec())
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
