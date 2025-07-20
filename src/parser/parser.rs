use std::collections::HashMap;

use crate::ast::ast::{Expression, ExpressionStatement, Program, Statement};
use crate::constants::NULL_CHAR;
use crate::parser::parse_functions::parse_assignment_expression::parse_assignment_expression;
use crate::parser::parse_functions::parse_boolean::parse_boolean;
use crate::parser::parse_functions::parse_call_expression::parse_call_expression;
use crate::parser::parse_functions::parse_class_member_expression::parse_class_member_expression;
use crate::parser::precedence::*;
use crate::token::token::Token;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::{Comma, Eof, Nonsense, Semicolon};

use crate::parser::parse_functions::parse_ident::parse_ident;
use crate::parser::parse_functions::parse_infix_expression::parse_infix_expression;
use crate::parser::parse_functions::parse_let_statement::parse_let_statement;
use crate::parser::parse_functions::parse_number::parse_number;
use crate::parser::parse_functions::parse_string::parse_string;
use crate::parser::parse_functions::parse_if_expression::parse_if_expression;
use crate::parser::parse_functions::parse_function_expression::parse_function_expression;
use crate::parser::precedence::Precedence::Lowest;

use super::parse_functions::parse_class_statement::parse_class_statement;
use super::parse_functions::parse_object_member_expression::parse_object_member_expression;
use super::parse_functions::parse_return_expression::parse_return_expression;
use super::parse_functions::parse_while_statement::parse_while_statement;

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;
type StmtParseFn = fn(&mut Parser) -> Option<Box<dyn Statement>>;


pub struct Parser {
    tokens: Vec<Token>,
    pub errors: Vec<String>,

    pos: usize,
    next_pos: usize,

    pub cur_token: Token,
    pub peek_token: Token,

