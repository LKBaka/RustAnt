use crate::ast::expr::Expression;
use crate::ast::expressions::assignment_expression::create_assignment_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence::Lowest;

pub fn parse_assignment_expression(
    parser: &mut Parser,
    left: Expression,
) -> Option<Expression> {
    let token = parser.cur_token.clone();
    let left_expression = left;

    parser.next_token(); // 离开等于号

    let value = parser.parse_expression(Lowest);

    if let Some(value) = value {
        return Some(Expression::AssignmentExpression(create_assignment_expression(
            token,
            Box::new(left_expression),
            Box::new(value),
        )));
    }

    parser.push_err(format!(
        "missing expression.",
    ));
    None
}
