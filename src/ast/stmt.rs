use enum_dispatch::enum_dispatch;

use crate::ast::{ast::{ExpressionStatement, INode, IStatement}, statements::{block_statement::BlockStatement, class_statement::ClassStatement, let_statement::LetStatement, use_statement::UseStatement, while_statement::WhileStatement}};

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


impl INode for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::BlockStatement(stmt) => stmt.token_literal(),
            Statement::ClassStatement(stmt) => stmt.token_literal(),
            Statement::LetStatement(stmt) => stmt.token_literal(),
            Statement::UseStatement(stmt) => stmt.token_literal(),
            Statement::WhileStatement(stmt) => stmt.token_literal(),
            Statement::ExpressionStatement(stmt) => stmt.token_literal(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Statement::BlockStatement(stmt) => stmt.to_string(),
            Statement::ClassStatement(stmt) => stmt.to_string(),
            Statement::LetStatement(stmt) => stmt.to_string(),
            Statement::UseStatement(stmt) => stmt.to_string(),
            Statement::WhileStatement(stmt) => stmt.to_string(),
            Statement::ExpressionStatement(stmt) => stmt.to_string(),
        }
    }
}

impl IStatement for Statement {}