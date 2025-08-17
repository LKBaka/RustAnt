use std::any::Any;

use crate::ast::ast::Statement;
use crate::ast::expressions::identifier::Identifier;
use crate::ast::statements::block_statement::BlockStatement;
use crate::ast::statements::class_statement::create_class_statement;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType;

use super::parse_block_statement::parse_block_statement;

pub fn parse_class_statement(parser: &mut Parser) -> Option<Box<dyn Statement>> {
    let token = parser.cur_token.clone();

    let mut parent_class_ident = None;

    parser.next_token(); // 离开 class 词法单元

    if !parser.expect_cur(TokenType::Ident) {
        return None;
    }

    let class_ident = Some(Identifier {
        token: parser.cur_token.clone(),
        value: parser.cur_token.value.clone(),
    });

    parser.next_token(); // 离开标识符

    if parser.cur_token_is(TokenType::Colon) {
        parser.next_token(); // 离开冒号

        if !parser.expect_cur(TokenType::Ident) {
            parser.errors.push(format!(
                "missing parent class name. at file <{}>, line {}",
                parser.cur_token.file, parser.cur_token.line
            ));
            return None;
        }

        // 处理父类标识符
        parent_class_ident = Some(Identifier {
            token: parser.cur_token.clone(),
            value: parser.cur_token.value.clone(),
        });

        parser.next_token(); // 离开父类标识符 (正常应前进到左大括号)
    }

    let block = parse_block_statement(parser);

    match block {
        Some(block) => {
            let class_ident = if let Some(ident) = class_ident {
                ident
            } else {
                parser.errors.push(format!(
                    "missing class name. at file <{}>, line {}",
                    parser.cur_token.file, parser.cur_token.line
                ));

                return None;
            };

            if let Some(block) = (block as Box<dyn Any>).downcast_ref::<BlockStatement>() {
                Some(Box::new(create_class_statement(
                    token,
                    class_ident,
                    parent_class_ident,
                    block.clone(),
                )))
            } else {
                None
            }
        }
        None => None,
    }
}
