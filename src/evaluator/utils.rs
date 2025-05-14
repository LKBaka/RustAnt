use crate::ast::ast::Expression;
use crate::constants::{ant_false, ant_true};
use crate::environment::environment::Environment;
use crate::object::object::IAntObject;

use super::evaluator::Evaluator;

pub fn eval_expressions(expressions: &Vec<Box<dyn Expression>>, evaluator: &mut Evaluator, env: &mut Environment) -> Vec<Box<dyn IAntObject>> {
    let mut vec = vec![];

    for expression in expressions {
        let eval_result = expression.clone().eval(evaluator, env);
        if let Some(it) = eval_result {
            vec.push(it)
        }
    }

    vec
}

pub fn native_boolean_to_boolean_obj(boolean: bool) -> Box<dyn IAntObject> {
    if boolean {
        ant_true.clone()
    } else {
        ant_false.clone()
    }
}