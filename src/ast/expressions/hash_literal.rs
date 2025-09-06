use crate::ast::ast::{Expression, Node};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct HashLiteral {
    pub pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
    pub token: Token,
}

impl Node for HashLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
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

impl Expression for HashLiteral {}

pub fn create_hash_literal(token: Token, pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>) -> HashLiteral {
    HashLiteral { token, pairs }
}
