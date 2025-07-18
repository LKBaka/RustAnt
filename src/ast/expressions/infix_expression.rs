use crate::ast::ast::{Expression, Node};
use crate::ast::constants::OPERATOR_TO_FUNCTION_NAME_MAP;
use crate::environment::environment::Environment;
use crate::function_caller::function_caller::call_function_with_name;
use crate::object::object::Object;
use crate::evaluator::evaluator::Evaluator;
use crate::token::token::Token;
use crate::object::utils::is_error;

impl Clone for InfixExpression {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            operator: self.operator.clone(),
            token: self.token.clone()
        }
    }
}

pub struct InfixExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: String,
    pub token: Token,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("({} {} {})", self.left.to_string(), self.operator, self.right.to_string())
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        let left_obj = self.left.eval(evaluator, env)?;
        if is_error(&left_obj) {return Some(left_obj)}

        let right_obj = self.right.eval(evaluator, env)?;
        if is_error(&right_obj) {return Some(right_obj)}

        let call_result = call_function_with_name(
            OPERATOR_TO_FUNCTION_NAME_MAP[&self.operator].to_string(), 
            &vec![&left_obj, &right_obj], 
            evaluator, 
            &mut left_obj.get_env()
        );

        if let Ok(it) = call_result {
            return it
        } else if let Err(err) = call_result {
            return Some(err)
        } else {None}
    }
}

impl Expression for InfixExpression {}

pub fn create_infix_expression(token: Token, left: Box<dyn Expression>, right: Box<dyn Expression>, operator: String) -> InfixExpression {
    InfixExpression {
        token, left, right, operator
    }
}