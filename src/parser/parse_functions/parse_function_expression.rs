use std::any::Any;

use crate::ast::ast::INode;
use crate::ast::expressions::identifier::Identifier;
use crate::ast::stmt::Statement;
use crate::parser::parse_functions::parse_block_statement::parse_block_statement;
use crate::ast::expr::Expression;
use crate::token::token_type::TokenType;

use crate::ast::expressions::function_expression::create_function_expression;
use crate::parser::parser::Parser;

pub fn parse_function_expression(parser: &mut Parser) -> Option<Expression> {
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

    parser.next_token(); // 离开右括号 (正常应前进到左大括号 或者 '->' )

    let mut ret_type = None;

    if parser.cur_token_is(TokenType::Minus) {
        if !parser.expect_peek(TokenType::Gt) {
            return None
        }

        parser.next_token(); // 前进到 >

        if !parser.expect_peek(TokenType::Ident) {
            return None;
        }

        parser.next_token(); // 前进到 Ident

        ret_type = Some(Identifier {
            token: parser.cur_token.clone(),
            value: parser.cur_token.value.clone()
        });

        parser.next_token(); // 理应前进到左大括号
    }

    let block = parse_block_statement(parser);

    match block {
        Some(block) => {
            let converted_block = match block {
                Statement::BlockStatement(block) => block,
                _ => panic!("{}", &format!(
                    "expected block statement, got {:?}, ast to string: {}",
                    block.type_id(),
                    block.to_string()
                ))
            };

            Some(Expression::FunctionExpression(create_function_expression(
                token,
                name,
                params,
                converted_block,
                ret_type,
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
