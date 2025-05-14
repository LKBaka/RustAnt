use std::any::Any;

use crate::ast::ast::{Expression, Node};
use crate::ast::expressions::identifier::Identifier;
use crate::environment::environment::Environment;
use crate::object::object::IAntObject;
use crate::evaluator::evaluator::Evaluator;
use crate::object::utils::{create_error, is_error};
use crate::token::token::Token;

impl Clone for AssignmentExpression {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            value: self.value.clone(),
            token: self.token.clone()
        }
    }
}

pub struct AssignmentExpression {
    pub left: Box<dyn Expression + 'static>,
    pub value: Box<dyn Expression>,
    pub token: Token,
}

impl Node for AssignmentExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("{} = {}", self.left.to_string(), self.value.to_string())
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        if let Some(it) = (self.left.to_owned() as Box<dyn Any>).downcast_ref::<Identifier>() {
            let name = it.to_string();

            let value =  self.value.eval(evaluator, env).expect("value is None!");
            if is_error(&value) {return Some(value)};

            let result = env.set_value(&name, value);
            if let Some(err_obj) = result {
                return Some(err_obj)
            }

            return None;
        }

        // 不支持赋值的表达式，抛出错误
        Some(
            create_error(
                format!(
                    "cannot assign to expression \"{}\" here. Maybe you meant '==' instead of '='?", 
                    self.left.to_string()
                )
            )
        )
    }
}

impl Expression for AssignmentExpression {}

pub fn create_assignment_expression(token: Token, left: Box<dyn Expression>, value: Box<dyn Expression>) -> AssignmentExpression {
    AssignmentExpression {
        token, left, value
    }
}