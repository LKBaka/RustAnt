use crate::{
    ast::{ast::Node, expr::Expression},
    byte_code_vm::{code::code::OP_CALL, compiler::compiler::{CompileError, Compiler}},
};

pub fn compile_call_expression(compiler: &mut Compiler, node: Node) -> Result<(), CompileError> {
    let call_expr = match match node {
        Node::Expression(expr) => expr,
        _ => panic!()
    } {
        Expression::CallExpression(it) => it,
        _ => panic!()
    };

    if let Err(msg) = compiler.compile_expr(*call_expr.func) {
        return Err(CompileError::from_none_token(
            format!("error compile call expresion: {msg}")
        ));
    }

    let args_len = call_expr.args.len();

    for arg in call_expr.args {
        if let Err(msg) = compiler.compile_expr(*arg) {
            return Err(CompileError::from_none_token(
                format!("error compile args: {msg}")
            ));
        }
    }

    compiler.emit(OP_CALL, vec![args_len as u16]);

    Ok(())
}
