use crate::ast::expr::Expression;
use crate::ast::expressions::continue_expression::create_continue_expression;
use crate::parser::parser::Parser;

pub fn parse_continue(parser: &mut Parser) -> Option<Expression> {
    Some(Expression::ContinueExpression(create_continue_expression(
        parser.cur_token.clone(),
    )))
}