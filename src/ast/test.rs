#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    #[test]
    fn test_print_nodes() {
        use crate::ast::ast::create_expression_statement;
        use crate::ast::expressions::integer_literal::create_integer_literal;
        use crate::ast::utils::print_nodes;
        use crate::token::token::Token;
        use crate::token::token_type::TokenType;

        let nodes = vec![
            create_expression_statement(
                create_integer_literal(
                    Token::new(TokenType::Integer, "91".to_string(), "__test_print_nodes__".to_string(), -1),
                    BigDecimal::from(91)
                )
            ),
            create_expression_statement(
                create_integer_literal(
                    Token::new(TokenType::Integer, "78".to_string(), "__test_print_nodes__".to_string(), -1),
                    BigDecimal::from(78)
                )
            ),
        ];

        print_nodes(nodes);
    }
}