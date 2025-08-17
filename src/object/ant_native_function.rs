use std::any::Any;
use uuid::Uuid;

use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::{IAntObject, NATIVE_FUNCTION, Object, ObjectType};

use super::type_hint::TypeHintMap;

pub type NativeFunction = fn(arg_env: &mut Environment) -> Option<Object>;

pub struct AntNativeFunction {
    pub id: Uuid,
    pub env: Environment,
    pub param_env: Environment,
    pub type_hint_map: Option<TypeHintMap>,
    pub function: NativeFunction,
}

impl Clone for AntNativeFunction {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            param_env: self.param_env.clone(),
            type_hint_map: self.type_hint_map.clone(),
            function: self.function.clone(),
        }
    }
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

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("<function id: {}>", self.id)
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
    param_env: Environment,
    type_hint_map: Option<TypeHintMap>,
    function: NativeFunction,
) -> Object {
    let env = Environment::new();
    let id = Uuid::new_v4();

    Box::new(AntNativeFunction {
        id,
        env,
        param_env,
        type_hint_map,
        function,
    })
}

impl_object!(AntNativeFunction);
