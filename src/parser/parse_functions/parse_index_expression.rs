use crate::ast::expr::Expression;
use crate::ast::expressions::index_expression::create_index_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;

pub fn parse_index_expression(
    parser: &mut Parser,
    left: Expression,
) -> Option<Expression> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开左括号

    let index = if let Some(it) = parser.parse_expression(Precedence::Lowest) {
        it
    } else {
        parser.push_err(format!(
            "missing index.",
        ));

        return None;
    };

    parser.next_token(); // 前进到右括号

    Some(Expression::IndexExpression(create_index_expression(token, Box::new(left), Box::new(index))))
}
