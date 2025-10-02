use std::fmt::Display;

use hashbrown::HashMap;

use crate::ast::ast::{ExpressionStatement, Program};
use crate::ast::stmt::Statement;
use crate::constants::NULL_CHAR;
use crate::parser::parse_functions::parse_array_literal::parse_array_literal;
use crate::parser::parse_functions::parse_assignment_expression::parse_assignment_expression;
use crate::parser::parse_functions::parse_boolean::parse_boolean;
use crate::parser::parse_functions::parse_break::parse_break;
use crate::parser::parse_functions::parse_call_expression::parse_call_expression;
use crate::parser::parse_functions::parse_class_member_expression::parse_class_member_expression;
use crate::parser::parse_functions::parse_continue::parse_continue;
use crate::parser::parse_functions::parse_decorator::parse_decorator;
use crate::parser::parse_functions::parse_hash_literal::parse_hash_literal;
use crate::parser::parse_functions::parse_index_expression::parse_index_expression;
use crate::parser::parse_functions::parse_none::parse_none;
use crate::parser::parse_functions::parse_prefix_expression::parse_prefix_expression;
use crate::parser::parse_functions::parse_return_statement::parse_return_statement;
use crate::parser::parse_functions::parse_test_print_expression::parse_test_print_expression;
use crate::parser::parse_functions::parse_tuple_expression::parse_tuple_expression;
use crate::parser::parse_functions::parse_use_statement::parse_use_statement;
use crate::parser::precedence::*;
use crate::token::token::Token;
use crate::ast::expr::Expression;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::{Comma, Eof, Nonsense, Semicolon};

use crate::parser::parse_functions::parse_function_expression::parse_function_expression;
use crate::parser::parse_functions::parse_ident::parse_ident;
use crate::parser::parse_functions::parse_if_expression::parse_if_expression;
use crate::parser::parse_functions::parse_infix_expression::parse_infix_expression;
use crate::parser::parse_functions::parse_let_statement::parse_let_statement;
use crate::parser::parse_functions::parse_number::{parse_number, parse_number_i64};
use crate::parser::parse_functions::parse_string::parse_string;
use crate::parser::precedence::Precedence::Lowest;

use super::parse_functions::parse_class_statement::parse_class_statement;
use super::parse_functions::parse_object_member_expression::parse_object_member_expression;
use super::parse_functions::parse_while_statement::parse_while_statement;

