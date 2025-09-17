use crate::ast::expr::Expression;
use crate::ast::expressions::infix_expression::create_infix_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::get_token_precedence;

pub fn parse_infix_expression(
    parser: &mut Parser,
    left: Expression,
) -> Option<Expression> {
    let token = parser.cur_token.clone();
    let left_expression = left;
    let operator = parser.cur_token.clone();

    let precedence = get_token_precedence(parser.cur_token.token_type);
    parser.next_token(); // 离开运算符

    let right_expression = parser.parse_expression(if operator.value == "+" {
        precedence - 1
    } else {
        precedence
    });

    match right_expression {
        None => {
            parser.push_err(format!(
                "missing expression."
            ));

            None
        }
        Some(right_expression) => Some(Expression::InfixExpression(create_infix_expression(
            token,
            Box::new(left_expression),
            Box::new(right_expression),
            operator,
        ))),
    }
}
