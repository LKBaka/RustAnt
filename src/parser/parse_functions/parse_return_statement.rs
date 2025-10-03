use crate::ast::statements::return_statement::create_return_statement;
use crate::ast::stmt::Statement;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;
use crate::token::token_type::TokenType;

pub fn parse_return_statement(parser: &mut Parser) -> Option<Statement> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开 return 词法单元

    let expr = parser.parse_expression(Precedence::Lowest);

    if parser.peek_token_is(TokenType::Semicolon) {
        parser.next_token();
    }

    if let Some(it) = expr {
        Some(Statement::ReturnStatement(create_return_statement(token, Box::new(it))))
    } else {
        parser.push_err(format!(
            "missing return value.",
        ));
        None
    }
}
