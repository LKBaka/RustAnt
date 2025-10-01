#[cfg(test)]
mod tests {
    use crate::ast::{expr::Expression, expressions::integer64_literal::Int64Literal, stmt::Statement};


    #[test]
    fn test_conversion() {
        use crate::ast::ast::ExpressionStatement;
        use crate::token::{token::Token, token_type::TokenType};

        use crate::lg_ir_gen::converter::LgIrConverter;
        use crate::ast::ast::Program;

        let file = "__test_conversion__".to_string();

        let integer_token = Token::new(TokenType::Integer64, "1".into(), file, 1, 1);

        let i64_literal = Int64Literal {
            token: integer_token.clone(),
            value: 1,
        };

        let program = Program {
            token: integer_token,
            statements: vec![Statement::ExpressionStatement(ExpressionStatement {
                expression: Some(Box::new(Expression::Int64Literal(i64_literal))),
            })],
        };

        let mut converter = LgIrConverter::new(program);
        let result = converter.start_convert();

        assert_eq!(result.is_ok(), true, "Conversion failed: {:?}", result.err());

        println!("{}", converter.ir_module().to_string())
    }
}