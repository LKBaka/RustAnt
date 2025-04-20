use std::any::{Any, TypeId};
use std::fmt::Debug;

pub fn assert_eq<T, F>(left: T, right: T, on_failure: F)
where
    T: PartialEq + Debug,        // 允许比较和格式化输出
    F: FnOnce(),                 // 接受无参闭包/函数指针
{
    if left != right {
        on_failure();            // 断言失败时调用回调函数
        panic!("assertion failed: left != right\n  left: `{:?}`,\n right: `{:?}`", left, right);
    }
}

pub fn type_of<T: Any>(_: &T) -> TypeId {
    TypeId::of::<T>()
}