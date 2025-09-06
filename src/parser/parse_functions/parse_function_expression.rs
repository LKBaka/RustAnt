use std::any::Any;

use crate::ast::statements::block_statement::BlockStatement;
use crate::parser::parse_functions::parse_block_statement::parse_block_statement;
use crate::token::token_type::TokenType;

use crate::ast::ast::Expression;
use crate::ast::expressions::function_expression::create_function_expression;
use crate::parser::parser::Parser;

pub fn parse_function_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    let name = if parser.peek_token_is(TokenType::Ident) {
        parser.next_token(); // 前进到标识符

        Some(parser.cur_token.value.clone())
    } else {
        None
    };

    if !parser.expect_peek(TokenType::LParen) {
        return None;
    }

    parser.next_token(); // 前进到左括号

    let params = parser.parse_expression_list(TokenType::RParen);

    parser.next_token(); // 离开右括号 (正常应前进到左大括号)

    let block = parse_block_statement(parser);

    match block {
        Some(block) => {
            let converted_block = (block.as_ref() as &dyn Any)
                .downcast_ref::<BlockStatement>()
                .expect(&format!(
                    "expected block statement, got {:?}, ast to string: {}",
                    block.type_id(),
                    block.to_string()
                ))
                .clone();

            Some(Box::new(create_function_expression(
                token,
                name,
                params,
                converted_block,
            )))
        }
        None => {
            parser.push_err(format!(
                "missing function body.",
            ));
            None
        }
    }
}
