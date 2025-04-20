use std::ops::Deref;
use crate::ast::ast::{Expression, Node};
use crate::environment::environment::Environment;
use crate::object::ant_error::AntError;
use crate::object::object::IAntObject;
use crate::token::token::Token;

impl Clone for Identifier {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone()
        }
    }
}

pub struct Identifier {
    value: String,
    token: Token,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn eval(&mut self, env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        let result = env.clone().get(self.value.deref().trim());

        Some(
            match result {
                None => {
                    AntError::new_with_native_value(Box::new(format!("identifier not found: \"{}\"", self.value.clone())))
                }
                Some(it) => {
                    it
                }
            }
        )
    }
}

impl Expression for Identifier {}

pub fn create_identifier(token: Token, value: String) -> Identifier {
    Identifier {
        token, value
    }
}