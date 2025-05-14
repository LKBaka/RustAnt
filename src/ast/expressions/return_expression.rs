use std::any::Any;

use crate::ast::ast::{Expression, Node};
use crate::environment::environment::Environment;
use crate::object::ant_return_value::AntReturnValue;
use crate::object::object::IAntObject;
use crate::token::token::Token;
use crate::evaluator::evaluator::Evaluator;

use super::call_expression::CallExpression;

impl Clone for ReturnExpression {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone()
        }
    }
}

pub struct ReturnExpression {
    value: Box<dyn Expression>,
    token: Token,
}

impl Node for ReturnExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        let return_value = evaluator.eval_box(self.value.to_owned(), env);

        if let Some(_it) = (self.value.to_owned() as Box<dyn Any>).downcast_ref::<CallExpression>() {
            env.drop_all();
        }

        if let Some(it) = return_value {
            Some(AntReturnValue::new_with_native_value(Box::new(it.to_owned())))
        } else {None}
    }
}

impl Expression for ReturnExpression {}

pub fn create_return_expression(token: Token, value: Box<dyn Expression>) -> ReturnExpression {
    ReturnExpression {
        token, value
    }
}