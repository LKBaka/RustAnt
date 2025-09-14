use enum_dispatch::enum_dispatch;

use crate::ast::{
    ast::{IExpression, INode},
    expressions::{
        array_literal::ArrayLiteral, assignment_expression::AssignmentExpression,
        boolean_literal::BooleanLiteral, call_expression::CallExpression,
        class_member_expression::ClassMemberExpression, decorator::Decorator,
        double_literal::DoubleLiteral, function_expression::FunctionExpression,
        hash_literal::HashLiteral, identifier::Identifier, if_expression::{ElseIfExpression, IfExpression},
        index_expression::IndexExpression, infix_expression::InfixExpression,
        integer_literal::IntegerLiteral, none_literal::NoneLiteral,
        object_member_expression::ObjectMemberExpression, prefix_expression::PrefixExpression,
        return_expression::ReturnExpression, string_literal::StringLiteral,
        test_print_expression::TestPrintExpression, tuple_expression::TupleExpression,
    },
};

#[enum_dispatch(IExpression)]
#[derive(Clone, Debug)]
pub enum Expression {
    ArrayLiteral,
    AssignmentExpression,
    BooleanLiteral,
    CallExpression,
    ClassMemberExpression,
    Decorator,
    DoubleLiteral,
    FunctionExpression,
    HashLiteral,
    Identifier,
    IfExpression,
    IndexExpression,
    InfixExpression,
    IntegerLiteral,
    NoneLiteral,
    ObjectMemberExpression,
    PrefixExpression,
    ReturnExpression,
    StringLiteral,
    TestPrintExpression,
    TupleExpression,
    ElseIfExpression,
}

impl INode for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::ArrayLiteral(expr) => expr.token_literal(),
            Expression::AssignmentExpression(expr) => expr.token_literal(),
            Expression::BooleanLiteral(expr) => expr.token_literal(),
            Expression::CallExpression(expr) => expr.token_literal(),
            Expression::ClassMemberExpression(expr) => expr.token_literal(),
            Expression::Decorator(expr) => expr.token_literal(),
            Expression::DoubleLiteral(expr) => expr.token_literal(),
            Expression::FunctionExpression(expr) => expr.token_literal(),
            Expression::HashLiteral(expr) => expr.token_literal(),
            Expression::Identifier(expr) => expr.token_literal(),
            Expression::IfExpression(expr) => expr.token_literal(),
            Expression::IndexExpression(expr) => expr.token_literal(),
            Expression::InfixExpression(expr) => expr.token_literal(),
            Expression::IntegerLiteral(expr) => expr.token_literal(),
            Expression::NoneLiteral(expr) => expr.token_literal(),
            Expression::ObjectMemberExpression(expr) => expr.token_literal(),
            Expression::PrefixExpression(expr) => expr.token_literal(),
            Expression::ReturnExpression(expr) => expr.token_literal(),
            Expression::StringLiteral(expr) => expr.token_literal(),
            Expression::TestPrintExpression(expr) => expr.token_literal(),
            Expression::TupleExpression(expr) => expr.token_literal(),
            Expression::ElseIfExpression(expr) => expr.token_literal(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expression::ArrayLiteral(expr) => expr.to_string(),
            Expression::AssignmentExpression(expr) => expr.to_string(),
            Expression::BooleanLiteral(expr) => expr.to_string(),
            Expression::CallExpression(expr) => expr.to_string(),
            Expression::ClassMemberExpression(expr) => expr.to_string(),
            Expression::Decorator(expr) => expr.to_string(),
            Expression::DoubleLiteral(expr) => expr.to_string(),
            Expression::FunctionExpression(expr) => expr.to_string(),
            Expression::HashLiteral(expr) => expr.to_string(),
            Expression::Identifier(expr) => expr.to_string(),
            Expression::IfExpression(expr) => expr.to_string(),
            Expression::IndexExpression(expr) => expr.to_string(),
            Expression::InfixExpression(expr) => expr.to_string(),
            Expression::IntegerLiteral(expr) => expr.to_string(),
            Expression::NoneLiteral(expr) => expr.to_string(),
            Expression::ObjectMemberExpression(expr) => expr.to_string(),
            Expression::PrefixExpression(expr) => expr.to_string(),
            Expression::ReturnExpression(expr) => expr.to_string(),
            Expression::StringLiteral(expr) => expr.to_string(),
            Expression::TestPrintExpression(expr) => expr.to_string(),
            Expression::TupleExpression(expr) => expr.to_string(),
            Expression::ElseIfExpression(expr) => expr.to_string(),
        }
    }
}

impl IExpression for Expression {}
