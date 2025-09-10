use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::id_counter::next_id;
use crate::object::object::{IAntObject, NATIVE_FUNCTION, ObjectType};

use super::type_hint::TypeHintMap;

pub type NativeFunction = fn(args: Vec<Rc<RefCell<Object>>>) -> Option<Object>;

#[derive(Clone)]
pub struct AntNativeFunction {
    pub id: usize,
    pub type_hint_map: Option<TypeHintMap>,
    pub function: NativeFunction,
}

impl IAntObject for AntNativeFunction {
    fn get_type(&self) -> ObjectType {
        NATIVE_FUNCTION.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(())
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn inspect(&self) -> String {
        format!("<NativeFunction id: {} func: {:?}>", self.id, self.function)
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
            || if other.get_type() == NATIVE_FUNCTION {
                std::ptr::fn_addr_eq(
                    other
                        .as_any()
                        .downcast_ref::<AntNativeFunction>()
                        .unwrap()
                        .function,
                    self.function,
                )
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn create_ant_native_function(
    type_hint_map: Option<TypeHintMap>,
    function: NativeFunction,
) -> AntNativeFunction {
    let id = next_id();

    AntNativeFunction {
        id,
        type_hint_map,
        function,
    }
}

impl_object!(AntNativeFunction);
