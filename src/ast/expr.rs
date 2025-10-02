use enum_dispatch::enum_dispatch;

use crate::{
    ast::{
        ast::{IExpression, INode},
        expressions::{
            array_literal::ArrayLiteral,
            assignment_expression::AssignmentExpression,
            boolean_literal::BooleanLiteral,
            call_expression::CallExpression,
            class_member_expression::ClassMemberExpression,
            decorator::Decorator,
            double_literal::DoubleLiteral,
            function_expression::FunctionExpression,
            hash_literal::HashLiteral,
            identifier::Identifier,
            if_expression::{ElseIfExpression, IfExpression},
            index_expression::IndexExpression,
            infix_expression::InfixExpression,
            integer_literal::IntegerLiteral,
            integer64_literal::Int64Literal,
            none_literal::NoneLiteral,
            object_member_expression::ObjectMemberExpression,
            prefix_expression::PrefixExpression,
            string_literal::StringLiteral,
            test_print_expression::TestPrintExpression,
            tuple_expression::TupleExpression,
        },
    },
    token::token::Token,
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
    Int64Literal,
    NoneLiteral,
    ObjectMemberExpression,
    PrefixExpression,
    StringLiteral,
    TestPrintExpression,
    TupleExpression,
    ElseIfExpression,
}

macro_rules! auto_expr {
    ($self:ident, $method:ident) => {
        match $self {
            Expression::ArrayLiteral(expr) => expr.$method(),
            Expression::AssignmentExpression(expr) => expr.$method(),
            Expression::BooleanLiteral(expr) => expr.$method(),
            Expression::CallExpression(expr) => expr.$method(),
            Expression::ClassMemberExpression(expr) => expr.$method(),
            Expression::Decorator(expr) => expr.$method(),
            Expression::DoubleLiteral(expr) => expr.$method(),
            Expression::FunctionExpression(expr) => expr.$method(),
            Expression::HashLiteral(expr) => expr.$method(),
            Expression::Identifier(expr) => expr.$method(),
            Expression::IfExpression(expr) => expr.$method(),
            Expression::IndexExpression(expr) => expr.$method(),
            Expression::InfixExpression(expr) => expr.$method(),
            Expression::IntegerLiteral(expr) => expr.$method(),
            Expression::NoneLiteral(expr) => expr.$method(),
            Expression::ObjectMemberExpression(expr) => expr.$method(),
            Expression::PrefixExpression(expr) => expr.$method(),
            Expression::StringLiteral(expr) => expr.$method(),
            Expression::TestPrintExpression(expr) => expr.$method(),
            Expression::TupleExpression(expr) => expr.$method(),
            Expression::ElseIfExpression(expr) => expr.$method(),
            Expression::Int64Literal(expr) => expr.$method(),
        }
    };
}

impl INode for Expression {
    fn token_literal(&self) -> String {
        auto_expr!(self, token_literal)
    }

    fn token(&self) -> Token {
        auto_expr!(self, token)
    }

    fn to_string(&self) -> String {
        auto_expr!(self, to_string)
    }
}

impl IExpression for Expression {}
