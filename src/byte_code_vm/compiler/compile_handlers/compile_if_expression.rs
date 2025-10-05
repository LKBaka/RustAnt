use core::panic;

use crate::{
    ast::{
        ast::{INode, Node}, expr::Expression
    },
    byte_code_vm::{
        code::code::{OP_JUMP, OP_JUMP_NOT_TRUTHY, OP_NONE, OP_POP, OP_SET_FIELD},
        compiler::compiler::{CompileError, Compiler},
        constants::FAKE_OFFSET_JUMP,
    },
};

pub fn compile_if_expression(compiler: &mut Compiler, node: Node) -> Result<(), CompileError> {
    let if_expr = match match node {
        Node::Expression(expr) => expr,
        _ => panic!()
    } {
        Expression::IfExpression(it) => it,
        _ => panic!()
    };

    let condition_result = compiler.compile_expr(*if_expr.condition);
    if let Err(msg) = condition_result {
        return Err(CompileError::from_none_token(
            format!("error compile condition: {}", msg)
        ));
    }

    // 先插入一个 OpJumpNotTruthy, 后面再修改他的操作数
    let jump_not_truthy_command_pos = compiler.emit(OP_JUMP_NOT_TRUTHY, vec![FAKE_OFFSET_JUMP]);

    let consequence_result = compiler.compile_stmt(if_expr.consequence);
    if let Err(msg) = consequence_result {
        return Err(CompileError::from_none_token(
            format!("error compile consequence: {}", msg)
        ));
    }

    if compiler.last_instruction_is(OP_POP) {
        compiler.remove_last_pop();
    }

    if compiler.last_instruction_is(OP_SET_FIELD) {
        compiler.emit(OP_NONE, vec![]);
    }

    // 只有 if
    if (&if_expr.alternative).is_none()
        && (if_expr.else_if_expressions.as_ref()).is_none_or(|exprs| (&exprs).is_empty())
    {
        // 回到 OpJumpNotTruthy 纪元 并且修改它的操作数

        // after_all_pos: 指向在编译完 所有的 if 和 else if 块之后的指令坐标
        let after_all_pos = compiler.current_instructions().borrow().len();

        compiler.change_operand(jump_not_truthy_command_pos, after_all_pos as u16);

        compiler.emit(OP_NONE, vec![]);

        return Ok(());
    }

    // jump_to_end_command_pos: 指向在编译完 所有的东西之后 (比如 else, else if) 的指令坐标
    let jump_to_end_command_pos = compiler.emit(OP_JUMP, vec![FAKE_OFFSET_JUMP]);

    let mut else_if_jump_to_end_command_pos: Vec<usize> = vec![];

    let instructions_length: u16 = {
        let len = compiler.current_instructions().borrow().len();
        len as u16
    };

    compiler.change_operand(jump_not_truthy_command_pos, instructions_length);

    if let Some(expressions) = if_expr.else_if_expressions {
        for else_if in expressions {
            let else_if = match *else_if {
                Expression::ElseIfExpression(it) => it,
                _ => return Err(CompileError::from(
                    format!("cannot convert '{}' to else if expression", else_if.to_string()),
                    Some(else_if.token())
                ))
            };

            // 编译else if条件
            let cond_result = compiler.compile_expr(*else_if.condition);
            if let Err(msg) = cond_result {
                return Err(CompileError::from_none_token(
                    format!("error compile else-if condition: {}", msg)
                ));
            }

            // 发出条件跳转指令（稍后回填）
            let else_if_jump_pos = compiler.emit(OP_JUMP_NOT_TRUTHY, vec![FAKE_OFFSET_JUMP]);

            // 编译else if主体块
            let block_result = compiler.compile_stmt(else_if.consequence);
            if let Err(msg) = block_result {
                return Err(CompileError::from_none_token(
                    format!("error compile else-if block: {}", msg)
                ));
            }

            // 移除最后的pop指令（如果有）
            if compiler.last_instruction_is(OP_POP) {
                compiler.remove_last_pop();
            }

            if compiler.last_instruction_is(OP_SET_FIELD) {
                compiler.emit(OP_NONE, vec![]);
            }

            // 发出跳过后续块的无条件跳转
            let jump_pos = compiler.emit(OP_JUMP, vec![FAKE_OFFSET_JUMP]);
            else_if_jump_to_end_command_pos.push(jump_pos);

            // 回填当前else if的条件跳转目标

            let instructions_length: u16 = {
                let len = compiler.current_instructions().borrow().len();
                len as u16
            };

            compiler.change_operand(else_if_jump_pos, instructions_length);
        }
    }

    // 有 if - else 但是没有 else if
    if let Some(alternative) = if_expr.alternative {
        // 编译 else 块

        let alternative_result = compiler.compile_stmt(alternative);
        if let Err(msg) = alternative_result {
            return Err(CompileError::from_none_token(
                format!("error compile alternative: {}", msg)
            ));
        }

        if compiler.last_instruction_is(OP_POP) {
            compiler.remove_last_pop();
        }

        if compiler.last_instruction_is(OP_SET_FIELD) {
            compiler.emit(OP_NONE, vec![]);
        }

        let end_pos = compiler.current_instructions().borrow().len();

        compiler.change_operand(jump_to_end_command_pos, end_pos as u16);

        // 回填 else if 无条件跳转
        for pos in else_if_jump_to_end_command_pos {
            compiler.change_operand(pos, end_pos as u16);
        }

        return Ok(());
    }

    // 有 if 和 else if 但是没有 else

    // 回填 else if 无条件跳转
    let end_pos = compiler.current_instructions().borrow().len();

    for pos in else_if_jump_to_end_command_pos {
        compiler.change_operand(pos, end_pos as u16);
    }

    compiler.emit(OP_NONE, vec![]);

    Ok(())
}
