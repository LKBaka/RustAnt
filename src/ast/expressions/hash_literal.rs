use crate::ast::ast::{IExpression, INode};
use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct HashLiteral {
    pub pairs: Vec<(Box<Expression>, Box<Expression>)>,
    pub token: Token,
}

impl INode for HashLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        let pair_string = self.pairs
            .iter()
            .map(
                |(k, v)| 
                format!("{}: {}", k.to_string(), v.to_string()
            )).collect::<Vec<String>>()
            .join(", ");

        format!("{{{}}}", pair_string)
    }
}

impl IExpression for HashLiteral {}

pub fn create_hash_literal(token: Token, pairs: Vec<(Box<Expression>, Box<Expression>)>) -> HashLiteral {
    HashLiteral { token, pairs }
}
