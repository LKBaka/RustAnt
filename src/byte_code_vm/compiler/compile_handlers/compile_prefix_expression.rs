use crate::{ast::{ast::Node, expressions::prefix_expression::PrefixExpression}, byte_code_vm::{code::code::PREFIX_OPERATOR_TO_OPCODE, compiler::compiler::Compiler}, convert_type};

pub fn compile_prefix_expression(
    compiler: &mut Compiler,
    node: Box<dyn Node>
) -> Result<(), String> {
    let prefix_expr = convert_type!(PrefixExpression, node);

    let result = compiler.compile(prefix_expr.expression);
    if result.is_err() {
        return result;
    }

    let op_code = if let Some(op) = PREFIX_OPERATOR_TO_OPCODE.get(&prefix_expr.operator) {
        *op
    } else {
        return Err(format!("Unknown prefix operator: {}", prefix_expr.operator));
    };

    compiler.emit(op_code, vec![]);

    Ok(())
}