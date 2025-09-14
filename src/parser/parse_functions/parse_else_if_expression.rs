use crate::ast::expressions::if_expression::create_else_if_expression;
use crate::parser::parse_functions::parse_block_statement::parse_block_statement;
use crate::parser::precedence::Precedence;

use crate::ast::expr::Expression;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType;

pub fn parse_else_if_expression(parser: &mut Parser) -> Option<Expression> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开 if 词法单元

    let condition = match parser.parse_expression(Precedence::Lowest) {
        Some(expr) => expr,
        None => {
            parser.push_err(format!("missing condition.",));
            return None;
        }
    };

    parser.next_token(); // 离开表达式 (正常应跳转到左大括号)

    let block = match parse_block_statement(parser) {
        Some(block) => block,
        None => {
            parser.push_err(format!("missing else if body",));
            return None;
        }
    };

    if !parser.expect_cur(TokenType::RBrace) {
        return None;
    }

    Some(Expression::ElseIfExpression(create_else_if_expression(
        token,
        Box::new(condition),
        block,
    )))
}
