use crate::ast::ast::{Expression, Node};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct TestPrintExpression {
    pub value: Box<dyn Expression>,
    pub token: Token,
}

impl Node for TestPrintExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("TestPrint {}", self.value.to_string())
    }
}

impl Expression for TestPrintExpression {}

pub fn create_test_print_expression(
    token: Token,
    value: Box<dyn Expression>,
) -> TestPrintExpression {
    TestPrintExpression { token, value }
}
