use crate::ast::ast::{Expression, Node};

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: String,
    pub token: Token,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        )
    }
}

impl Expression for InfixExpression {}

pub fn create_infix_expression(
    token: Token,
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    operator: String,
) -> InfixExpression {
    InfixExpression {
        token,
        left,
        right,
        operator,
    }
}
