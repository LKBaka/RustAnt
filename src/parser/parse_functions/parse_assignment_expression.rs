use crate::ast::ast::Expression;
use crate::ast::expressions::assignment_expression::create_assignment_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence::Lowest;

pub fn parse_assignment_expression(
    parser: &mut Parser,
    left: Box<dyn Expression>,
) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();
    let left_expression = left;

    parser.next_token(); // 离开等于号

    let value = parser.parse_expression(Lowest);

    if let Some(value) = value {
        return Some(Box::new(create_assignment_expression(
            token,
            left_expression,
            value,
        )));
    }

    parser.errors.push(format!(
        "missing expression. at file <{}>, line {}",
        token.file, token.line
    ));
    None
}
