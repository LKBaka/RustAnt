use lazy_static::lazy_static;

use crate::environment::environment::Environment;
use crate::object::ant_boolean::AntBoolean;
use crate::object::ant_null::AntNull;
use crate::object::ant_uninit::AntUninit;
use crate::object::object::IAntObject;

pub const NULL_CHAR: char = '\0';
pub const NEW_LINE: char = '\n';

lazy_static! {
    pub static ref null_obj: Box<dyn IAntObject> = {
        AntNull::new(Environment::new())
    };

    pub static ref uninit_obj: Box<dyn IAntObject> = {
        AntUninit::new(Environment::new())
    };

    pub static ref ant_true: Box<dyn IAntObject> = {
        AntBoolean::new_with_native_value(Box::new(true))
    };

    pub static ref ant_false: Box<dyn IAntObject> = {
        AntBoolean::new_with_native_value(Box::new(false))
    };
}
