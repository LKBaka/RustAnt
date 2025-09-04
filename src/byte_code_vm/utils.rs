use crate::{
    byte_code_vm::constants::{FALSE, TRUE}, obj_enum::object::Object,
};

pub fn native_boolean_to_object(value: bool) -> Object {
    Object::AntBoolean(if value { TRUE.clone() } else { FALSE.clone() })
}
