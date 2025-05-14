use num_bigint::BigInt;

use crate::ast::ast::{Expression, Node};
use crate::environment::environment::Environment;
use crate::object::ant_int::AntInt;
use crate::object::object::IAntObject;
use crate::evaluator::evaluator::Evaluator;
use crate::token::token::Token;

impl Clone for IntegerLiteral {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone()
        }
    }
}

pub struct IntegerLiteral {
    pub value: BigInt,
    pub token: Token,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn eval(&mut self, _: &mut Evaluator, _: &mut Environment) -> Option<Box<dyn IAntObject>> { 
        Some(AntInt::new_with_native_value(Box::new(self.value.to_owned())))
    }
}

impl Expression for IntegerLiteral {}

pub fn create_integer_literal(token: Token, value: BigInt) -> IntegerLiteral {
    IntegerLiteral {
        token, value
    }
}