use std::any::{Any, TypeId};

use colored::Colorize;

use crate::ast::expressions::function_expression::FunctionExpression;
use crate::environment::environment::Environment;
use crate::object::object::{IAntObject, Object};
use crate::ast::ast::{Node, Program};
use crate::object::ant_error::AntError;
use crate::utils::type_of;

pub struct Info {
    pub module: Option<String>,
    pub file: String,
    pub line: i64,
}

impl Info {
    fn new(module: Option<String>, file: String, line: i64) -> Info {
        Info {
            module, file, line
        }
    }

    fn to_string(&self) -> String {
        if let Some(module) = self.module.clone() {
            format!("File <{}>, line {}, in <{}>:", self.file, self.line, module)
        } else {
            format!("File <{}>, line {}:", self.file, self.line)
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

    pub fn eval_box(&mut self, node: &mut Box<dyn Node + 'static>, env: &mut Environment) -> Option<Object> {
        if type_of(node) == TypeId::of::<Program>() {
            return self.eval_program(node.as_mut(), env)
        }

        if let Some(it) = (node.as_ref() as &dyn Any).downcast_ref::<FunctionExpression>() {
            self.call_stack.push(Info::new(it.name.clone(), it.token.file.clone(), it.token.line));

            return node.eval(self, env)
        }

        node.eval(self, env)
    }

    pub fn eval(&mut self, node: &mut (impl Node + Clone + 'static), env: &mut Environment) -> Option<Object> {
        if type_of(node) == TypeId::of::<Program>() {
            return self.eval_program(node, env)
        }

        if let Some(it) = (Box::new(node.clone())).into_any().downcast_ref::<FunctionExpression>() {
            self.call_stack.push(Info::new(it.name.clone(), it.token.file.clone(), it.token.line.clone()));

            return node.clone().eval(self, env)
        }

        node.clone().eval(self, env)
    }

    pub fn eval_program(&mut self, node: &mut dyn Node, env: &mut Environment) -> Option<Object> {
        let program = (node as &mut dyn Any).downcast_mut::<Program>().unwrap();

        let program_token = program.token.clone();

        self.call_stack.push(Info::new(None, program_token.file.clone(), program_token.line.clone()));

        let result = program.eval(self, env);
        match result {
            None => {None},
            Some(result) => {
                if let Some(error) = result
                    .as_any()
                    .downcast_ref::<AntError>() 
                {
                    self.print_call_stack(error);
                }

                Some(result)
            }
        }
    }

    pub fn print_call_stack(&self, error: &AntError) {
        println!("{}", "Traceback (most recent call last):".to_string().red());

        for info in &self.call_stack {
            println!("  {}", info.to_string())
        }

        println!("{}", error.inspect().red())
    }
}