use crate::ast::ast::Expression;
use crate::ast::expressions::class_member_expression::create_class_member_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;

pub fn parse_class_member_expression(
    parser: &mut Parser,
    left: Box<dyn Expression>,
) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();
    let left_expression = left;

    parser.next_token(); // 离开双冒号 (::)

    let right_expression = parser.parse_expression(Precedence::ObjMember);

    match right_expression {
        None => {
            parser.push_err(format!("missing expression.",));
            None
        }
        Some(right_expression) => Some(Box::new(create_class_member_expression(
            token,
            left_expression,
            right_expression,
        ))),
    }
}
