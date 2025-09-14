use crate::ast::ast::{IExpression, INode};

use crate::ast::expr::Expression;
use crate::ast::stmt::Statement;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,          // 条件
    pub consequence: Statement,         // 默认块
    pub alternative: Option<Statement>, // Else 分支块
    pub else_if_expressions: Option<Vec<Box<Expression>>>, // ElseIf 分支块
}

impl INode for IfExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        let alternative_string = if let Some(alternative) = &self.alternative {
            alternative.to_string()
        } else {
            "".to_string()
        };

        let else_if_string = if let Some(it) = &self.else_if_expressions {
            let mut s = String::from("");

            for else_if_expression in it {
                s.push_str(&else_if_expression.to_string())
            }

            s
        } else {
            "".to_string()
        };

        let mut result = format!(
            "if ({}) {{{}}}",
            self.condition.to_string(),
            self.consequence.to_string()
        );

        if !else_if_string.is_empty() {
            result.push_str(&format!(" {else_if_string}"));
        }

        if !alternative_string.is_empty() {
            result.push_str(&format!(" else {{{alternative_string}}}"));
        }

        result
    }
}

impl IExpression for IfExpression {}

pub fn create_if_expression(
    token: Token,
    condition: Box<Expression>,
    consequence: Statement,
    alternative: Option<Statement>,
    else_if_expressions: Option<Vec<Box<Expression>>>,
) -> IfExpression {
    IfExpression {
        token,
        condition,
        consequence,
        alternative,
        else_if_expressions,
    }
}

impl Clone for ElseIfExpression {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            condition: self.condition.clone(),
            consequence: self.consequence.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ElseIfExpression {
    pub token: Token,
    pub condition: Box<Expression>,  // 条件
    pub consequence: Statement, // 默认块
}

impl INode for ElseIfExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "else if ({}) {{{}}}",
            self.condition.to_string(),
            self.consequence.to_string()
        )
    }
}

impl IExpression for ElseIfExpression {}

pub fn create_else_if_expression(
    token: Token,
    condition: Box<Expression>,
    consequence: Statement,
) -> ElseIfExpression {
    ElseIfExpression {
        token,
        condition,
        consequence,
    }
}
