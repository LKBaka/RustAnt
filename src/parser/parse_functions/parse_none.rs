use crate::ast::expr::Expression;
use crate::ast::expressions::none_literal::create_none_literal;
use crate::parser::parser::Parser;

pub fn parse_none(parser: &mut Parser) -> Option<Expression> {
    Some(Expression::NoneLiteral(create_none_literal(
        parser.cur_token.clone(),
    )))
}
