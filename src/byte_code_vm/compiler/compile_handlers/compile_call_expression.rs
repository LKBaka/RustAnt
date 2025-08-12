use crate::{ast::{ast::Node, expressions::call_expression::CallExpression}, byte_code_vm::{code::code::OP_CALL, compiler::compiler::Compiler}, convert_type};

pub fn compile_call_expression(
    compiler: &mut Compiler,
    node: Box<dyn Node>
) -> Result<(), String> {
    let call_expr = convert_type!(CallExpression, node);

    if let Err(msg) = compiler.compile(call_expr.func) {
        return Err(format!("error compile call expresion: {msg}"))
    }

    // WARNING: 后面滚回来写参数处理!

    compiler.emit(OP_CALL, vec![]);

    Ok(())
}