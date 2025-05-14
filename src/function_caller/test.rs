use crate::evaluator::evaluator::Evaluator;
use crate::object::ant_function::AntFunction;
use crate::object::object::IAntObject;

pub fn test_function_call(function: &mut AntFunction, expected_return_value: Box<dyn IAntObject>) {
    use crate::function_caller::function_caller::call_function;

    let mut evaluator: Evaluator = Evaluator::new();

    if function.param_env.map.keys().len() > 0 {
        panic!("this function (test_function_call) is for testing calls to function objects without arguments. If you want to test calls with multiple arguments, switch to test_function_call_with_args.")
    }

    let result = call_function(Box::new(function.clone()), vec![], &mut evaluator, &mut function.env);
    if &result != &expected_return_value {
        panic!("result {} is not equals to {}", result.inspect(), expected_return_value.inspect())
    }

    println!("OK. result: {} expected: {}", result.inspect(), expected_return_value.inspect())
}

pub fn test_function_call_with_args(function: &mut AntFunction, args: Vec<Box<dyn IAntObject>>, expected_return_value: Box<dyn IAntObject>) {
    use crate::function_caller::function_caller::call_function;

    let mut evaluator: Evaluator = Evaluator::new();

    let result = call_function(Box::new(function.clone()), args, &mut evaluator, &mut function.env);
    if &result != &expected_return_value {
        panic!("result {} is not equals to {}", result.inspect(), expected_return_value.inspect())
    }

    println!("OK. result: {} expected: {}", result.inspect(), expected_return_value.inspect())
}

#[test]
fn test_functions_call() {
    use uuid::Uuid;
    use num_bigint::BigInt;

    use crate::environment::environment::Environment;

    use crate::ast::statements::block_statement::create_block_statement;
    use crate::ast::ast::create_expression_statement;
    use crate::ast::expressions::integer_literal::create_integer_literal;

    use crate::token::token::Token;
    use crate::token::token_type::TokenType::LBrace;
    use crate::token::token_type::TokenType::Integer;

    use crate::object::ant_int::AntInt;

    let expected_function_result_map = vec![
        (
            AntFunction {
                id: Uuid::new_v4(),
                env: Environment::new(),
                param_env: Environment::new(),
                block:  create_block_statement(
                    Token::new(LBrace, "{".to_string() , "__test_functions_call__".to_string(), -1),
                    vec![
                        Box::new(
                            create_expression_statement(
                                create_integer_literal(
                                    Token::new(Integer, "".to_string(), "__test_functions_call__".to_string(), -1),
                                    BigInt::from(1)
                                )
                            )
                        )
                    ]
                ),
            }, AntInt::new_with_native_value(Box::new(BigInt::from(1)))
        )
    ];

    for (func, expected_obj) in expected_function_result_map {
        test_function_call(&mut func.to_owned(), expected_obj)
    }
}

#[test]
fn test_functions_call_with_args() {
    use uuid::Uuid;
    use num_bigint::BigInt;

    use crate::environment::environment::Environment;
    use crate::environment::utils::create_env;

    use crate::ast::statements::block_statement::create_block_statement;
    use crate::ast::ast::create_expression_statement;
    use crate::ast::expressions::identifier::create_identifier;

    use crate::token::token::Token;
    use crate::token::token_type::TokenType::LBrace;
    use crate::token::token_type::TokenType::Ident;

    use crate::constants::null_obj;

    use crate::object::ant_int::AntInt;

    let expected_function_result_map = vec![
        (
            AntFunction {
                id: Uuid::new_v4(),
                env: Environment::new(),
                param_env: create_env(vec![("value".to_string(), null_obj.clone())]),
                block:  create_block_statement(
                    Token::new(LBrace, "{".to_string() , "__test_functions_call__".to_string(), -1),
                    vec![
                        Box::new(
                            create_expression_statement(
                                create_identifier(
                                    Token::new(Ident, "value".to_string() , "__test_functions_call__".to_string(), -1),
                                    "value".to_string()
                                )
                            )
                        )
                    ]
                ),
            },
            vec![AntInt::new_with_native_value(Box::new(BigInt::from(91)))],
            AntInt::new_with_native_value(Box::new(BigInt::from(91)))
        )
    ];

    for (func, args, expected_obj) in expected_function_result_map {
        test_function_call_with_args(&mut func.to_owned(), args, expected_obj)
    }
}