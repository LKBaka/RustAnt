
use crate::ast::ast::{Expression, Node};

use crate::token::token::Token;
use crate::impl_node;


impl Clone for ClassMemberExpression {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            token: self.token.clone()
        }
    }
}

#[derive(Debug)]
pub struct ClassMemberExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub token: Token,
}

impl Node for ClassMemberExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("({}::{})", self.left.to_string(), self.right.to_string())
    }
}

impl Expression for ClassMemberExpression {}

impl_node!(ClassMemberExpression);

pub fn create_class_member_expression(token: Token, left: Box<dyn Expression>, right: Box<dyn Expression>) -> ClassMemberExpression {
    ClassMemberExpression {
        token, left, right
    }
}