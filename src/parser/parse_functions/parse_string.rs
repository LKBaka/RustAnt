use crate::ast::expr::Expression;
use crate::ast::expressions::string_literal::create_string_literal;
use crate::parser::parser::Parser;

pub fn parse_string(parser: &mut Parser) -> Option<Expression> {
    Some(Expression::StringLiteral(create_string_literal(
        parser.cur_token.clone(),
        parser.cur_token.value.clone(),
    )))
}