type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;
type StmtParseFn = fn(&mut Parser) -> Option<Statement>;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub token: Token,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}\n    (at line: {}, at column: {}, at file: {})",
            self.message, self.token.line, self.token.column, self.token.file
        )
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    errors: Vec<ParseError>,

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
            cur_token: Token::new(
                Nonsense,
                NULL_CHAR.to_string(),
                "<uninit_parser>".to_string(),
                0,
                0,
            ),
            peek_token: Token::new(
                Nonsense,
                NULL_CHAR.to_string(),
                "<uninit_parser>".to_string(),
                0,
                0,
            ),
            prefix_parse_fn_map: HashMap::with_capacity(12),
            infix_parse_fn_map: HashMap::with_capacity(12),
            statement_parse_fn_map: HashMap::with_capacity(5),
        };

        parser
            .statement_parse_fn_map
            .insert(TokenType::Class, parse_class_statement);
        parser
            .statement_parse_fn_map
            .insert(TokenType::Let, parse_let_statement);
        parser
            .statement_parse_fn_map
            .insert(TokenType::While, parse_while_statement);
        parser
            .statement_parse_fn_map
            .insert(TokenType::Use, parse_use_statement);
        parser
            .statement_parse_fn_map
            .insert(TokenType::Continue, parse_continue);
        parser
            .statement_parse_fn_map
            .insert(TokenType::Break, parse_break);
        parser
            .statement_parse_fn_map
            .insert(TokenType::Return, parse_return_statement);

        parser
            .prefix_parse_fn_map
            .insert(TokenType::Ident, parse_ident);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::IntegerBig, parse_number);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::Integer64, parse_number_i64);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::BoolTrue, parse_boolean);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::BoolFalse, parse_boolean);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::String, parse_string);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::If, parse_if_expression);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::Func, parse_function_expression);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::LParen, parse_tuple_expression);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::LBracket, parse_array_literal);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::Bang, parse_prefix_expression);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::Minus, parse_prefix_expression);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::None, parse_none);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::LBrace, parse_hash_literal);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::NumberSign, parse_decorator);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::TestPrint, parse_test_print_expression);
        parser
            .prefix_parse_fn_map
            .insert(TokenType::Comment, |_| None);

        parser
            .infix_parse_fn_map
            .insert(TokenType::LParen, parse_call_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::LBracket, parse_index_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::Assign, parse_assignment_expression);

        parser
            .infix_parse_fn_map
            .insert(TokenType::Dot, parse_object_member_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::GetClassMember, parse_class_member_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::Plus, parse_infix_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::Asterisk, parse_infix_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::Minus, parse_infix_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::Slash, parse_infix_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::Lt, parse_infix_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::Gt, parse_infix_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::Eq, parse_infix_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::NotEq, parse_infix_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::BoolAnd, parse_infix_expression);
        parser
            .infix_parse_fn_map
            .insert(TokenType::BoolOr, parse_infix_expression);

        parser.next_token(); // 初始化当前词法单元

        parser
    }

    pub fn parse_expression_list(&mut self, end: TokenType) -> Vec<Box<Expression>> {
        // 检查下一个词法单元是否为对应结束的词法单元
        if self.peek_token_is(end) {
            self.next_token();
            return vec![]; // 如果是，直接退出，跳过表达式解析
        }

        self.next_token(); // 前进到表达式

        let mut expressions = vec![];
        let expr = self.parse_expression(Lowest);
        if let Some(it) = expr {
            expressions.push(Box::new(it))
        }

        while self.peek_token_is(Comma) {
            self.next_token(); // 离开表达式

            if self.peek_token_is(end) {
                // 尾逗号
                self.next_token();
                break;
            }

            self.next_token(); // 离开逗号

            let expression = self.parse_expression(Lowest);
            if let Some(it) = expression {
                expressions.push(Box::new(it))
            }
        }

        self.next_token(); // 前进到结束的词法单元

        // WARNING: 若想在调用后跳过结束的词法单元，请自行在使用后处理

        expressions
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left: Expression;

        if self
            .prefix_parse_fn_map
            .contains_key(&self.cur_token.token_type)
        {
            left = self.prefix_parse_fn_map[&self.cur_token.token_type](self)?
        } else {
            let token_str = if self.cur_token.token_type == TokenType::Illegal {
                &self.cur_token.value
            } else {
                self.cur_token.token_type.to_string()
            };

            self.errors.push(ParseError { 
                message: format!("no prefix parse function for {token_str} found."),
                token: self.cur_token.clone()
            });

            return None;
        }

        while (!self.peek_token_is(Semicolon))
            && precedence < get_token_precedence(self.peek_token.token_type)
        {
            let infix_parse_fn = self.infix_parse_fn_map.get(&self.peek_token.token_type);
            match infix_parse_fn.cloned() {
                None => {
                    let token_str = if self.cur_token.token_type == TokenType::Illegal {
                        &self.cur_token.value
                    } else {
                        self.cur_token.token_type.to_string()
                    };

                    self.errors.push(ParseError { 
                        message: format!("no infix parse function for {token_str} found."),
                        token: self.cur_token.clone()
                    });

                    return None;
                }
                Some(it) => {
                    self.next_token();
                    left = it(self, left)?
                }
            }
        }

        Some(left)
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let expression_statement = ExpressionStatement {
            expression: match self.parse_expression(Lowest) {
                Some(it) => Some(Box::new(it)),
                None => None,
            }
        };

        if self.peek_token_is(Semicolon) {
            self.next_token();
        }

        Statement::ExpressionStatement(expression_statement)
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        if self
            .statement_parse_fn_map
            .contains_key(&self.cur_token.token_type)
        {
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
                Token::new(
                    Nonsense,
                    NULL_CHAR.to_string(),
                    "<uninit_parser>".to_string(),
                    0,
                    0,
                )
            },
            statements: vec![],
        };

        while !self.cur_token_is(Eof) {
            let statement = self.parse_statement();
            if statement.is_none() {
                self.next_token();
                continue;
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
                Token::eof(
                    self.cur_token.file.clone(), 
                    self.cur_token.line,
                    self.cur_token.column,
                )
            };
        } else {
            self.cur_token = Token::eof(
                self.cur_token.file.clone(), 
                self.cur_token.line,
                self.cur_token.column,
            );
            self.peek_token = Token::eof(
                self.peek_token.file.clone(), 
                self.peek_token.line,
                self.peek_token.column,
            );
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
            self.errors.push(ParseError { 
                message: format!("missing {}.", self.peek_token.token_type.to_string()),
                token: self.peek_token.clone()
            });

            return false;
        }

        self.peek_token.token_type == token_type
    }

    pub fn expect_cur(&mut self, token_type: TokenType) -> bool {
        if self.cur_token.token_type != token_type {
            self.errors.push(ParseError { 
                message: format!("missing {}.", self.peek_token.token_type.to_string()),
                token: self.cur_token.clone()
            });

            return false;
        }

        self.cur_token.token_type == token_type
    }

    pub fn contains_error(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn push_err(&mut self, msg: String) {
        self.errors
            .push(ParseError { message: msg, token: self.cur_token.clone() });
    }

    pub fn print_errors(&self) {
        #[cfg(target_arch = "wasm32")]
        use crate::println;

        println!(
            "parser {}:",
            if self.errors.len() > 1 {
                "errors"
            } else {
                "error"
            }
        );

        for error in &self.errors {
            println!("---> {}", error);
        }
    }
}
