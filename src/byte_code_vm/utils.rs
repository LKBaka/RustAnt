use crate::{
    byte_code_vm::constants::{FALSE, TRUE},
    object::object::Object,
};

pub fn native_boolean_to_object(value: bool) -> Object {
    Box::new(if value { TRUE.clone() } else { FALSE.clone() })
}
