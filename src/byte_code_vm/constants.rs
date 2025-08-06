use once_cell::sync::Lazy;

use crate::object::ant_boolean::AntBoolean;

pub static TRUE: Lazy<AntBoolean> = Lazy::new(|| AntBoolean::from(true));
pub static FALSE: Lazy<AntBoolean> = Lazy::new(|| AntBoolean::from(false));