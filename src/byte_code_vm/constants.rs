use once_cell::sync::Lazy;

use crate::{obj_enum::object::Object, object::{ant_boolean::AntBoolean, ant_none::AntNone, ant_uninit::AntUninit}};

pub static UNINIT_OBJ: Lazy<AntUninit> = Lazy::new(AntUninit::create);
pub static UNINIT_OBJECT: Lazy<Object> = Lazy::new(AntUninit::new);
pub static NONE_OBJ: Lazy<Object> = Lazy::new(AntNone::new);
pub static TRUE_OBJ: Lazy<Object> = Lazy::new(|| Object::AntBoolean(AntBoolean::from(true)));
pub static FALSE_OBJ: Lazy<Object> = Lazy::new(|| Object::AntBoolean(AntBoolean::from(false)));

pub const FAKE_OFFSET_JUMP: u16 = 91 * 2 + 78 * 4 + 13 * 2;