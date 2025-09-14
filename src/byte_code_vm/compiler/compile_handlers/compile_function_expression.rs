use std::rc::Rc;

#[cfg(feature = "debug")]
use crate::object::id_counter::next_id;
use crate::{
    ast::{
        ast::{Node, TypeNameGetter}, expr::Expression, expressions::identifier::Identifier, stmt::Statement
    }, byte_code_vm::{
        code::code::{OP_CLOSURE, OP_POP, OP_RETURN_VALUE, OP_SET_GLOBAL, OP_SET_LOCAL},
        compiler::compiler::Compiler,
    }, obj_enum::object::Object, object::ant_compiled_function::CompiledFunction
};

pub fn compile_function_expression(
    compiler: &mut Compiler,
    node: Node,
) -> Result<(), String> {
    let is_closure = compiler.symbol_table.borrow().outer.is_some();

    let func_expr = match match node {
        Node::Expression(expr) => expr,
        _ => panic!()
    } {
        Expression::FunctionExpression(it) => it,
        _ => panic!()
    };

    let symbol_index = if let Some(name) = &func_expr.name {
        Some(compiler.symbol_table.borrow_mut().define(name).index as u16)
    } else {
        None
    };

    compiler.enter_scope();

    if let Some(name) = &func_expr.name {
        compiler
            .symbol_table
            .borrow_mut()
            .define_function_name(name)
            .index as u16;
    }

    let param_vec: Vec<&Identifier> = func_expr
        .params
        .iter()
        .map(|expr| {
            match &**expr {
                Expression::Identifier(it) => it,
                _ => panic!("{}",&format!(
                    "expected an identifier, got: {}",
                    expr.type_name()
                ))
            }
        })
        .collect();

    for param in param_vec {
        compiler.symbol_table.borrow_mut().define(&param.value);
    }

    let compile_body_result = compiler.compile_stmt(Statement::BlockStatement(
        func_expr.block
    ));
    
    if let Err(msg) = compile_body_result {
        return Err(format!("error compile function body: {msg}"));
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
