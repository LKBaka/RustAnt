use crate::ast::expressions::decorator::create_decorator;
use crate::parser::precedence::Precedence;
use crate::ast::expr::Expression;
use crate::token::token_type::TokenType;

use crate::parser::parser::Parser;

pub fn parse_decorator(parser: &mut Parser) -> Option<Expression> {
    let token = parser.cur_token.clone();

    if !parser.expect_peek(TokenType::LBracket) {
        return None;
    }

    parser.next_token(); // 离开 #
    parser.next_token(); // 离开 [

    let decorator = match parser.parse_expression(Precedence::Lowest) {
        Some(expr) => expr,
        None => {
            parser.push_err(String::from("missing decorator"));
            return None
        }
    };

    parser.next_token(); // 离开表达式
    parser.next_token(); // 离开 ]

    let to_decorate = match parser.parse_statement() {
        Some(stmt) => stmt,
        None => {
            parser.push_err(String::from("missing statement to decorate"));
            return None
        }
    };

    Some(Expression::Decorator(create_decorator(token, Box::new(decorator), to_decorate)))
}