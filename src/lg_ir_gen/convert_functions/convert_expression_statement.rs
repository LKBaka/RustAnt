use crate::{ast::ast::{ExpressionStatement, Node}, convert_type, lg_ir_gen::converter::LgIrConverter};

pub fn convert_expression_statement(
    converter: &mut LgIrConverter,
    node: Box<dyn Node>
) -> Result<(), String> {
    let expr_stmt = convert_type!(ExpressionStatement, node);

    if let Some(expr) = expr_stmt.expression {
        converter.convert(expr)?;
    } else {
        return Err(String::from("expected an expression"));
    }

    Ok(())
}