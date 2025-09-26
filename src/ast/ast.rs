use dyn_clone::{DynClone, clone_trait_object};
use enum_dispatch::enum_dispatch;
use std::any::Any;
use std::fmt::Debug;

use crate::ast::expr::Expression;
use crate::ast::stmt::Statement;
use crate::constants::NEW_LINE;

use crate::token::token::Token;

pub trait TypeNameGetter {
    fn type_name(&self) -> String;
}

impl<T: INode> TypeNameGetter for T {
    fn type_name(&self) -> String {
        self.to_string()
    }
}

#[enum_dispatch(INode)]
#[derive(Debug, Clone)]
pub enum Node {
    Program,
    Statement,
    Expression
}

#[enum_dispatch]
pub trait INode: DynClone + Sync + Send + Any + Debug + TypeNameGetter {
    fn token_literal(&self) -> String;
    fn token(&self) -> Token;
    fn to_string(&self) -> String;
    
    fn as_any(&self) -> &(dyn Any + '_)
    where
        Self: Sized,
    {
        self
    }
}

clone_trait_object!(INode);

pub trait IExpression: INode {}
pub trait IStatement: INode {}

clone_trait_object!(IExpression);
clone_trait_object!(IStatement);

#[derive(Debug)]
pub struct Program {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            statements: self.statements.clone(),
        }
    }
}

impl INode for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }

    fn token(&self) -> Token {
        if !self.statements.is_empty() {
            self.statements[0].token()
        } else {
            Token::eof(String::from("unknown"), 0, 0)
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        for statement in &self.statements {
            s.push_str(&format!(
                "{}{}",
                statement.to_string(),
                NEW_LINE.to_string()
            ));
        }

        s
    }
}

impl Clone for ExpressionStatement {
    fn clone(&self) -> Self {
        Self {
            expression: self.expression.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Option<Box<Expression>>,
}

impl INode for ExpressionStatement {
    fn token_literal(&self) -> String {
        if self.expression.is_none() {
            "".to_string()
        } else {
            self.expression.clone().unwrap().token_literal()
        }
    }

    fn token(&self) -> Token {
        if self.expression.is_none() {
            Token::eof(String::from("unknown"), 0, 0)
        } else {
            self.expression.clone().unwrap().token()
        }
    }

    fn to_string(&self) -> String {
        if self.expression.is_none() {
            "".to_string()
        } else {
            self.expression.clone().unwrap().to_string()
        }
    }
}

impl IStatement for ExpressionStatement {}

#[cfg(test)]
pub fn create_expression_statement(expression: Expression) -> ExpressionStatement {
    ExpressionStatement {
        expression: Some(Box::new(expression)),
    }
}
