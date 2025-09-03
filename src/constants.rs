use lazy_static::lazy_static;

use crate::obj_enum::object::Object;
use crate::object::ant_boolean::AntBoolean;
use crate::object::ant_none::AntNone;
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
