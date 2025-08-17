use crate::{
    ast::{ast::Node, statements::while_statement::WhileStatement},
    byte_code_vm::{
        code::code::{OP_JUMP, OP_JUMP_NOT_TRUTHY, OP_NOP},
        compiler::compiler::Compiler,
        constants::FAKE_OFFSET_JUMP,
    },
    convert_type,
};

pub fn compile_while_statement(compiler: &mut Compiler, node: Box<dyn Node>) -> Result<(), String> {
    let start_ip = compiler.current_instructions().borrow().len();

    let while_stmt = convert_type!(WhileStatement, node);

    if let Err(msg) = compiler.compile(while_stmt.condition) {
        return Err(format!("error compile while loop condition: {msg}"));
    }

    let jump_not_truthy_command_pos = compiler.emit(OP_JUMP_NOT_TRUTHY, vec![FAKE_OFFSET_JUMP]);

    if let Err(msg) = compiler.compile(Box::new(while_stmt.block)) {
        return Err(format!("error compile while loop body: {msg}"));
    }

    compiler.emit(OP_JUMP, vec![start_ip as u16]);

    let jump_not_truthy_to = compiler.current_instructions().borrow().len();

    // 回填 OP_JUMP_NOT_TRUTHY 的 操作数
    compiler.change_operand(jump_not_truthy_command_pos, jump_not_truthy_to as u16);

    compiler.emit(OP_NOP, vec![]);

    Ok(())
}
