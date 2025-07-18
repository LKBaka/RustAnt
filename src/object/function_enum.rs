use crate::object::object::Object;

pub enum Function {
    Func(Object),
    NativeFunc(Object),
}