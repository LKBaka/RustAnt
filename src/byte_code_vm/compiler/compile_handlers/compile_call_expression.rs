use crate::{
    ast::{ast::Node, expressions::call_expression::CallExpression},
    byte_code_vm::{code::code::OP_CALL, compiler::compiler::Compiler},
    convert_type_to_owned,
};

pub fn compile_call_expression(compiler: &mut Compiler, node: Box<dyn Node>) -> Result<(), String> {
    let call_expr = convert_type_to_owned!(CallExpression, node);

    if let Err(msg) = compiler.compile(call_expr.func) {
        return Err(format!("error compile call expresion: {msg}"));
    }

    let args_len = call_expr.args.len();

    for arg in call_expr.args {
        if let Err(msg) = compiler.compile(arg) {
            return Err(format!("err compile args: {msg}"));
        }
    }

    compiler.emit(OP_CALL, vec![args_len as u16]);

    Ok(())
}
