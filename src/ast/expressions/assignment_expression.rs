use crate::ast::ast::{IExpression, INode};

use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub left: Box<Expression>,
    pub value: Box<Expression>,
    pub token: Token,
}

impl INode for AssignmentExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        format!("{} = {}", self.left.to_string(), self.value.to_string())
    }
}

impl IExpression for AssignmentExpression {}

pub fn create_assignment_expression(
    token: Token,
    left: Box<Expression>,
    value: Box<Expression>,
) -> AssignmentExpression {
    AssignmentExpression { token, left, value }
}
