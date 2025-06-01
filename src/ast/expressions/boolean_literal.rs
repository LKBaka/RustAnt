use crate::ast::ast::{Expression, Node};
use crate::constants::{ant_false, ant_true};
use crate::environment::environment::Environment;
use crate::evaluator::evaluator::Evaluator;
use crate::object::object::Object;
use crate::token::token::Token;

impl Clone for BooleanLiteral {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone()
        }
    }
}

pub struct BooleanLiteral {
    pub value: bool,
    pub token: Token,
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn eval(&mut self, _: &mut Evaluator, _: &mut Environment) -> Option<Object> {
        Some(
            if self.value {
                ant_true.clone()
            } else {
                ant_false.clone()
            }
        )
    }
}

impl Expression for BooleanLiteral {}

pub fn create_boolean_literal(token: Token, value: bool) -> BooleanLiteral {
    BooleanLiteral {
        token, value
    }
}