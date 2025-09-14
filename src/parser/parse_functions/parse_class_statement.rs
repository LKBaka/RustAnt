use crate::ast::stmt::Statement;
use crate::ast::expressions::identifier::Identifier;
use crate::ast::statements::class_statement::create_class_statement;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType;

use super::parse_block_statement::parse_block_statement;

pub fn parse_class_statement(parser: &mut Parser) -> Option<Statement> {
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
            parser.push_err(format!(
                "missing parent class name",
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
                parser.push_err(format!(
                    "missing class name.",
                ));

                return None;
            };

            if let Statement::BlockStatement(block) = block {
                Some(Statement::ClassStatement(create_class_statement(
                    token,
                    class_ident,
                    parent_class_ident,
                    block,
                )))
            } else {
                None
            }
        }
        None => None,
    }
}
