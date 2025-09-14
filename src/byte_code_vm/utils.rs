use crate::{
    byte_code_vm::constants::{FALSE_OBJ, TRUE_OBJ}, obj_enum::object::Object,
};

#[inline(always)]
pub fn native_boolean_to_object(value: bool) -> Object {
    if value {
        TRUE_OBJ.clone()
    } else {
        FALSE_OBJ.clone()
    }
}
