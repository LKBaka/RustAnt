use std::any::Any;

use crate::ast::ast::{Expression, Node};
use crate::environment::environment::Environment;
use crate::object::object::{Object, ObjectType};
use crate::evaluator::evaluator::Evaluator;
use crate::token::token::Token;
use crate::object::utils::{check_error_name, create_error_with_name, is_error};
use crate::impl_node;

use super::identifier::Identifier;

impl Clone for ClassMemberExpression {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            token: self.token.clone()
        }
    }
}

#[derive(Debug)]
pub struct ClassMemberExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub token: Token,
}

impl Node for ClassMemberExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("({}::{})", self.left.to_string(), self.right.to_string())
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        let left_obj = self.left.eval(evaluator, env);
        if is_error(&left_obj.clone()?) {return Some(left_obj?)}

        let member_not_found_err_fn = |obj_type: ObjectType, member_name: String| -> Object {
            create_error_with_name(
    "MemberNotFoundError", format!(
                    "type object '{}' has no member '{}'", obj_type, member_name
                )
            )
        };


        if let Some(mut it) = left_obj {
            let obj_env = it.get_env_ref();
            let result =  self.right.eval(evaluator, obj_env);
            
            if let Some(result) = result {
                if !check_error_name(&result, "NameError") {return Some(result)}

                if let Some(ident) = (self.right.as_ref() as &dyn Any).downcast_ref::<Identifier>() {  
                    return Some(member_not_found_err_fn(it.get_type(), ident.to_string()))
                }
            }

        }

        None
    }
}

impl Expression for ClassMemberExpression {}

impl_node!(ClassMemberExpression);

pub fn create_class_member_expression(token: Token, left: Box<dyn Expression>, right: Box<dyn Expression>) -> ClassMemberExpression {
    ClassMemberExpression {
        token, left, right
    }
}