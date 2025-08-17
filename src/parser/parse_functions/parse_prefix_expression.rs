use crate::ast::ast::Expression;
use crate::ast::expressions::prefix_expression::create_prefix_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;

pub fn parse_prefix_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();
    let operator = token.value.clone();

    parser.next_token(); // 离开前缀运算符

    let right = {
        match parser.parse_expression(Precedence::Prefix) {
            Some(expr) => expr,
            None => {
                parser
                    .errors
                    .push(format!("expected expression after '{}'", operator));
                return None;
            }
        }
    };

    Some(Box::new(create_prefix_expression(token, operator, right)))
}
