use crate::{
    ast::{ast::Node, expr::Expression},
    byte_code_vm::{code::code::PREFIX_OPERATOR_TO_OPCODE, compiler::compiler::Compiler},
};

pub fn compile_prefix_expression(
    compiler: &mut Compiler,
    node: Node,
) -> Result<(), String> {
    let prefix_expr = match match node {
        Node::Expression(expr) => expr,
        _ => panic!()
    } {
        Expression::PrefixExpression(it) => it,
        _ => panic!()
    };

    let result = compiler.compile_expr(*prefix_expr.expression);
    if result.is_err() {
        return result;
    }

    let op_code = if let Some(op) = PREFIX_OPERATOR_TO_OPCODE.get(&prefix_expr.operator) {
        *op
    } else {
        return Err(format!("unknown prefix operator: {}", prefix_expr.operator));
    };

    compiler.emit(op_code, vec![]);

    Ok(())
}
