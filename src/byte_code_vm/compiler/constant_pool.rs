use once_cell::sync::Lazy;

use crate::{obj_enum::object::Object, object::ant_int::AntInt};

pub static CONSTANT_POOL_0_256: Lazy<Vec<Object>> = Lazy::new(|| {
    let mut v: Vec<Object> = Vec::with_capacity(256);

    for i in 0..=256 {
        v.push(Object::AntInt(AntInt::from(i)));
    }

    v
});
