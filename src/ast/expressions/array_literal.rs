use crate::ast::ast::{IExpression, INode};

use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub items: Vec<Box<Expression>>,
    pub token: Token,
}

impl INode for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        let mut item_strings = vec![];

        for item in self.items.clone() {
            item_strings.push(item.to_string())
        }

        format!("[{}]", item_strings.join(", "))
    }
}

impl IExpression for ArrayLiteral {}

pub fn create_array_literal(token: Token, items: Vec<Box<Expression>>) -> ArrayLiteral {
    ArrayLiteral { token, items }
}
