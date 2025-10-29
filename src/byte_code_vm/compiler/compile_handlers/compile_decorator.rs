use crate::{
    ast::{
        ast::Node, expr::Expression, expressions::call_expression::CallExpression, stmt::Statement,
    },
    byte_code_vm::{
        code::code::{OP_CALL, OP_POP, OP_SET_GLOBAL, OP_SET_LOCAL},
        compiler::{
            compiler::{CompileError, Compiler},
            symbol_table::symbol_table::SymbolScope,
        },
    },
};

fn compile_decorator_call(
    compiler: &mut Compiler,
    to_decorate: Statement,
    mut it: CallExpression,
) -> Result<(), CompileError> {
    if let Statement::LetStatement(ref let_stmt) = to_decorate {
        it.args.insert(
            0,
            match to_decorate {
                Statement::ExpressionStatement(expr_stmt) => expr_stmt,
                _ => panic!(),
            }
            .expression
            .unwrap(),
        );

        if let Err(msg) = compiler.compile_expr(Expression::CallExpression(it)) {
            return Err(CompileError::from_none_token(format!(
                "error compile decorator: \n{msg}"
            )));
        }

        let symbol = compiler
            .symbol_table
            .borrow_mut()
            .define(&let_stmt.name.value);

        compiler.emit(
            if symbol.scope == SymbolScope::Global {
                OP_SET_GLOBAL
            } else {
                OP_SET_LOCAL
            },
            vec![symbol.index as u16],
        );

        return Ok(());
    }

    if let Statement::ExpressionStatement(ref expr_stmt) = to_decorate
        && let Expression::FunctionExpression(func_expr) =
            expr_stmt.expression.as_ref().unwrap().as_ref()
    {
        let func_name = func_expr.name.clone();

        it.args.insert(
            0,
            match to_decorate {
                Statement::ExpressionStatement(expr_stmt) => expr_stmt,
                _ => panic!(),
            }
            .expression
            .unwrap(),
        );

        if let Err(msg) = compiler.compile_expr(Expression::CallExpression(it)) {
            return Err(CompileError::from_none_token(format!(
                "error compile decorator: \n{msg}"
            )));
        }

        if compiler.last_instruction_is(OP_POP) {
            compiler.remove_last_instruction();
            return Ok(());
        }

        let mut symbol_and_mode = (None, OP_SET_GLOBAL);

        if compiler.last_instruction_is(OP_SET_GLOBAL) {
            compiler.remove_last_instruction();

            func_name.and_then(|it| {
                if let Some(symbol) = compiler.symbol_table.borrow_mut().resolve(&it) {  
                    symbol_and_mode = (Some(symbol.index), OP_SET_GLOBAL);  
                }

                Some(())
            });
        } else if compiler.last_instruction_is(OP_SET_LOCAL) {
            compiler.remove_last_instruction();

            func_name.and_then(|it| {
                if let Some(symbol) = compiler.symbol_table.borrow_mut().resolve(&it) {  
                    symbol_and_mode = (Some(symbol.index), OP_SET_LOCAL);  
                }

                Some(())
            });
        }
        
        compiler.emit(OP_CALL, vec![1u16]);
        
        if let (Some(symbol), mode) = symbol_and_mode {
            compiler.emit(mode, vec![symbol as u16]);
        }

        return Ok(());
    }

    it.args.insert(
        0,
        match to_decorate {
            Statement::ExpressionStatement(expr_stmt) => expr_stmt,
            _ => panic!(),
        }
        .expression
        .unwrap(),
    );

    if let Err(msg) = compiler.compile_expr(Expression::CallExpression(it)) {
        return Err(CompileError::from_none_token(format!(
            "error compile decorator: \n{msg}"
        )));
    }

    compiler.emit(OP_CALL, vec![1u16]);

    Ok(())
}

fn compile_decorator_ident(
    compiler: &mut Compiler,
    decorator: Box<Expression>,
    to_decorate: Statement,
) -> Result<(), CompileError> {
    if let Err(msg) = compiler.compile_expr(*decorator) {
        return Err(CompileError::from_none_token(format!(
            "error compile decorator: \n{msg}"
        )));
    }

    if let Statement::LetStatement(let_stmt) = to_decorate {
        let symbol = compiler
            .symbol_table
            .borrow_mut()
            .define(&let_stmt.name.value);

        if let Err(msg) = compiler.compile_expr(*let_stmt.value) {
            return Err(CompileError::from_none_token(format!(
                "error compile decorate expression: \n{msg}"
            )));
        }

        compiler.emit(OP_CALL, vec![1u16]);

        compiler.emit(
            if symbol.scope == SymbolScope::Global {
                OP_SET_GLOBAL
            } else {
                OP_SET_LOCAL
            },
            vec![symbol.index as u16],
        );

        return Ok(());
    }

    if let Statement::ExpressionStatement(ref expr_stmt) = to_decorate
        && let Expression::FunctionExpression(func_expr) =
            expr_stmt.expression.as_ref().unwrap().as_ref()
    {
        let func_name = func_expr.name.clone();

        if let Err(msg) = compiler.compile_stmt(to_decorate) {
            return Err(CompileError::from_none_token(format!(
                "error compile decorate expression: \n{msg}"
            )));
        }

        let mut symbol_and_mode = (None, OP_SET_GLOBAL);

        if compiler.last_instruction_is(OP_SET_GLOBAL) {
            compiler.remove_last_instruction();

            func_name.and_then(|it| {
                if let Some(symbol) = compiler.symbol_table.borrow_mut().resolve(&it) {  
                    symbol_and_mode = (Some(symbol.index), OP_SET_GLOBAL);  
                }

                Some(())
            });
        } else if compiler.last_instruction_is(OP_SET_LOCAL) {
            compiler.remove_last_instruction();

            func_name.and_then(|it| {
                if let Some(symbol) = compiler.symbol_table.borrow_mut().resolve(&it) {  
                    symbol_and_mode = (Some(symbol.index), OP_SET_LOCAL);  
                }
                
                Some(())
            });
        }
        
        compiler.emit(OP_CALL, vec![1u16]);
        
        if let (Some(symbol), mode) = symbol_and_mode {
            compiler.emit(mode, vec![symbol as u16]);
        }

        return Ok(());
    }

    if let Err(msg) = compiler.compile_stmt(to_decorate) {
        return Err(CompileError::from_none_token(format!(
            "error compile decorate expression: \n{msg}"
        )));
    }

    compiler.emit(OP_CALL, vec![1u16]);

    Ok(())
}

pub fn compile_decorator(compiler: &mut Compiler, node: Node) -> Result<(), CompileError> {
    let decorator = match match node {
        Node::Expression(expr) => expr,
        _ => unreachable!(),
    } {
        Expression::Decorator(it) => it,
        _ => unreachable!(),
    };

    if let &Expression::Identifier(_) = &*decorator.decorator {
        return compile_decorator_ident(compiler, decorator.decorator, decorator.to_decorate);
    } else if let Expression::CallExpression(it) = *decorator.decorator {
        return compile_decorator_call(compiler, decorator.to_decorate, it);
    }

    Ok(())
}
