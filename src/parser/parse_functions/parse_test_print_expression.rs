use crate::ast::ast::Expression;
use crate::ast::expressions::test_print_expression::create_test_print_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;

pub fn parse_test_print_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开 TestPrint 词法单元

    let to_print_expr = if let Some(it) = parser.parse_expression(Precedence::Lowest) {
        it
    } else {
        parser.push_err(format!(
            "missing expression to print.",
        ));
        return None;
    };

    Some(Box::new(create_test_print_expression(token, to_print_expr)))
}
