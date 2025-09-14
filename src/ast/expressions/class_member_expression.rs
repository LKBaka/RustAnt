use crate::ast::ast::{IExpression, INode};

use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct ClassMemberExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub token: Token,
}

impl INode for ClassMemberExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("({}::{})", self.left.to_string(), self.right.to_string())
    }
}

impl IExpression for ClassMemberExpression {}

pub fn create_class_member_expression(
    token: Token,
    left: Box<Expression>,
    right: Box<Expression>,
) -> ClassMemberExpression {
    ClassMemberExpression { token, left, right }
}
