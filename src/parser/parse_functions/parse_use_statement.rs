use crate::ast::stmt::Statement;
use crate::ast::expressions::identifier::create_identifier;
use crate::ast::statements::use_statement::create_use_statement;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::Ident;
use crate::token::token_type::TokenType::Semicolon;

pub fn parse_use_statement(parser: &mut Parser) -> Option<Statement> {
    let token = parser.cur_token.clone();
    let ident;

    parser.next_token();

    // 检查当前词法单元是否为标识符
    if !parser.expect_cur(Ident) {
        return None;
    }

    // 设置标识符
    ident = create_identifier(parser.cur_token.clone(), parser.cur_token.value.clone());

    if !parser.peek_token_is(TokenType::As) {
        parser.next_token();
        return Some(Statement::UseStatement(create_use_statement(token, ident, None)));
    }

    // 前进，脱离标识符
    parser.next_token();

    if !parser.expect_cur(TokenType::As) {
        return None
    }

    // 前进，脱离词法单元 As
    parser.next_token();

    // 检查当前词法单元是否为标识符
    if !parser.expect_cur(Ident) {
        return None;
    }

    // 设置标识符
    let alias = create_identifier(parser.cur_token.clone(), parser.cur_token.value.clone());

    if parser.peek_token_is(Semicolon) {
        parser.next_token();
        return Some(Statement::UseStatement(create_use_statement(token, ident, Some(alias))));
    }

    Some(Statement::UseStatement(create_use_statement(token, ident, Some(alias))))
}
