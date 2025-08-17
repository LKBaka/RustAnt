use crate::ast::ast::{Expression, Node, Statement};

pub fn print_nodes(nodes: Vec<impl Node>) {
    for node in nodes {
        println!("{}", node.to_string());
    }
}

pub fn print_nodes_with_box(nodes: Vec<Box<dyn Node>>) {
    for node in nodes {
        println!("{}", node.to_string());
    }
}

pub fn print_statements_with_box(statements: Vec<Box<dyn Statement>>) {
    for statement in statements {
        println!("{}", statement.to_string());
    }
}

pub fn print_expressions_with_box(expressions: Vec<Box<dyn Expression>>) {
    for expression in expressions {
        println!("{}", expression.to_string());
    }
}

pub fn expressions_to_string(expressions: &Vec<Box<dyn Expression>>, separator: &str) -> String {
    let mut strings = vec![];

    for expression in expressions {
        strings.push(expression.to_string())
    }

    strings.join(separator)
}

#[macro_export]
macro_rules! impl_node {
    ($struct_name:ident) => {
        impl crate::ast::ast::TypeNameGetter for $struct_name {
            fn type_name(&self) -> String {
                stringify!($struct_name).to_string()
            }
        }
    };
}
