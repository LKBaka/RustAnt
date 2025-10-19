use crate::{
    ast::{ast::Node, expr::Expression},
    byte_code_vm::{code::code::OP_HASH, compiler::compiler::{CompileError, Compiler}},
};

pub fn compile_hash_literal(compiler: &mut Compiler, node: Node) -> Result<(), CompileError> {
    let hash_literal = match match node {
        Node::Expression(expr) => expr,
        _ => panic!()
    } {
        Expression::HashLiteral(it) => it,
        _ => panic!()
    };

    let items_len = hash_literal.pairs.len() * 2;

    for (k, v) in hash_literal.pairs {
        if let Err(msg) = compiler.compile_expr(*k) {
            return Err(CompileError::from_none_token(
                format!("error compile key: \n{msg}")
            ))
        }

        if let Err(msg) = compiler.compile_expr(*v) {
            return Err(CompileError::from_none_token(
                format!("error compile value: \n{msg}")
            ))
        }
    }

    compiler.emit(OP_HASH, vec![items_len as u16]);

    Ok(())
}
