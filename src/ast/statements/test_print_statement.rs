use crate::ast::ast::{IExpression, INode};
use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct TestPrintStatement {
    pub value: Box<Expression>,
    pub token: Token,
}

impl INode for TestPrintStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        format!("TestPrint {}", self.value.to_string())
    }
}

impl IExpression for TestPrintStatement {}

pub fn create_test_print_statement(
    token: Token,
    value: Box<Expression>,
) -> TestPrintStatement {
    TestPrintStatement { token, value }
}
