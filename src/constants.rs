use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use once_cell::sync::Lazy;

use crate::obj_enum::object::Object;
use crate::object::ant_array::AntArray;
use crate::object::ant_boolean::AntBoolean;
use crate::object::ant_none::AntNone;
use crate::object::ant_string::AntString;
use crate::object::ant_uninit::AntUninit;

pub const NULL_CHAR: char = '\0';
pub const NEW_LINE: char = '\n';

lazy_static! {
    pub static ref null_obj: Object = {
        AntNone::new()
    };

    // 未初始化对象 通常用于在初始化函数参数前填充形参, 在类中则表示需要定义 (参考接口)
    pub static ref uninit_obj: Object = {
        AntUninit::new()
    };

    pub static ref ant_true: Object = {
        Object::AntBoolean(AntBoolean::from(true))
    };

    pub static ref ant_false: Object = {
        Object::AntBoolean(AntBoolean::from(false))
    };
}

pub static MODULE_PATHS: Lazy<Arc<Mutex<AntArray>>> = Lazy::new(
    || Arc::new(Mutex::new(AntArray::from(vec![
        Object::AntString(AntString::new(
            std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        )),
    ])))
);