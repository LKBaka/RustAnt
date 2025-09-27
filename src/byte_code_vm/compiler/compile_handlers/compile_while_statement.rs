use crate::{
    ast::{ast::Node, stmt::Statement},
    byte_code_vm::{
        code::code::{OP_JUMP, OP_JUMP_NOT_TRUTHY, OP_NOP},
        compiler::compiler::{CompileError, Compiler},
        constants::FAKE_OFFSET_JUMP,
    },
};

pub fn compile_while_statement(compiler: &mut Compiler, node: Node) -> Result<(), CompileError> {
    let start_ip = compiler.current_instructions().borrow().len();

    let while_stmt = match match node {
        Node::Statement(stmt) => stmt,
        _ => panic!()
    } {
        Statement::WhileStatement(it) => it,
        _ => panic!()
    };

    if let Err(msg) = compiler.compile_expr(*while_stmt.condition) {
        return Err(CompileError::from_none_token(
            format!("error compile while loop condition: {msg}")
        ));
    }

    let jump_not_truthy_command_pos = compiler.emit(OP_JUMP_NOT_TRUTHY, vec![FAKE_OFFSET_JUMP]);

    if let Err(msg) = compiler.compile_stmt(Statement::BlockStatement(while_stmt.block)) {
        return Err(CompileError::from_none_token(
            format!("error compile while loop body: {msg}")
        ));
    }

    compiler.emit(OP_JUMP, vec![start_ip as u16]);

    let while_loop_end = compiler.current_instructions().borrow().len() as u16;

    // 回填 OP_JUMP_NOT_TRUTHY 的 操作数
    compiler.change_operand(jump_not_truthy_command_pos, while_loop_end);

    // 回填 break 的操作数
    for pos in compiler.break_command_pos.clone() {
        compiler.change_operand(pos, while_loop_end);
    }

    // 回填 continue 的操作数
    for pos in compiler.continue_command_pos.clone() {
        compiler.change_operand(pos, start_ip as u16);
    }

    compiler.emit(OP_NOP, vec![]);

    Ok(())
}
