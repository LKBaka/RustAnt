use crate::ast::ast::Expression;
use crate::ast::expressions::return_expression::create_return_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;

pub fn parse_return_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开 return 词法单元

    let expr = parser.parse_expression(Precedence::Lowest);

    if let Some(it) = expr {
        Some(
            Box::new(
                create_return_expression(token, it)
            )
        )
    } else {
        parser.errors.push(
            format!("missing expression. at file <{}>, line {}", token.file, token.line)
        ); None
    }
}