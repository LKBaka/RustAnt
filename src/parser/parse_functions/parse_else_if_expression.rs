use crate::ast::ast::Expression;
use crate::ast::expressions::if_expression::create_else_if_expression;
use crate::parser::parse_functions::parse_block_statement::parse_block_statement;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;
use crate::token::token_type::TokenType::Eol;

pub fn parse_else_if_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();
    
    // 解析条件表达式
    let condition = parser.parse_expression(Precedence::Lowest).unwrap();
    parser.next_token(); // 离开条件语句

    // 跳过换行符
    while parser.cur_token_is(Eol) {
        parser.next_token();
    }

    // 解析代码块
    if let Some(consequence) = parse_block_statement(parser) {
        parser.next_token(); // 离开右大括号
        Some(Box::new(create_else_if_expression(
            token,
            condition,
            consequence
        )))
    } else {
        None
    }
} 