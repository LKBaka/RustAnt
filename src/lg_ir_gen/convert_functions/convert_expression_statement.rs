use crate::{
    ast::ast::{ExpressionStatement, INode},
    convert_type_ref,
    lg_ir_gen::converter::LgIrConverter,
};

pub fn convert_expression_statement(
    converter: &mut LgIrConverter,
    node: &dyn INode,
) -> Result<(), String> {
    let expr_stmt = convert_type_ref!(ExpressionStatement, node);

    if let Some(expr) = expr_stmt.expression {
        converter.convert(&*expr)?;
    } else {
        return Err(String::from("expected an expression"));
    }

    Ok(())
}
