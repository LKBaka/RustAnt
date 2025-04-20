use std::any::Any;
use uuid::Uuid;

use crate::constants::null_obj;
use crate::environment::environment::Environment;
use crate::impl_object_get_env_function;
use crate::object::object::{IAntObject, ObjectType, NATIVE_FUNCTION};
use crate::object::object::GetEnv;

type NativeFunction = fn(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>>;

pub struct AntNativeFunction {
    pub id: Uuid,
    pub env: Environment,
    pub param_env: Environment,
    pub function: NativeFunction,
}

impl Clone for AntNativeFunction {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            param_env: self.param_env.clone(),
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

    fn get_base(&self) -> Option<Box<dyn IAntObject>> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("<function id: {}>", self.id)
    }

    fn new(_: Environment) -> Box<dyn IAntObject> {
        null_obj.clone()
    }

    fn new_with_native_value(_: Box<dyn Any>) -> Box<dyn IAntObject> {
        null_obj.clone()
    }

    fn eq(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == NATIVE_FUNCTION {
            other.as_any().downcast_ref::<AntNativeFunction>().unwrap().function == self.function
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn create_ant_native_function(param_env: Environment, function: NativeFunction) -> Box<dyn IAntObject> {
    let env = Environment::new();
    let id = Uuid::new_v4();

    Box::new(
        AntNativeFunction {
            id,
            env,
            param_env,
            function
        }
    )
}

impl_object_get_env_function!(AntNativeFunction);