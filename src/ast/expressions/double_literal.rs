use bigdecimal::BigDecimal;

use crate::ast::ast::{Expression, Node};
use crate::environment::environment::Environment;
use crate::object::ant_double::AntDouble;
use crate::object::object::Object;
use crate::evaluator::evaluator::Evaluator;
use crate::token::token::Token;

impl Clone for DoubleLiteral {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone()
        }
    }
}

pub struct DoubleLiteral {
    pub value: BigDecimal,
    pub token: Token,
}

impl Node for DoubleLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn eval(&mut self, _: &mut Evaluator, _: &mut Environment) -> Option<Object> {        
        Some(AntDouble::new_with_native_value(Box::new(self.value.clone())))
    }
}

impl Expression for DoubleLiteral {}

pub fn create_double_literal(token: Token, value: BigDecimal) -> DoubleLiteral {
    DoubleLiteral {
        token, value
    }
}