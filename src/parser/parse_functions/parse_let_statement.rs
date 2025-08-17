use crate::ast::ast::Statement;
use crate::ast::expressions::identifier::create_identifier;
use crate::ast::statements::let_statement::create_let_statement;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;
use crate::token::token_type::TokenType::Ident;
use crate::token::token_type::TokenType::Semicolon;

pub fn parse_let_statement(parser: &mut Parser) -> Option<Box<dyn Statement>> {
    let token = parser.cur_token.clone();
    let ident;

    parser.next_token();

    // 检查当前词法单元是否为标识符
    if !parser.expect_cur(Ident) {
        return None;
    }

    // 设置标识符
    ident = create_identifier(parser.cur_token.clone(), parser.cur_token.value.clone());

    // 前进，脱离标识符
    parser.next_token();

    // 前进，脱离等号
    parser.next_token();

    // 解析表达式
    let temp_value = parser.parse_expression(Precedence::Lowest);

    if parser.peek_token_is(Semicolon) {
        parser.next_token();
    }

    if let Some(value) = temp_value {
        return Some(Box::new(create_let_statement(token, ident, value)));
    }

    parser.errors.push(format!(
        "missing expression. at file <{}>, line {}",
        token.file, token.line
    ));
    None
}
