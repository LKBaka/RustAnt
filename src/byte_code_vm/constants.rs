use once_cell::sync::Lazy;

use crate::{
    obj_enum::object::Object,
    object::{ant_boolean::AntBoolean, ant_none::AntNone, ant_uninit::AntUninit},
};

pub static TRUE: Lazy<AntBoolean> = Lazy::new(|| AntBoolean::from(true));
pub static FALSE: Lazy<AntBoolean> = Lazy::new(|| AntBoolean::from(false));
pub static UNINIT_OBJ: Lazy<AntUninit> = Lazy::new(AntUninit::create);
pub static NONE_OBJ: Lazy<Object> = Lazy::new(|| AntNone::new());

pub const FAKE_OFFSET_JUMP: u16 = 91 * 2 + 78 * 4 + 13 * 2;
