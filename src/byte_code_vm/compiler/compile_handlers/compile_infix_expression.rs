use crate::{
    ast::{ast::Node, expr::Expression},
    byte_code_vm::{
        code::code::{INFIX_OPERATOR_TO_OPCODE, OP_GT},
        compiler::compiler::{CompileError, Compiler},
    },
};

pub fn compile_infix_expression(
    compiler: &mut Compiler,
    node: Node,
) -> Result<(), CompileError> {
    let infix_expr = match match node {
        Node::Expression(expr) => expr,
        _ => panic!()
    } {
        Expression::InfixExpression(it) => it,
        _ => panic!()
    };

    if infix_expr.operator.value == "<" {
        if let Err(right_err) = compiler.compile_expr(*infix_expr.right) {
            return Err(CompileError::from_none_token(
                format!("error compiling right expression: {}", right_err)
            ));
        };

        if let Err(left_err) = compiler.compile_expr(*infix_expr.left) {
            return Err(CompileError::from_none_token(
                format!("error compiling left expression: {}", left_err)
            ));
        };

        compiler.emit(OP_GT, vec![]);

        return Ok(());
    }

    if let Err(left_err) = compiler.compile_expr(*infix_expr.left) {
        return Err(CompileError::from_none_token(
            format!("error compiling left expression: {}", left_err)
        ));
    };

    if let Err(right_err) = compiler.compile_expr(*infix_expr.right) {
        return Err(CompileError::from_none_token(
            format!("error compiling right expression: {}", right_err)
        ));
    };

    let operator_opcode = INFIX_OPERATOR_TO_OPCODE.get(&&infix_expr.operator.value.to_lowercase());

    if let Some(op) = operator_opcode {
        compiler.emit(*op, vec![]);
        Ok(())
    } else {
        Err(CompileError::from(
            format!("unknown operator: {}", infix_expr.operator.value),
            Some(infix_expr.operator)
        ))
    }
}
