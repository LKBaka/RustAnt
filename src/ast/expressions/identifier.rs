use crate::ast::ast::{Expression, Node};
use crate::environment::environment::Environment;
use crate::object::object::Object;
use crate::object::utils::create_error_with_name;
use crate::token::token::Token;
use crate::evaluator::evaluator::Evaluator;

impl Clone for Identifier {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone()
        }
    }
}

pub struct Identifier {
    pub value: String,
    pub token: Token,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn eval(&mut self, _: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        let result = env.get(&self.value.trim());

        Some(
            match result {
                None => {
                    create_error_with_name(
                        "NameError", 
                        format!("name '{}' is not defined ", self.value.clone())
                    )
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