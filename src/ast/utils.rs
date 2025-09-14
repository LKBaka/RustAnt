use crate::ast::expr::Expression;

use crate::ast::ast::INode;

#[cfg(test)]
pub fn print_nodes(nodes: Vec<impl INode>) {
    for node in nodes {
        println!("{}", node.to_string());
    }
}

pub fn expressions_to_string(expressions: &Vec<Box<Expression>>, separator: &str) -> String {
    let mut strings = vec![];

    for expression in expressions {
        strings.push(expression.to_string())
    }

    strings.join(separator)
}
