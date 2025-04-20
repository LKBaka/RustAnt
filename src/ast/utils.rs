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

