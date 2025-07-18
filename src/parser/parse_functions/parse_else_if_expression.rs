use crate::ast::expressions::if_expression::create_else_if_expression;
use crate::parser::parse_functions::parse_block_statement::parse_block_statement;
use crate::parser::precedence::Precedence;

use crate::ast::ast::Expression;
use crate::parser::parser::Parser;


pub fn parse_else_if_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开 if 词法单元

    let condition = match parser.parse_expression(Precedence::Lowest) {
        Some(expr) => expr,
        None => {
            parser.errors.push(
                format!(
                    "missing condition. at file <{}>, line {}",
                    parser.cur_token.file, parser.cur_token.line
                )
            );
            return None;
        }
    };

    parser.next_token(); // 离开表达式 (正常应跳转到左大括号)

    let block = match parse_block_statement(parser) { 
        Some(block) => block,
        None => {
            parser.errors.push(
                format!(
                    "missing else if body. at file <{}>, line {}",
                    parser.cur_token.file, parser.cur_token.line
                )
            ); return None;
        }
    };

    Some(Box::new(create_else_if_expression(
        token, condition, block
    )))
}