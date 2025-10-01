use crate::{
    ast::{ast::INode, expressions::return_expression::ReturnExpression},
    convert_type_ref,
    lg_ir_gen::converter::LgIrConverter,
};

pub fn convert_return(
    converter: &mut LgIrConverter,
    node: &dyn INode,
) -> Result<(), String> {
    let ret_expr = convert_type_ref!(ReturnExpression, node);

    converter.convert(&*ret_expr.value)?;

    // ...

    Ok(())
}
