use std::fmt::Debug;

use crate::constants::NULL_CHAR;
use crate::token::token_type::TokenType;

#[derive(Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub file: String,
    pub line: i64,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, file: String, line: i64) -> Token {
        Token {
            token_type,
            value,
            line,
            file,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "value: {}, type: {:?}, file: {}, line: {}",
            self.value, self.token_type, self.file, self.line
        )
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

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("type", &self.token_type) // 使用 token_type 的 Debug 实现
            .field("value", &self.value) // 字符串直接显示内容
            .field("file", &self.file) // 文件名保留完整路径
            .field("line", &self.line) // 行号直接显示数值
            .finish()
    }
}
