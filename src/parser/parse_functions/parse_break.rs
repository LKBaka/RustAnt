use crate::ast::expr::Expression;
use crate::ast::expressions::break_expression::create_break_expression;
use crate::parser::parser::Parser;

pub fn parse_break(parser: &mut Parser) -> Option<Expression> {
    Some(Expression::BreakExpression(create_break_expression(
        parser.cur_token.clone(),
    )))
}
