use crate::ast::ast::Expression;
use crate::ast::expressions::if_expression::create_if_expression;
use crate::parser::parse_functions::parse_block_statement::parse_block_statement;
use crate::parser::parse_functions::parse_else_if_expression::parse_else_if_expression;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;
use crate::token::token_type::TokenType::{Else, Eol, If};

pub fn parse_if_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();
    
    // 解析条件表达式
    parser.next_token(); // 离开 if 关键字
    let condition = parser.parse_expression(Precedence::Lowest).unwrap();

    parser.next_token(); // 离开条件语句

    // 跳过换行符
    while parser.cur_token_is(Eol) {
        parser.next_token();
    }
    
    // 解析默认分支
    let consequence = parse_block_statement(parser);
    parser.next_token(); // 离开右大括号

    if let Some(consequence) = consequence {
        let mut else_if_expressions = Vec::new();
        let mut alternative = None;

        // 处理 else if 和 else 分支
        while parser.cur_token_is(Else) {
            let else_token = parser.cur_token.clone();
            parser.next_token(); // 离开 else 关键字

            // 检查是否是 else if
            if parser.cur_token_is(If) {
                parser.next_token(); // 离开 if 关键字
                
                if let Some(else_if_expr) = parse_else_if_expression(parser) {
                    else_if_expressions.push(else_if_expr);
                }
            } else {
                // 处理普通的 else 分支
                // 跳过换行符
                while parser.cur_token_is(Eol) {
                    parser.next_token();
                }

                alternative = parse_block_statement(parser);
                parser.next_token(); // 离开右大括号
                break; // 遇到 else 分支后不再继续处理
            }
        }

        Some(Box::new(create_if_expression(
            token,
            condition,
            consequence,
            alternative,
            if else_if_expressions.is_empty() { None } else { Some(else_if_expressions) }
        )))
    } else {
        None
    }
} 