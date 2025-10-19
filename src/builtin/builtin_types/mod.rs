use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    builtin::builtin_types::{array_type::ARRAY_MEMBERS, int_type::INT_MEMBERS},
    obj_enum::object::Object,
    object::object::{ARRAY, INT},
};

pub mod array_type;
pub mod int_type;

pub static BUILTIN_TYPE_MAP: Lazy<HashMap<String, HashMap<String, Object>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(INT.to_string(), INT_MEMBERS.clone());
    m.insert(ARRAY.to_string(), ARRAY_MEMBERS.clone());

    m
});
