use crate::{
    ast::{ast::Node, expr::Expression},
    byte_code_vm::{code::code::OP_HASH, compiler::compiler::Compiler},
};

pub fn compile_hash_literal(compiler: &mut Compiler, node: Node) -> Result<(), String> {
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
            return Err(format!("error compile key: {msg}"))
        }

        if let Err(msg) = compiler.compile_expr(*v) {
            return Err(format!("error compile value: {msg}"))
        }
    }

    compiler.emit(OP_HASH, vec![items_len as u16]);

    Ok(())
}
