use crate::ast::ast::Expression;
use crate::ast::expressions::index_expression::create_index_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;


pub fn parse_index_expression(parser: &mut Parser, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开左括号

    let index  = if let Some(it) = parser.parse_expression(Precedence::Lowest) {
        it
    } else {
        parser.errors.push(format!(
            "missing index. at file <{}>, line {}",
            parser.cur_token.file, parser.cur_token.line
        ));

        return None;
    };

    parser.next_token(); // 前进到右括号

    Some(Box::new(
        create_index_expression(token, left, index)
    ))
}