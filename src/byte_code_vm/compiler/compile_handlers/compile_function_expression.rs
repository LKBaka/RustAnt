use std::rc::Rc;

#[cfg(feature = "debug")]
use crate::object::id_counter::next_id;
use crate::{
    ast::{
        ast::{INode, Node, TypeNameGetter},
        expr::Expression,
        stmt::Statement,
    },
    byte_code_vm::{
        code::code::{OP_CLOSURE, OP_POP, OP_RETURN_VALUE, OP_SET_GLOBAL, OP_SET_LOCAL},
        compiler::compiler::{CompileError, Compiler},
        scope_info::ScopeInfo,
    },
    obj_enum::object::Object,
    object::ant_compiled_function::CompiledFunction,
};

pub fn compile_function_expression(
    compiler: &mut Compiler,
    node: Node,
) -> Result<(), CompileError> {
    let is_closure = compiler.symbol_table.borrow().outer.is_some();

    let func_expr = match match node {
        Node::Expression(expr) => expr,
        _ => panic!(),
    } {
        Expression::FunctionExpression(it) => it,
        _ => panic!(),
    };

    let func_token = func_expr.token();

    let symbol_index = if let Some(name) = &func_expr.name {
        Some(compiler.symbol_table.borrow_mut().define(name).index as u16)
    } else {
        None
    };

    compiler.enter_scope(ScopeInfo {
        file_name: func_token.file.as_str().into(),
        scope_name: if let Some(name) = &func_expr.name {
            name.as_str().into()
        } else {
            format!(
                "<Function (Line {} Column {})>",
                func_token.line, func_token.column
            )
            .into()
        },
    });

    if let Some(name) = &func_expr.name {
        compiler
            .symbol_table
            .borrow_mut()
            .define_function_name(name)
            .index as u16;
    }

    let mut param_vec = vec![];

    for param in &func_expr.params {
        match &**param {
            Expression::Identifier(it) => param_vec.push(it),
            _ => {
                return Err(CompileError::from(
                    format!("expected an identifier, got: {}", param.type_name()),
                    Some(param.token()),
                ));
            }
        }
    }

    for param in param_vec {
        compiler.symbol_table.borrow_mut().define(&param.value);
    }

    let compile_body_result = compiler.compile_stmt(Statement::BlockStatement(func_expr.block));

    if let Err(msg) = compile_body_result {
        return Err(CompileError::from_none_token(format!(
            "error compile function body: {msg}"
        )));
    }

    if compiler.last_instruction_is(OP_POP) {
        compiler.remove_last_pop_to(OP_RETURN_VALUE, &vec![]);
    }

    let free_symbols = compiler.symbol_table.borrow().free_symbols.clone();

    let local_count = compiler.symbol_table.borrow().num_definitions;
    let param_count = func_expr.params.len();

    let instructions = compiler.leave_scope().borrow().clone();

    for symbol in &free_symbols {
        compiler.load_symbol(symbol);
    }

    let compiled_function = CompiledFunction {
        #[cfg(feature = "debug")]
        id: next_id(),
        instructions: Rc::new(instructions),
        local_count,
        param_count,
        scope_info: ScopeInfo {
            file_name: func_expr.token.file.as_str().into(),
            scope_name: if let Some(name) = &func_expr.name {
                name.as_str().into()
            } else {
                format!(
                    "<Function (Line {} Column {})>",
                    func_token.line, func_token.column
                )
                .into()
            },
        },
    };

    let constant_index = compiler.add_constant(Object::CompiledFunction(compiled_function)) as u16;

    compiler.emit(OP_CLOSURE, vec![constant_index, free_symbols.len() as u16]);

    if func_expr.name.is_some() {
        compiler.emit(
            if is_closure {
                OP_SET_LOCAL
            } else {
                OP_SET_GLOBAL
            },
            vec![symbol_index.unwrap()],
        );
    }

    Ok(())
}
