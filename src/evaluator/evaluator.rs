use std::any::TypeId;
use std::ops::Deref;
use crate::environment::environment::Environment;
use crate::object::object::{IAntObject, ERROR};
use crate::ast::ast::{Node, Program};
use crate::object::ant_error::AntError;
use crate::utils::type_of;

pub struct Info {
    pub(crate) file: String,
    pub(crate) line: i64,
}

impl Info {
    fn new(file: String, line: i64) -> Info {
        Info {
            file, line
        }
    }
}

pub struct Evaluator {
    pub(crate) call_stack: Vec<Info>
}

impl Evaluator {
    pub(crate) fn new() -> Evaluator {
        Evaluator {
            call_stack: vec![]
        }
    }

    pub fn eval(&mut self, node: impl Node + Clone + 'static, env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        if type_of(&node) == TypeId::of::<Program>() {
            return self.eval_program(node.clone(), env)
        }

        node.clone().eval(env)
    }

    pub fn eval_program(&mut self, node: impl Node + Clone + 'static, env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        let cloned_node = node.clone();

        let program = Box::new(cloned_node).into_any().downcast_ref::<Program>().unwrap().clone();

        let program_token = program.token.clone();

        self.call_stack.push(Info::new(program_token.file.clone(), program_token.line.clone()));

        let result = node.clone().eval(env);
        match result {
            None => {None},
            Some(result) => {
                if result.get_type() == ERROR.to_string() {
                    self.print_call_stack(result.deref().as_any().downcast_ref::<AntError>().cloned().unwrap())
                }

                Some(result)
            }
        }
    }

    pub fn print_call_stack(&self, error: AntError) {
        for info in &self.call_stack {
            println!("at file <{}>, line {}:", info.file, info.line)
        }

        println!("---> {}", error.inspect())
    }
}