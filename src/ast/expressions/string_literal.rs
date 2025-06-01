use crate::ast::ast::{Expression, Node};
use crate::environment::environment::Environment;
use crate::object::ant_string::AntString;
use crate::object::object::Object;
use crate::evaluator::evaluator::Evaluator;
use crate::token::token::Token;

impl Clone for StringLiteral {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone()
        }
    }
}

pub struct StringLiteral {
    pub value: String,
    pub token: Token,
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn eval(&mut self, _: &mut Evaluator, _: &mut Environment) -> Option<Object> {
        Some(AntString::new_with_native_value(Box::new(self.value.to_owned())))
    }
}

impl Expression for StringLiteral {}

pub fn create_string_literal(token: Token, value: String) -> StringLiteral {
    StringLiteral {
        token, value
    }
}