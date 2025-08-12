use std::any::Any;
use uuid::Uuid;

use crate::environment::environment::Environment;
use crate::impl_object;
use crate::object::object::{IAntObject, Object, ObjectType, ARRAY, STRING};

pub struct AntArray {
    id: Uuid,
    env: Environment,
    pub items: Vec<Object>,
}

impl Clone for AntArray {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            items: self.items.clone(),
        }
    }
}

impl IAntObject for AntArray {
    fn get_type(&self) -> ObjectType {
        ARRAY.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(
            self.items
                .iter()
                .map(|item| item.get_value())
                .collect::<Vec<Box<dyn Any>>>()
        )
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!(
            "<CompiledFunction {}>",
            self.items
                .iter()
                .map(|item| if item.get_type() != STRING {item.inspect()} else {format!("\"{}\"", item.inspect())})
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if let Some(arr) = other.as_any().downcast_ref::<AntArray>() {
            arr.items == self.items
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntArray);

impl From<Vec<Object>> for AntArray {
    fn from(items: Vec<Object>) -> Self {
        AntArray { 
            id: Uuid::new_v4(), 
            env: Environment::new(),
            items
        }
    }
}