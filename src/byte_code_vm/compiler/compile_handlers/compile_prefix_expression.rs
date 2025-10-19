use crate::{
    ast::{ast::Node, expr::Expression},
    byte_code_vm::{code::code::PREFIX_OPERATOR_TO_OPCODE, compiler::compiler::{CompileError, Compiler}},
};

pub fn compile_prefix_expression(
    compiler: &mut Compiler,
    node: Node,
) -> Result<(), CompileError> {
    let prefix_expr = match match node {
        Node::Expression(expr) => expr,
        _ => unreachable!()
    } {
        Expression::PrefixExpression(it) => it,
        _ => unreachable!()
    };

    let result = compiler.compile_expr(*prefix_expr.expression);
    if result.is_err() {
        return result;
    }

    let op_code = if let Some(op) = PREFIX_OPERATOR_TO_OPCODE.get(&prefix_expr.operator.value) {
        *op
    } else {
        return Err(CompileError::from(
            format!("unknown prefix operator: \n{}", &prefix_expr.operator.value),
            Some(prefix_expr.operator)
        ));
    };

    compiler.emit(op_code, vec![]);

    Ok(())
}
