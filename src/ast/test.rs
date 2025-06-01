#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use std::ops::Deref;

use crate::ast::ast::Node;
use crate::environment::environment::Environment;
use crate::object::object::Object;
use crate::evaluator::evaluator::{self, Evaluator};


fn test_node_eval(mut node: impl Node, expected_obj: Object) {
    let mut evaluator = Evaluator::new();

    let result = node.eval(&mut evaluator, &mut Environment::new());
    match result {
        None => {}
        Some(it) => {
            if !(it == expected_obj.clone()) {
                panic!("Expected eval result is {}, but now it is {}", expected_obj.clone().inspect(), it.inspect())
            }

            println!("OK. result: {}, expected: {}", it.inspect(), expected_obj.inspect())
        }
    }

}


#[test]
fn test_print_nodes() {
    use num_bigint::BigInt;

    use crate::ast::ast::create_expression_statement;
    use crate::ast::expressions::integer_literal::create_integer_literal;
    use crate::ast::utils::print_nodes;
    use crate::token::token::Token;
    use crate::token::token_type::TokenType;

    let nodes = vec![
        create_expression_statement(
            create_integer_literal(
                Token::new(TokenType::Integer, "91".to_string(), "__test_print_nodes__".to_string(), -1),
                BigInt::from(91)
            )
        ),
        create_expression_statement(
            create_integer_literal(
                Token::new(TokenType::Integer, "78".to_string(), "__test_print_nodes__".to_string(), -1),
                BigInt::from(78)
            )
        ),
    ];

    print_nodes(nodes);
}

#[test]
fn test_nodes_eval() {
    use num_bigint::BigInt;

    use crate::ast::ast::create_expression_statement;
    use crate::ast::expressions::integer_literal::create_integer_literal;
    use crate::object::ant_int::AntInt;
    use crate::token::token::Token;
    use crate::token::token_type::TokenType;

    let expected_obj_map = vec![
        (
            create_expression_statement(
                create_integer_literal(
                    Token::new(TokenType::Integer, "91".to_string(), "__test_print_nodes__".to_string(), -1),
                    BigInt::from(91)
                )
            ),
            AntInt::new_with_native_value(Box::new(BigInt::from(91)))
        ),
        (
            create_expression_statement(
                create_integer_literal(
                    Token::new(TokenType::Integer, "78".to_string(), "__test_print_nodes__".to_string(), -1),
                    BigInt::from(78)
                )
            ),
            AntInt::new_with_native_value(Box::new(BigInt::from(78)))
        )
    ];

    for (node, expected_obj) in expected_obj_map {
        test_node_eval(node, expected_obj)
    }
}