use std::any::Any;

use crate::ast::ast::Statement;
use crate::ast::expressions::identifier::create_identifier;
use crate::ast::expressions::identifier::Identifier;
use crate::ast::statements::block_statement::BlockStatement;
use crate::ast::statements::class_statement::create_class_statement;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::LBrace;
use crate::token::token_type::TokenType::Ident;

use super::parse_block_statement::parse_block_statement;

pub fn parse_class_statement(parser: &mut Parser) -> Option<Box<dyn Statement>> {
    let mut base: Option<Identifier> = None;
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开 class 词法单元

    // 判断是否为标识符
    if !parser.expect_cur(Ident) {
        return None;
    }

    let ident = create_identifier(parser.cur_token.clone(), parser.cur_token.value.clone());

    parser.next_token(); // 离开标识符

    if parser.cur_token_is(TokenType::Colon) {
        if !parser.expect_peek(Ident) {
            return None;
        }

        parser.next_token(); // 离开冒号   
        parser.next_token(); // 离开标识符     

        base = Some(create_identifier(parser.cur_token.clone(), parser.cur_token.value.clone()));
    }

    if !parser.expect_cur(LBrace) {
        return None;
    }

    parser.next_token(); // 离开大括号

    let block = parse_block_statement(parser);
    if let Option::None = block {
        parser.errors.push(
            format!(
                "missing block. at file <{}>, line {}",
                parser.cur_token.file, parser.cur_token.line
            )
        );

        return None;
    }

    Some(Box::new(
        create_class_statement(
            token, 
            ident,
            base, 
            (block.unwrap() as Box<dyn Any>).downcast_ref::<BlockStatement>()?.to_owned()
        )
    ))
}