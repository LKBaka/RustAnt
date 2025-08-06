use std::any::Any;
use std::fmt::Debug;
use dyn_clone::{clone_trait_object, DynClone};

use crate::constants::{null_obj, NEW_LINE};
use crate::evaluator::evaluator::Evaluator;
use crate::impl_node;
use crate::object::utils::is_error;
use crate::token::token::Token;
use crate::environment::environment::Environment;
use crate::object::object::Object;

pub trait TypeNameGetter {
    fn type_name(&self) -> String;
}

pub trait Node: DynClone + Sync + Send + Any + Debug + TypeNameGetter {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object>;

    fn as_any(&self) -> &(dyn Any + '_) where Self: Sized {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> where Self: Sized {
        self
    }
}

clone_trait_object!(Node);

pub trait Expression: Node {}
pub trait Statement: Node {}

clone_trait_object!(Expression);
clone_trait_object!(Statement);

#[derive(Debug)]
pub struct Program {
    pub token: Token,
    pub(crate) statements: Vec<Box<dyn Statement>>
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            statements: self.statements.clone()
        }
    }
}


impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        for statement in &self.statements {
            s.push_str(&format!("{}{}", statement.to_string(), NEW_LINE.to_string()));
        }
        
        s
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        let mut result = Some(null_obj.clone());

        for statement in &mut self.statements {
            result = statement.eval(evaluator, env);

            if let Some(it) = &result && 
                is_error(&it) 
            {
                return result;
            }
        }

        result
    }
}

impl_node!(Program);

impl Clone for ExpressionStatement {
    fn clone(&self) -> Self {
        Self {
            expression: self.expression.clone()
        }
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Option<Box<dyn Expression>>
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
       if self.expression.is_none() {
           "".to_string()
       } else {
           self.expression.clone().unwrap().token_literal()
       }
    }

    fn to_string(&self) -> String {
        if self.expression.is_none() {
            "".to_string()
        } else {
            self.expression.clone().unwrap().to_string()
        }
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        if let Some(expression) = self.expression.as_mut() {
            expression.eval(evaluator, env)
        } else {
            None
        }
    }
}

impl Statement for ExpressionStatement {}

#[cfg(test)]
pub fn create_expression_statement(expression: impl Expression + 'static) -> ExpressionStatement {
    ExpressionStatement {
        expression: Some(Box::new(expression) as Box<dyn Expression>)
    }
}

impl_node!(ExpressionStatement);