    prefix_parse_fn_map: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fn_map: HashMap<TokenType, InfixParseFn>,
    statement_parse_fn_map: HashMap<TokenType, StmtParseFn>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            errors: vec![],
            pos: 0,
            next_pos: 0,
            cur_token: Token::new(Nonsense, NULL_CHAR.to_string(), "<uninit_parser>".to_string(), -1),
            peek_token: Token::new(Nonsense, NULL_CHAR.to_string(), "<uninit_parser>".to_string(), -1),
            prefix_parse_fn_map: HashMap::new(),
            infix_parse_fn_map: HashMap::new(),
            statement_parse_fn_map: HashMap::new(),
        };

        parser.statement_parse_fn_map.insert(TokenType::Class, parse_class_statement);
        parser.statement_parse_fn_map.insert(TokenType::Let, parse_let_statement);
        parser.statement_parse_fn_map.insert(TokenType::While, parse_while_statement);

        parser.prefix_parse_fn_map.insert(TokenType::Ident, parse_ident);
        parser.prefix_parse_fn_map.insert(TokenType::Integer, parse_number);
        parser.prefix_parse_fn_map.insert(TokenType::BoolTrue, parse_boolean);
        parser.prefix_parse_fn_map.insert(TokenType::BoolFalse, parse_boolean);
        parser.prefix_parse_fn_map.insert(TokenType::String, parse_string);
        parser.prefix_parse_fn_map.insert(TokenType::Return, parse_return_expression);
        parser.prefix_parse_fn_map.insert(TokenType::If, parse_if_expression);
        parser.prefix_parse_fn_map.insert(TokenType::Func, parse_function_expression);

        parser.infix_parse_fn_map.insert(TokenType::LParen, parse_call_expression);
        parser.infix_parse_fn_map.insert(TokenType::Assign, parse_assignment_expression);

        parser.infix_parse_fn_map.insert(TokenType::Dot, parse_object_member_expression);
        parser.infix_parse_fn_map.insert(TokenType::GetClassMember, parse_class_member_expression);
        parser.infix_parse_fn_map.insert(TokenType::Plus, parse_infix_expression);
        parser.infix_parse_fn_map.insert(TokenType::Asterisk, parse_infix_expression);
        parser.infix_parse_fn_map.insert(TokenType::Minus, parse_infix_expression);
        parser.infix_parse_fn_map.insert(TokenType::Slash, parse_infix_expression);
        parser.infix_parse_fn_map.insert(TokenType::Lt, parse_infix_expression);
        parser.infix_parse_fn_map.insert(TokenType::Gt, parse_infix_expression);
        parser.infix_parse_fn_map.insert(TokenType::Eq, parse_infix_expression);
        parser.infix_parse_fn_map.insert(TokenType::NotEq, parse_infix_expression);

        parser.next_token(); // 初始化当前词法单元

        parser
    }

    pub fn parse_expression_list(&mut self, end: TokenType) -> Vec<Box<dyn Expression>> {
        // 检查下一个词法单元是否为对应结束的词法单元
        if self.peek_token_is(end) {
            self.next_token();
            return vec![]; // 如果是，直接退出，跳过表达式解析
        }

        self.next_token(); // 前进到表达式

        let mut expressions = vec![];
        let expr = self.parse_expression(Lowest);
        if let Some(it) = expr {
            expressions.push(it)
        }

        while self.peek_token_is(Comma) {
            self.next_token(); // 离开表达式
            self.next_token(); // 离开逗号

            let expression = self.parse_expression(Lowest);
            if let Some(it) = expression {
                expressions.push(it)
            } 
        }

        self.next_token(); // 前进到结束的词法单元

        // WARNING: 若想在调用后跳过结束的词法单元，请自行在使用后处理

        expressions
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let mut left: Box<dyn Expression>;
        
        if self.prefix_parse_fn_map.contains_key(&self.cur_token.token_type) {
            left = self.prefix_parse_fn_map[&self.cur_token.token_type](self)?
        } else {
            self.errors.push(
                format!(
                    "no prefix parse function for {} found. at file <{}> line {}",
                    self.cur_token.token_type.to_string(), self.cur_token.file, self.cur_token.line
                )
            );
            return None
        }

        while
            (!self.peek_token_is(Semicolon)) &&
            get_token_precedence(self.peek_token.token_type) > precedence
        {
            let infix_parse_fn = self.infix_parse_fn_map.get(&self.peek_token.token_type);
            match infix_parse_fn.cloned() {
                None => {
                    self.errors.push(
                        format!(
                            "no infix parse function for {} found. at file <{}> line {}",
                            self.cur_token.token_type.to_string(), self.cur_token.file, self.cur_token.line
                        )
                    );
                    
                    return None
                }
                Some(it) => {
                    self.next_token();
                    left = it(self, left)?
                }
            }
        }

        Some(left)
    }

    fn parse_expression_statement(&mut self) -> Box<dyn Statement> {
        let expression_statement = ExpressionStatement {
            expression: self.parse_expression(Precedence::Lowest)
        };

        if self.peek_token_is(Semicolon) {
            self.next_token();
        }
        
        Box::new(expression_statement)
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        if self.statement_parse_fn_map.contains_key(&self.cur_token.token_type) {
            let stmt = self.statement_parse_fn_map[&self.cur_token.token_type](self);

            return stmt;
        }

        Some(self.parse_expression_statement())
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            token: if !self.tokens.is_empty() {
                self.tokens[0].clone()
            } else {
                Token::new(Nonsense, NULL_CHAR.to_string(), "<uninit_parser>".to_string(), -1)
            },
            statements: vec![]
        };

        while !self.cur_token_is(Eof) {
            let statement = self.parse_statement();
            if statement.is_none() {
                self.next_token();
                continue
            }

            program.statements.push(statement.unwrap());

            self.next_token()
        }

        program
    }

    pub fn next_token(&mut self) {
        if self.next_pos < self.tokens.len() {
            self.pos = self.next_pos;
            self.next_pos += 1;

            self.cur_token = self.tokens[self.pos].clone();
            
            self.peek_token = if self.next_pos < self.tokens.len() {
                self.tokens[self.next_pos].clone()
            } else {
                Token::eof(self.cur_token.file.clone(), self.cur_token.line.clone())
            };
            
        } else {
            self.cur_token = Token::eof(self.cur_token.file.clone(), self.cur_token.line.clone());
            self.peek_token = Token::eof(self.peek_token.file.clone(), self.peek_token.line.clone());
        }
    }

    pub fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    pub fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token.token_type != token_type {
            self.errors.push(
                format!(
                    "missing {}. at file <{}>, line {}",
                    token_type.to_string(),
                    self.cur_token.file, self.cur_token.line
                )
            );
            return false
        } 

        self.peek_token.token_type == token_type
    }

    pub fn expect_cur(&mut self, token_type: TokenType) -> bool {
        if self.cur_token.token_type != token_type {
            self.errors.push(
                format!(
                    "missing {}. at file <{}>, line {}",
                    token_type.to_string(),
                    self.cur_token.file, self.cur_token.line
                )
            );

            return false
        }

        self.cur_token.token_type == token_type
    }

    pub fn contains_error(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn print_errors(&self) {
        println!("parser {}:", if self.errors.len() > 1 {"errors"} else {"error"} );

        for error in &self.errors {
            println!("---> {}", error);
        }
    }
}