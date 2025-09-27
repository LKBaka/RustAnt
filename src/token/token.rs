use std::fmt::{Debug, Display};

use crate::constants::NULL_CHAR;
use crate::token::token_type::TokenType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, file: String, line: usize, column: usize) -> Token {
        Token {
            token_type,
            value,
            line,
            column,
            file,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "value: {}, type: {:?}, file: {}, line: {}",
            self.value, self.token_type, self.file, self.line
        )
    }

    pub fn eof(file: String, line: usize, column: usize) -> Token {
        Token {
            token_type: TokenType::Eof,
            value: NULL_CHAR.to_string(),
            file,
            line,
            column,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}