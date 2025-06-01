use std::any::Any;

use crate::ast::ast::{Expression, Node};
use crate::environment::environment::Environment;
use crate::object::object::{Object, ObjectType};
use crate::evaluator::evaluator::Evaluator;
use crate::token::token::Token;
use crate::object::utils::{check_error_name, create_error_with_name, is_error};

use super::identifier::Identifier;

impl Clone for ObjectMemberExpression {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            token: self.token.clone()
        }
    }
}

pub struct ObjectMemberExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub token: Token,
}

impl Node for ObjectMemberExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("({}.{})", self.left.to_string(), self.right.to_string())
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        let left_obj = self.left.eval(evaluator, env);
        if is_error(&left_obj.to_owned()?) {return Some(left_obj?)}

        let member_not_found_err_fn = |obj_type: ObjectType, member_name: String| -> Object {
            create_error_with_name(
    "MemberNotFoundError", format!(
                    "type object '{}' has no member '{}'", obj_type, member_name
                )
            )
        };


        if let Some(mut it) = left_obj {
            let result =  self.right.eval(evaluator, it.get_env_ref());
            
            if result.is_none() {return None}
            if !check_error_name(result.to_owned().unwrap(), "NameError") {return result}

            if let Some(ident) = (self.right.to_owned() as Box<dyn Any>).downcast_ref::<Identifier>() {  
                return Some(member_not_found_err_fn(it.get_type(), ident.to_string()))
            }
        }

        None
    }
}

impl Expression for ObjectMemberExpression {}

pub fn create_object_member_expression(token: Token, left: Box<dyn Expression>, right: Box<dyn Expression>) -> ObjectMemberExpression {
    ObjectMemberExpression {
        token, left, right
    }
}