use crate::ast::ast::INode;
use crate::ast::stmt::Statement;
use crate::ast::statements::while_statement::create_while_statement;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;
use crate::token::token_type::TokenType::LBrace;

use super::parse_block_statement::parse_block_statement;

pub fn parse_while_statement(parser: &mut Parser) -> Option<Statement> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开 while 词法单元

    // 条件
    let condition = parser.parse_expression(Precedence::Lowest);
    if let Option::None = condition {
        parser.push_err(format!(
            "missing condition",
        ));

        return None;
    }

    if !parser.expect_peek(LBrace) {
        return None;
    }

    parser.next_token(); // 离开条件表达式

    let block = parse_block_statement(parser);
    if block.is_none() {
        parser.push_err(format!(
            "missing while body.",
        ));
        return None;
    }

    Some(Statement::WhileStatement(create_while_statement(
        token,
        Box::new(condition.unwrap()),
        match block.as_ref().unwrap() {
            Statement::BlockStatement(it) => it.clone(),
            _ => panic!("expected an block, got: {}", block.unwrap().to_string())
        }
    )))
}
