use lg_rust_binding::ir::{types::{IRIntegerType, IRIntegerTypeSize}, IRConstantPoolEntry};

use crate::{ast::{ast::Node, expressions::integer_literal::IntegerLiteral}, convert_type, lg_ir_gen::converter::LgIrConverter};

pub fn convert_integer_literal(
    converter: &mut LgIrConverter,
    node: Box<dyn Node>
) -> Result<(), String> {
    let integer_literal = convert_type!(IntegerLiteral, node);

    converter.ir_module().constant_pool.entries.push(Box::new(IRConstantPoolEntry::new(
        Box::new(IRIntegerType::new(IRIntegerTypeSize::FourBytes, false)),
        Box::new(integer_literal.value),
    )));

    Ok(())
}