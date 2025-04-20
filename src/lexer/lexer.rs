use crate::char_string::char_string::CharString;
use crate::constants::*;
use crate::token::token::Token;
use crate::token::token_type::{TokenType, TOKEN_TYPE_MAP};

pub struct Lexer {
    code: CharString,
    file: String,
    errors: Vec<String>,
    cur_char: char,
    pos: usize,
    next_pos: usize,
    line: i64,
}

impl Lexer {
    pub fn new(code: String, file: String) -> Lexer {
        let mut lexer = Lexer {
            code: CharString::from(code),
            file,
            errors: vec![],
            cur_char: NULL_CHAR,
            pos: 0,
            next_pos: 0,
            line: 1,
        };

        lexer.read_char(); // 初始化
        lexer
    }

    fn get_ident_token_type(&self, ident: &str) -> TokenType {
        TOKEN_TYPE_MAP
            .get(&ident.to_uppercase())
            .cloned()
            .unwrap_or(TokenType::Ident)
    }

    fn peek_char(&self) -> char {
        self.code.chars().nth(self.next_pos).unwrap_or(NULL_CHAR)
    }

    fn is_valid_char(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn read_char(&mut self) -> char {
        if self.next_pos < self.code.len() {
            self.cur_char = self.code.get(self.next_pos).unwrap()
        } else {
            self.cur_char = NULL_CHAR;
        }

        if self.cur_char == NEW_LINE {
            self.line += 1;
        }

        self.pos = self.next_pos;
        self.next_pos += 1;
        self.cur_char
    }

    fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\n' || self.cur_char == '\r' {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let start = self.pos;
        while self.is_valid_char(self.cur_char) {
            self.read_char();
        }

        self.code.slice(start..self.pos)
    }

    fn read_number(&mut self) -> String {
        let start = self.pos;

        while self.cur_char.is_ascii_digit() {
            self.read_char();
        }

        self.code.slice(start..self.pos)
    }

    fn read_string(&mut self) -> String {
        let start = self.pos + 1;
        let start_line = self.line;

        loop {
            self.read_char();

            if self.cur_char == '"'{
                let s = self.code.slice(start..self.pos);

                self.read_char();
                return s
            }

            if self.eof() {
                self.errors.push(format!("unclosed string. file: <{}>. line: {}.", self.file, start_line));
                break
            }
        }

        "".to_string()
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut token = Token::new(TokenType::Illegal, self.cur_char.to_string(), self.file.clone(), self.line.clone());

        if TOKEN_TYPE_MAP.contains_key(&self.cur_char.to_string()) {
            token.token_type = TOKEN_TYPE_MAP[&self.cur_char.to_string()].clone();
            token.value = self.cur_char.to_string();

            self.read_char(); // 处理完符号立即推进指针
            return token; // 立即返回避免后续处理
        }

        match self.cur_char {
            '=' => {
                let peek_char = self.peek_char();
                if peek_char == '=' {
                    token.token_type = TokenType::Eq;
                    token.value = format!("{}, {}", self.cur_char, peek_char)
                }
            }

            '!' => {
                let peek_char = self.peek_char();
                if peek_char == '=' {
                    token.token_type = TokenType::Eq;
                    token.value = format!("{}, {}", self.cur_char, peek_char)
                }
            }

            '"' => {
                if self.cur_char == '"' {
                    let s = self.read_string();
                    token.value = s;
                    token.token_type = TokenType::String;

                    return token;
                }
            }

            _ => {
                if self.cur_char.is_ascii_alphabetic() {
                    let ident = self.read_ident();
                    token.token_type = self.get_ident_token_type(&ident);
                    token.value = ident;

                    return token;
                } else if self.cur_char.is_ascii_digit() {
                    let num = self.read_number();
                    token.token_type = TokenType::Integer;
                    token.value = num;

                    return token
                }
            }
        }

        if token.token_type != TokenType::Illegal {
            self.read_char();
        }

        token
    }

    fn eof(&self) -> bool {
        self.cur_char == NULL_CHAR
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.eof() {
            tokens.push(self.next_token());
        }
        tokens
    }

    pub fn contains_error(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn print_errors(&self) {
        println!("lexer {}:", if self.errors.len() > 1 {"errors"} else {"error"} );

        for error in self.errors.clone() {
            println!("---> {}", error);
        }
    }
}

#[test]
fn test_lexer() {
    use crate::token::utils::print_tokens;
    use crate::utils::assert_eq;

    let mut l = Lexer::new("{let a = 1 + 2 * 3 / 4}".to_string(), String::from("__test_lexer__"));
    let tokens = l.get_tokens();

    let on_failure_function = || print_tokens(tokens.clone());
    let expected_token_types = vec![
        TokenType::LBrace,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Integer,
        TokenType::Plus,
        TokenType::Integer,
        TokenType::Asterisk,
        TokenType::Integer,
        TokenType::Slash,
        TokenType::Integer,
        TokenType::RBrace,
    ];
    
    // 验证词法单元
    for i in 0 .. tokens.len() - 1 {
        assert_eq(tokens[i].token_type, expected_token_types[i], on_failure_function);
    }
}