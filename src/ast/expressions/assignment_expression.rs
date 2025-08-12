
use crate::ast::ast::{Expression, Node};

use crate::token::token::Token;
use crate::impl_node;


impl Clone for AssignmentExpression {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            value: self.value.clone(),
            token: self.token.clone()
        }
    }
}

#[derive(Debug)]
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
}

impl_node!(AssignmentExpression);

impl Expression for AssignmentExpression {}

pub fn create_assignment_expression(token: Token, left: Box<dyn Expression>, value: Box<dyn Expression>) -> AssignmentExpression {
    AssignmentExpression {
        token, left, value
    }
}