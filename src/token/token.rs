use crate::constants::NULL_CHAR;
use crate::token::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub file: String,
    pub line: i64,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, file: String, line: i64) -> Token {
        Token{
            token_type, value, line, file
        }
    }

    pub fn to_string(&self) -> String {
        format!("value: {}, type: {:?}, file: {}, line: {}", self.value, self.token_type, self.file, self.line)
    }

    pub fn eof(file: String, line: i64) -> Token {
        Token {
            token_type: TokenType::Eof,
            value: NULL_CHAR.to_string(),
            file,
            line,
        }
    }
}