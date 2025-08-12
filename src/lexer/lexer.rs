
use unicode_properties::UnicodeEmoji;

use crate::constants::*;
use crate::token::token::Token;
use crate::token::token_type::{TokenType, TOKEN_TYPE_MAP};

pub struct Lexer {
    code: String,
    file: String,
    errors: Vec<String>,
    cur_char: char,
    pos: usize,
    next_pos: usize,
    line: i64,
    code_vec: Vec<char>,
}

impl Lexer {
    pub fn new(code: String, file: String) -> Lexer {
        let mut lexer = Lexer {
            code,
            file,
            errors: vec![],
            cur_char: NULL_CHAR,
            pos: 0,
            next_pos: 0,
            line: 1,
            code_vec: vec![],
        };

        lexer.code_vec = lexer.code.chars().collect();

        lexer.read_char(); // 初始化
        lexer
    }

    fn get_ident_token_type(&self, ident: &str) -> TokenType {
        *TOKEN_TYPE_MAP
            .get(&ident.to_uppercase())
            .unwrap_or(&TokenType::Ident)
    }

    fn peek_char(&self) -> char {
        self.code_vec[self.next_pos]
    }

    fn is_valid_char(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_' || c.is_emoji_char()
    }

    fn read_char(&mut self) -> char {
        if self.next_pos < self.code_vec.len() {
            self.cur_char = self.code_vec[self.next_pos]
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

        while self.is_valid_char(self.cur_char) && !self.eof() {
            self.read_char();
        }

        self.code_vec[start..self.pos]
            .iter()
            .map(|ch| {ch.to_string()})
            .collect::<Vec<String>>()
            .concat()
    }

    fn read_number(&mut self) -> String {
        let start = self.pos;

        while self.cur_char.is_ascii_digit() {
            self.read_char();
        }

        self.code_vec[start..self.pos]
            .iter()
            .map(|ch| {ch.to_string()})
            .collect::<Vec<String>>()
            .concat()
    }

    fn read_string(&mut self) -> String {
        let start = self.pos + 1;
        let start_line = self.line;

        loop {
            self.read_char();

            if self.cur_char == '"'{
                let s = self.code_vec[start..self.pos]
                    .iter()
                    .map(|ch| {ch.to_string()})
                    .collect::<Vec<String>>()
                    .join("");

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

    fn read_comment(&mut self) -> String {
        let start = self.pos + 1;
        let start_line = self.line;

        loop {
            self.read_char();

            if self.cur_char == NEW_LINE{
                let s = self.code_vec[start..self.pos]
                    .iter()
                    .map(|ch| {ch.to_string()})
                    .collect::<Vec<String>>()
                    .join("");

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
        }

        match self.cur_char {
            '=' => {
                let peek_char = self.peek_char();

                if peek_char == '=' {
                    token.token_type = TokenType::Eq;
                    token.value = format!("{}{}", self.cur_char, peek_char);
                
                    self.read_char();
                }
            }

            '!' => {
                let peek_char = self.peek_char();
                if peek_char == '=' {
                    token.token_type = TokenType::Eq;
                    token.value = format!("{}{}", self.cur_char, peek_char);

                    self.read_char();
                }
            }

            ':' => {
                let peek_char = self.peek_char();
                if peek_char == ':' {
                    token.token_type = TokenType::GetClassMember;
                    token.value = format!("{}{}", self.cur_char, peek_char);

                    self.read_char();
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

            '/' => {
                let peek_char = self.peek_char();
                if peek_char == '/' {
                    token.token_type = TokenType::Comment;
                    token.value = format!("//{}", self.read_comment());

                    self.read_char();
                }
            }

            _ => {
                if self.is_valid_char(self.cur_char) && !self.cur_char.is_ascii_digit() {
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

        self.read_char();

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

#[test]
fn test_lexer_unicode() {
    use crate::token::utils::print_tokens;
    use crate::utils::assert_eq;

    let mut l = Lexer::new(
        "let ♿ = \"otto\"; let 你好 = \"Hello\"".to_string(), 
        String::from("__test_lexer_unicode__")
    );
    let tokens = l.get_tokens();

    let on_failure_function = || print_tokens(tokens.clone());
    let expected_token_types = vec![
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::String,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::String,
    ];
    
    // 验证词法单元
    for i in 0 .. tokens.len() - 1 {
        assert_eq(tokens[i].token_type, expected_token_types[i], on_failure_function);
    }
}

#[test]
fn test_lexer_comment() {
    use crate::token::utils::print_tokens;
    use crate::utils::assert_eq;

    let mut l = Lexer::new(
        "// test comment".into(),
        String::from("__test_lexer_comment__")
    );
    let tokens = l.get_tokens();

    let on_failure_function = || print_tokens(tokens.clone());
    let expected_token_types = vec![
        TokenType::Comment
    ];
    
    // 验证词法单元
    for i in 0 .. tokens.len() - 1 {
        assert_eq(tokens[i].token_type, expected_token_types[i], on_failure_function);
    }
}