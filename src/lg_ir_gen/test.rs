mod tests {

    #[test]
    fn test_conversion() {
        
        use bigdecimal::BigDecimal;

        use crate::ast::ast::ExpressionStatement;
        use crate::{ast::expressions::integer_literal::IntegerLiteral, token::{token::Token, token_type::TokenType}};

        use crate::lg_ir_gen::converter::LgIrConverter;
        use crate::ast::ast::Program;

        let file = "__test_conversion__".to_string();

        let integer_token = Token::new(TokenType::Integer, "1".into(), file, -1);

        let integer_literal = IntegerLiteral {
            token: integer_token.clone(),
            value: BigDecimal::from(1),
        };

        let program = Program {
            token: integer_token,
            statements: vec![Box::new(ExpressionStatement {
                expression: Some(Box::new(integer_literal)), // Assuming 1 is an integer literal
            })],
        };

        let mut converter = LgIrConverter::new(program);
        let result = converter.start_convert();

        assert_eq!(result.is_ok(), true, "Conversion failed: {:?}", result.err());

        println!("{}", converter.ir_module().to_string())
    }
}