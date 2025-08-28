use crate::ast::ast::{Expression, Node, Statement};

use crate::token::token::Token;

impl Clone for IfExpression {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            condition: self.condition.clone(),
            consequence: self.consequence.clone(),
            alternative: self.alternative.clone(),
            else_if_expressions: self.else_if_expressions.clone(),
        }
    }
}

#[derive(Debug)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<dyn Expression>,          // 条件
    pub consequence: Box<dyn Statement>,         // 默认块
    pub alternative: Option<Box<dyn Statement>>, // Else 分支块
    pub else_if_expressions: Option<Vec<Box<dyn Expression>>>, // ElseIf 分支块
}

impl Node for IfExpression {
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
                s.push_str(&*else_if_expression.to_string())
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
            result.push_str(&format!(" {}", else_if_string));
        }

        if !alternative_string.is_empty() {
            result.push_str(&format!(" else {{{}}}", alternative_string));
        }

        result
    }
}

impl Expression for IfExpression {}

pub fn create_if_expression(
    token: Token,
    condition: Box<dyn Expression>,
    consequence: Box<dyn Statement>,
    alternative: Option<Box<dyn Statement>>,
    else_if_expressions: Option<Vec<Box<dyn Expression>>>,
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
    pub condition: Box<dyn Expression>,  // 条件
    pub consequence: Box<dyn Statement>, // 默认块
}

impl Node for ElseIfExpression {
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

impl Expression for ElseIfExpression {}

pub fn create_else_if_expression(
    token: Token,
    condition: Box<dyn Expression>,
    consequence: Box<dyn Statement>,
) -> ElseIfExpression {
    ElseIfExpression {
        token,
        condition,
        consequence,
    }
}
