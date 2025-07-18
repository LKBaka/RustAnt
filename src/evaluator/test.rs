#[test]
fn test_call_stack_print() {
    use crate::ast::ast::Program;
    use crate::token::token::Token;
    use crate::environment::environment::Environment;
    use crate::object::ant_error::AntError;
    use crate::evaluator::evaluator::*;
    use crate::token::token_type::TokenType::Nonsense;

    // 创建一个模拟的 Program 节点
    let token = Token {
        file: "test.ant".to_string(),
        line: 42,
        token_type: Nonsense,
        value: "91".to_string(),
    };

    let mut program = Program {
        token: token.clone(),
        statements: vec![],
    };

    // 创建一个模拟的错误
    let error = AntError::new_with_native_value(Box::new("Test error".to_string()));

    // 创建 Evaluator 并测试调用栈打印
    let mut evaluator = Evaluator::new();
    let mut env = Environment::new();

    // 求值
    evaluator.eval(&mut program, &mut env);

    evaluator.print_call_stack(error.as_any().downcast_ref::<AntError>().unwrap());
}