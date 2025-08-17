use crate::ast::expressions::if_expression::create_if_expression;
use crate::parser::parse_functions::parse_block_statement::parse_block_statement;
use crate::parser::parse_functions::parse_else_if_expression::parse_else_if_expression;
use crate::parser::precedence::Precedence;
use crate::token::token_type::TokenType;

use crate::ast::ast::Expression;
use crate::parser::parser::Parser;

pub fn parse_if_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开 if 词法单元

    let condition = match parser.parse_expression(Precedence::Lowest) {
        Some(expr) => expr,
        None => {
            parser.errors.push(format!(
                "missing condition. at file <{}>, line {}",
                parser.cur_token.file, parser.cur_token.line
            ));
            return None;
        }
    };

    parser.next_token(); // 离开表达式 (正常应跳转到左大括号)

    let consequence = match parse_block_statement(parser) {
        Some(block) => block,
        None => {
            parser.errors.push(format!(
                "missing if body. at file <{}>, line {}",
                parser.cur_token.file, parser.cur_token.line
            ));
            return None;
        }
    };

    let mut else_if_expressions = vec![];

    while parser.peek_token_is(TokenType::Else) {
        parser.next_token(); // 前进到 else 词法单元

        if parser.peek_token_is(TokenType::If) {
            parser.next_token(); // 前进到 if 词法单元

            let else_if_expression = parse_else_if_expression(parser);
            if let Some(else_if_expression) = else_if_expression {
                else_if_expressions.push(else_if_expression);
            }

            continue;
        }

        // 处理 else
        parser.next_token(); // 前进到左大括号

        let else_block = match parse_block_statement(parser) {
            Some(block) => block,
            None => {
                parser.errors.push(format!(
                    "missing else body. at file <{}>, line {}",
                    parser.cur_token.file, parser.cur_token.line
                ));
                return None;
            }
        };

        return Some(Box::new(create_if_expression(
            token,
            condition,
            consequence,
            Some(else_block),
            Some(else_if_expressions),
        )));
    }

    Some(Box::new(create_if_expression(
        token,
        condition,
        consequence,
        None,
        Some(else_if_expressions),
    )))
}
