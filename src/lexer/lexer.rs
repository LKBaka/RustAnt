use unicode_properties::UnicodeEmoji;

use crate::constants::*;
use crate::token::token::Token;
use crate::token::token_type::{TOKEN_TYPE_MAP, TokenNumType, TokenType};

pub struct Lexer {
    code: String,
    file: String,
    errors: Vec<String>,
    cur_char: char,
    pos: usize,
    next_pos: usize,
    line: usize,
    column: usize,
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
            column: 1,
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
        match self.code_vec.get(self.next_pos) {
            Some(it) => it.clone(),
            None => NULL_CHAR,
        }
    }

    fn get_char(&self, pos: usize) -> char {
        match self.code_vec.get(pos) {
            Some(it) => it.clone(),
            None => NULL_CHAR,
        }
    }

    fn is_valid_char(&self, c: char) -> bool {
        (c.is_alphabetic() || c == '_' || c.is_emoji_char()) && c != '#' && c != '*'
    }

    fn read_char(&mut self) -> char {
        if self.next_pos < self.code_vec.len() {
            self.cur_char = self.code_vec[self.next_pos]
        } else {
            self.cur_char = NULL_CHAR;
        }

        if self.cur_char == NEW_LINE {
            self.line += 1;
            self.column = 0;
        }

        self.column += 1;

        self.pos = self.next_pos;
        self.next_pos += 1;

        self.cur_char
    }

    fn skip_whitespace(&mut self) {
        while self.cur_char == ' '
            || self.cur_char == '\t'
            || self.cur_char == '\n'
            || self.cur_char == '\r'
        {
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
            .map(|ch| ch.to_string())
            .collect::<Vec<String>>()
            .concat()
    }

    fn read_number(&mut self) -> TokenNumType {
        let start = self.pos;

        while self.cur_char.is_ascii_digit() {
            self.read_char();
        }

        let code = self.code_vec[start..self.pos]
            .iter()
            .map(|ch| ch.to_string())
            .collect::<Vec<String>>()
            .concat();

        if self.peek_char() == '6' && self.get_char(self.next_pos + 1) == '4' {
            self.read_char();
            self.read_char();
            self.read_char();

            TokenNumType::Int64(code)
        } else {
            TokenNumType::Big(code)
        }
    }

    fn read_string(&mut self) -> String {
        let start_line = self.line;
        let start_column = self.column;
        
        let mut result = String::new();

        // 跳过起始双引号
        self.read_char();

        loop {
            if self.cur_char == '"' {
                self.read_char(); // 跳过结束双引号
                return result;
            }

            if self.eof() {
                self.errors.push(format!(
                    "unclosed string. at file: <{}>, line {}, column {}",
                    self.file, start_line, start_column
                ));
                break;
            }

            if self.cur_char == '\\' {
                // 处理转义字符
                self.read_char();
                match self.cur_char {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    '0' => result.push('\0'),
                    'b' => result.push('\u{0008}'), // backspace
                    'f' => result.push('\u{000C}'), // form feed
                    'u' => {
                        // Unicode转义: \u{XXXX}
                        if self.peek_char() == '{' {
                            self.read_char(); // 跳过 {
                            let mut hex_digits = String::new();

                            loop {
                                self.read_char();
                                if self.cur_char == '}' {
                                    break;
                                }
                                if self.eof() || !self.cur_char.is_ascii_hexdigit() {
                                    self.errors.push(format!(
                                            "invalid unicode escape sequence. at file: <{}>, line {}, column {}",
                                            self.file, self.line, self.column
                                        ));
                                    return "".to_string();
                                }
                                hex_digits.push(self.cur_char);
                            }

                            match u32::from_str_radix(&hex_digits, 16) {
                                Ok(code_point) => match char::from_u32(code_point) {
                                    Some(ch) => result.push(ch),
                                    None => {
                                        self.errors.push(format!(
                                            "invalid unicode code point. at file: <{}>, line {}, column {}",
                                            self.file, self.line, self.column
                                        ));
                                        return "".to_string();
                                    }
                                },
                                Err(_) => {
                                    self.errors.push(format!(
                                        "invalid hex digits in unicode escape. at file: <{}>, line {}, column {}",
                                        self.file, self.line, self.column
                                    ));
                                    return "".to_string();
                                }
                            }
                        } else {
                            self.errors.push(format!(
                                "invalid unicode escape sequence, expected '{{' after '\\u'. at file: <{}>, line {}, column {}",
                                self.file, self.line, self.column
                            ));
                            return "".to_string();
                        }
                    }
                    _ => {
                        // 未知的转义序列，原样输出
                        result.push('\\');
                        result.push(self.cur_char);
                    }
                }
            } else {
                // 普通字符，直接添加
                result.push(self.cur_char);
            }

            self.read_char();
        }

        "".to_string()
    }

    fn read_comment(&mut self) -> String {
        let start = self.pos + 2; // 跳过 "//"

        loop {
            self.read_char();

            if self.cur_char == NEW_LINE || self.eof() {
                let s = self.code_vec[start..self.pos]
                    .iter()
                    .map(|ch| ch.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                return s;
            }
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut token = Token::new(
            TokenType::Illegal,
            self.cur_char.to_string(),
            self.file.clone(),
            self.line,
            self.column - 1,
        );

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
                    // 读取注释内容并跳过
                    self.read_comment();
                    // 递归调用 next_token 跳过注释，获取下一个有效token
                    return self.next_token();
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
                    match num {
                        TokenNumType::Big(num) => {
                            token.token_type = TokenType::IntegerBig;
                            token.value = num;
                        }

                        TokenNumType::Int64(num) => {
                            token.token_type = TokenType::Integer64;
                            token.value = num;
                        }
                    }

                    return token;
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
        println!(
            "lexer {}:",
            if self.errors.len() > 1 {
                "errors"
            } else {
                "error"
            }
        );

        for error in self.errors.clone() {
            println!("---> {}", error);
        }
    }
}

#[test]
fn test_lexer() {
    use crate::utils::assert_eq;

    let mut l = Lexer::new(
        "{let a = 1 + 2 * 3 / 4}".to_string(),
        String::from("__test_lexer__"),
    );
    let tokens = l.get_tokens();

    let on_failure_function = || println!("{:#?}", tokens);
    let expected_token_types = vec![
        TokenType::LBrace,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::IntegerBig,
        TokenType::Plus,
        TokenType::IntegerBig,
        TokenType::Asterisk,
        TokenType::IntegerBig,
        TokenType::Slash,
        TokenType::IntegerBig,
        TokenType::RBrace,
    ];

    // 验证词法单元
    for i in 0..tokens.len() - 1 {
        assert_eq(
            tokens[i].token_type,
            expected_token_types[i],
            on_failure_function,
        );
    }
}

#[test]
fn test_lexe_mul() {
    use crate::utils::assert_eq;

    let mut l = Lexer::new(
        "1*1".to_string(),
        String::from("__test_lexer__"),
    );
    let tokens = l.get_tokens();

    let on_failure_function = || println!("{:#?}", tokens);
    let expected_token_types = vec![
        TokenType::IntegerBig,
        TokenType::Asterisk,
        TokenType::IntegerBig,
    ];

    // 验证词法单元
    for i in 0..tokens.len() - 1 {
        assert_eq(
            tokens[i].token_type,
            expected_token_types[i],
            on_failure_function,
        );
    }
}

#[test]
fn test_lexer_unicode() {
    use crate::utils::assert_eq;

    let mut l = Lexer::new(
        "let ♿ = \"otto\"; let 你好 = \"Hello\"".to_string(),
        String::from("__test_lexer_unicode__"),
    );
    let tokens = l.get_tokens();

    let on_failure_function = || println!("{:#?}", tokens);
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
    for i in 0..tokens.len() - 1 {
        assert_eq(
            tokens[i].token_type,
            expected_token_types[i],
            on_failure_function,
        );
    }
}

#[test]
fn test_lexer_comment() {
    use crate::utils::assert_eq;

    let mut l = Lexer::new(
        "// test comment".into(),
        String::from("__test_lexer_comment__"),
    );
    let tokens = l.get_tokens();

    let on_failure_function = || println!("{:#?}", tokens);
    let expected_token_types = vec![TokenType::Comment];

    // 验证词法单元
    for i in 0..tokens.len() - 1 {
        assert_eq(
            tokens[i].token_type,
            expected_token_types[i],
            on_failure_function,
        );
    }
}

#[test]
fn test_lexer_test_print_token() {
    use crate::utils::assert_eq;

    let mut l = Lexer::new(
        "TestPrint n".into(),
        String::from("__test_lexer_test_print_token__"),
    );
    let tokens = l.get_tokens();

    let on_failure_function = || println!("{:#?}", tokens);
    let expected_token_types = vec![TokenType::TestPrint, TokenType::Ident];

    // 验证词法单元
    for i in 0..tokens.len() - 1 {
        assert_eq(
            tokens[i].token_type,
            expected_token_types[i],
            on_failure_function,
        );
    }
}
