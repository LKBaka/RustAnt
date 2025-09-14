use crate::ast::expr::Expression;
use crate::ast::expressions::prefix_expression::create_prefix_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;

pub fn parse_prefix_expression(parser: &mut Parser) -> Option<Expression> {
    let token = parser.cur_token.clone();
    let operator = token.value.clone();

    parser.next_token(); // 离开前缀运算符

    let right = {
        match parser.parse_expression(Precedence::Prefix) {
            Some(expr) => expr,
            None => {
                parser.push_err(format!("expected expression after '{}'.", operator));
                return None;
            }
        }
    };

    Some(Expression::PrefixExpression(create_prefix_expression(
        token,
        operator,
        Box::new(right),
    )))
}
