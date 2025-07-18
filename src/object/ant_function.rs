use std::any::Any;
use uuid::Uuid;

use crate::ast::statements::block_statement::BlockStatement;
use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::EnvGetter;
use crate::object::object::{IAntObject, Object, ObjectType, FUNCTION};

pub struct AntFunction {
    pub id: Uuid,
    pub env: Environment,
    pub param_env: Environment,
    pub block: BlockStatement,
}

impl Clone for AntFunction {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            param_env: self.param_env.clone(),
            block: self.block.clone(),
        }
    }
}

impl IAntObject for AntFunction {
    fn get_type(&self) -> ObjectType {
        FUNCTION.to_string()
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
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntFunction);