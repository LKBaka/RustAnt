use crate::ast::ast::{Expression, Node};
use crate::constants::null_obj;
use crate::environment::environment::Environment;
use crate::evaluator::utils::native_boolean_to_boolean_obj;
use crate::object::ant_double::AntDouble;
use crate::object::ant_int::AntInt;
use crate::object::object::Object;
use crate::object::utils::is_truthy;
use crate::token::token::Token;
use crate::evaluator::evaluator::Evaluator;
use crate::impl_node;

impl Clone for PrefixExpression {
    fn clone(&self) -> Self {
        Self {
            operator: self.operator.clone(),
            expression: self.expression.clone(),
            token: self.token.clone()
        }
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub operator: String,
    pub expression: Box<dyn Expression>,
    pub token: Token,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("{}{}", self.operator, self.expression.to_string())
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        let right = self.expression.eval(evaluator, env).unwrap_or(null_obj.clone());

        match self.operator.as_str() {
            "!" =>  Some(native_boolean_to_boolean_obj(!is_truthy(right))),
            "-" => {
                if let Some(num) = right.as_any().downcast_ref::<AntInt>() {
                    Some(Box::new(AntInt::from(-&num.value)))
                } else if let Some(num) = right.as_any().downcast_ref::<AntDouble>() {
                    Some(Box::new(AntDouble::from(-&num.value)))
                } else {
                    eprintln!("invalid operand for prefix operator '-': {:?}", right);
                    None
                }
            },
            _ => {
                eprintln!("unknown prefix operator: {}", self.operator);
                None
            }
        }
    }
}

impl Expression for PrefixExpression {}

impl_node!(PrefixExpression);

pub fn create_prefix_expression(token: Token, operator: String, expression: Box<dyn Expression>) -> PrefixExpression {
    PrefixExpression {
        token, operator, expression
    }
}