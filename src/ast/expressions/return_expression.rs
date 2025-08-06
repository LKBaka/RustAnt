use std::any::Any;

use crate::ast::ast::{Expression, Node};
use crate::environment::environment::Environment;
use crate::object::ant_return_value::AntReturnValue;
use crate::object::object::Object;
use crate::token::token::Token;
use crate::evaluator::evaluator::Evaluator;
use crate::impl_node;

use super::call_expression::CallExpression;

impl Clone for ReturnExpression {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone()
        }
    }
}

#[derive(Debug)]
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

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        let return_value = self.value.eval(evaluator, env);

        if let Some(_it) = (&self.value as &dyn Any).downcast_ref::<CallExpression>() {
            env.drop_all();
        }

        if let Some(it) = return_value {
            Some(AntReturnValue::new_with_native_value(Box::new(it.clone())))
        } else {None}
    }
}

impl Expression for ReturnExpression {}

impl_node!(ReturnExpression);

pub fn create_return_expression(token: Token, value: Box<dyn Expression>) -> ReturnExpression {
    ReturnExpression {
        token, value
    }
}