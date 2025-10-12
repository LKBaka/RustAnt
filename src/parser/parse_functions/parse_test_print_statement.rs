use crate::ast::statements::test_print_statement::create_test_print_statement;
use crate::ast::stmt::Statement;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;

pub fn parse_test_print_statement(parser: &mut Parser) -> Option<Statement> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开 TestPrint 词法单元

    let to_print_expr = if let Some(it) = parser.parse_expression(Precedence::Lowest) {
        it
    } else {
        parser.push_err(format!(
            "missing expression to print.",
        ));
        return None;
    };

    Some(Statement::TestPrintStatement(create_test_print_statement(token, Box::new(to_print_expr))))
}
