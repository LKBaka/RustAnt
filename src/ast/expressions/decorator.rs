use crate::ast::ast::{IExpression, INode};
use crate::ast::expr::Expression;
use crate::ast::stmt::Statement;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct Decorator {
    pub decorator: Box<Expression>,
    pub to_decorate: Statement,
    pub token: Token,
}

impl INode for Decorator {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("#[{}]\n{}", self.decorator.to_string(), self.to_decorate.to_string())
    }
}

impl IExpression for Decorator {}

pub fn create_decorator(
    token: Token,
    decorator: Box<Expression>,
    to_decorate: Statement,
) -> Decorator {
    Decorator { token, decorator, to_decorate }
}
