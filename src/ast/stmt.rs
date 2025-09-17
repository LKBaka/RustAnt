use enum_dispatch::enum_dispatch;

use crate::{ast::{ast::{ExpressionStatement, INode, IStatement}, statements::{block_statement::BlockStatement, class_statement::ClassStatement, let_statement::LetStatement, use_statement::UseStatement, while_statement::WhileStatement}}, token::token::Token};

#[enum_dispatch(IStatement)]
#[derive(Debug, Clone)]
pub enum Statement {
    BlockStatement,
    ClassStatement,
    LetStatement,
    UseStatement,
    WhileStatement,
    ExpressionStatement,
}

macro_rules! auto_stmt {
    ($self:ident, $method:ident) => {
        match $self {
            Statement::BlockStatement(stmt) => stmt.$method(),
            Statement::ClassStatement(stmt) => stmt.$method(),
            Statement::LetStatement(stmt) => stmt.$method(),
            Statement::UseStatement(stmt) => stmt.$method(),
            Statement::WhileStatement(stmt) => stmt.$method(),
            Statement::ExpressionStatement(stmt) => stmt.$method(),
        }
    };
}

impl INode for Statement {
    fn token_literal(&self) -> String {
        auto_stmt!(self, token_literal)
    }

    fn token(&self) -> Token {
        auto_stmt!(self, token)
    }

    fn to_string(&self) -> String {
        auto_stmt!(self, token_literal)
    }
}

impl IStatement for Statement {}