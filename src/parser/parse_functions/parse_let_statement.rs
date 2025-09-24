use crate::ast::expressions::identifier::create_identifier;
use crate::ast::statements::let_statement::create_let_statement;
use crate::ast::statements::let_statement::create_let_statement_with_type;
use crate::ast::stmt::Statement;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::Ident;
use crate::token::token_type::TokenType::Semicolon;

pub fn parse_let_statement(parser: &mut Parser) -> Option<Statement> {
    let token = parser.cur_token.clone();
    let ident;
    let mut type_hint = None;

    parser.next_token();

    // 检查当前词法单元是否为标识符
    if !parser.expect_cur(Ident) {
        return None;
    }

    // 设置标识符
    ident = create_identifier(parser.cur_token.clone(), parser.cur_token.value.clone());

    // 前进，脱离标识符
    parser.next_token();

    if parser.cur_token_is(TokenType::Colon) {
        // 前进，脱离冒号
        parser.next_token();

        let type_hint_expr = parser.parse_expression(Precedence::Assignment);

        if let Some(type_hint_expr) = type_hint_expr {
            type_hint = Some(Box::new(type_hint_expr));
        } else {
            parser.push_err(format!("missing type hint expression."));
            return None;
        }

        // 前进，脱离表达式
        parser.next_token();
    }

    // 前进，脱离等号
    parser.next_token();

    // 解析表达式
    let temp_value = parser.parse_expression(Precedence::Lowest);

    if parser.peek_token_is(Semicolon) {
        parser.next_token();
    }

    if let Some(value) = temp_value {
        return if let Some(type_hint) = type_hint {
            Some(Statement::LetStatement(create_let_statement_with_type(
                token,
                ident,
                Box::new(value),
                type_hint,
            )))
        } else {
            Some(Statement::LetStatement(create_let_statement(
                token,
                ident,
                Box::new(value),
            )))
        };
    }

    parser.push_err(format!("missing expression."));
    None
}
