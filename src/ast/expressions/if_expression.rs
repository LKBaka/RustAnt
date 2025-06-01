use crate::ast::ast::{Expression, Node, Statement};
use crate::constants::null_obj;
use crate::environment::environment::Environment;
use crate::object::object::Object;
use crate::evaluator::evaluator::Evaluator;
use crate::object::utils::is_truthy;
use crate::token::token::Token;

impl Clone for IfExpression {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            condition: self.condition.clone(),
            consequence: self.consequence.clone(),
            alternative: self.alternative.clone(),
            else_if_expressions: self.else_if_expressions.clone()
        }
    }
}

pub struct IfExpression {
    pub token: Token,
    condition: Box<dyn Expression>, // 条件
    consequence: Box<dyn Statement>, // 默认块
    alternative: Option<Box<dyn Statement>>, // Else 分支块
    else_if_expressions: Option<Vec<Box<dyn Expression>>> // ElseIf 分支块
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
        } else {"".to_string()};

        format!(
            "if ({}) {{{}}} else if {{{}}} else {{{}}}",
            self.condition.to_string(), self.consequence.to_string(), else_if_string, alternative_string
        )
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        let condition: Object = self.condition.eval(evaluator, env)?;
        
        if is_truthy(condition) {
            // 条件为真，直接求值默认代码块
            self.consequence.eval(evaluator, env)
        } else {
            if let Some(ref mut it) = self.else_if_expressions {
                // 条件为假，但是存在其他条件分支判断，遍历所有其他分支，只要其他分支返回一个值，则返回该值，否则继续尝试访问最终的分支

                let mut result: Option<Object> = None;

                for else_if_expression in it {
                    let eval_result = else_if_expression.eval(evaluator, env);

                    if let Some(it) = eval_result {
                        result = Some(it);
                        break;
                    }
                }

                if result.is_some() {
                    return result;
                }
            }

            if let Some(ref mut it) = self.alternative {
                return it.eval(evaluator, env)
            }

            // 如果不存在其他任何分支，则返回空
            Some(null_obj.clone())
        }
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
        else_if_expressions
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

pub struct ElseIfExpression {
    pub token: Token,
    pub condition: Box<dyn Expression>, // 条件
    pub consequence: Box<dyn Statement>, // 默认块
}

impl Node for ElseIfExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "else if {} {{{}}}",
            self.condition.to_string(), self.consequence.to_string()
        )
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        let condition = self.condition.eval(evaluator, env)?;
        
        if is_truthy(condition) {
            self.consequence.eval(evaluator, env)
        } else {
            None
        }
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