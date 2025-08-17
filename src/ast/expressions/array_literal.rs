use crate::ast::ast::{Expression, Node};

use crate::impl_node;
use crate::token::token::Token;

impl Clone for ArrayLiteral {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            items: self.items.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ArrayLiteral {
    pub items: Vec<Box<dyn Expression>>,
    pub token: Token,
}

impl Node for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        let mut item_strings = vec![];

        for item in self.items.clone() {
            item_strings.push(item.to_string())
        }

        format!("[{}]", item_strings.join(", "))
    }
}

impl_node!(ArrayLiteral);

impl Expression for ArrayLiteral {}

pub fn create_array_literal(token: Token, items: Vec<Box<dyn Expression>>) -> ArrayLiteral {
    ArrayLiteral { token, items }
}
