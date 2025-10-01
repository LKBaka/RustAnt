use lg_rust_binding::ir::{
    IRConstantPoolEntry,
    types::{IRIntegerType, IRIntegerTypeSize},
};

use crate::{
    ast::{ast::INode, expressions::integer64_literal::Int64Literal},
    convert_type_ref,
    lg_ir_gen::converter::LgIrConverter,
};

pub fn convert_i64_literal(converter: &mut LgIrConverter, node: &dyn INode) -> Result<(), String> {
    let integer_literal = convert_type_ref!(Int64Literal, node);

    converter
        .ir_module()
        .constant_pool
        .entries
        .push(Box::new(IRConstantPoolEntry::new(
            Box::new(IRIntegerType::new(IRIntegerTypeSize::FourBytes, false)),
            Box::new(integer_literal.value),
        )));

    Ok(())
}
