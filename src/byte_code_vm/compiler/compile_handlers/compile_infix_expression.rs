use crate::{
    ast::{ast::Node, expressions::infix_expression::InfixExpression},
    byte_code_vm::{
        code::code::{INFIX_OPERATOR_TO_OPCODE, OP_GT},
        compiler::compiler::Compiler,
    },
    convert_type,
};

pub fn compile_infix_expression(
    compiler: &mut Compiler,
    node: Box<dyn Node>,
) -> Result<(), String> {
    let infix_expr = convert_type!(InfixExpression, node);

    if infix_expr.operator == "<" {
        if let Err(right_err) = compiler.compile(infix_expr.right) {
            return Err(format!("error compiling right expression: {}", right_err));
        };

        if let Err(left_err) = compiler.compile(infix_expr.left) {
            return Err(format!("error compiling left expression: {}", left_err));
        };

        compiler.emit(OP_GT, vec![]);

        return Ok(());
    }

    if let Err(left_err) = compiler.compile(infix_expr.left) {
        return Err(format!("error compiling left expression: {}", left_err));
    };

    if let Err(right_err) = compiler.compile(infix_expr.right) {
        return Err(format!("error compiling right expression: {}", right_err));
    };

    let operator_opcode = INFIX_OPERATOR_TO_OPCODE.get(&infix_expr.operator);

    if let Some(op) = operator_opcode {
        compiler.emit(*op, vec![]);
        Ok(())
    } else {
        Err(format!("unknown operator: {}", infix_expr.operator))
    }
}
