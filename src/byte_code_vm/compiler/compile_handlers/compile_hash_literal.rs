use crate::{
    ast::{ast::Node, expressions::hash_literal::HashLiteral},
    byte_code_vm::{code::code::OP_HASH, compiler::compiler::Compiler},
    convert_type_to_owned,
};

pub fn compile_hash_literal(compiler: &mut Compiler, node: Box<dyn Node>) -> Result<(), String> {
    let hash_literal = convert_type_to_owned!(HashLiteral, node);

    let items_len = hash_literal.pairs.len() * 2;

    for (k, v) in hash_literal.pairs {
        if let Err(msg) = compiler.compile(k) {
            return Err(format!("error compile key: {msg}"))
        }

        if let Err(msg) = compiler.compile(v) {
            return Err(format!("error compile value: {msg}"))
        }
    }

    compiler.emit(OP_HASH, vec![items_len as u16]);

    Ok(())
}
