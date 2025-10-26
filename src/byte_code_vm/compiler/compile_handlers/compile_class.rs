#[cfg(feature = "debug")]
use crate::object::id_counter::next_id;
use crate::{
    ast::{ast::Node, stmt::Statement},
    byte_code_vm::{
        code::code::{
            OP_CALL, OP_CLASS, OP_CLOSURE, OP_CONSTANTS, OP_POP, OP_RETURN_VALUE, OP_SET_GLOBAL,
            OP_SET_LOCAL,
        },
        compiler::{
            compiler::{CompileError, Compiler},
            symbol_table::symbol_table::SymbolScope,
        },
        scope_info::ScopeInfo,
    },
    obj_enum::object::Object,
    object::{ant_compiled_function::CompiledFunction, ant_string::AntString},
};

pub fn compile_class(compiler: &mut Compiler, node: Node) -> Result<(), CompileError> {
    let clazz = match match node {
        Node::Statement(stmt) => stmt,
        _ => unreachable!(),
    } {
        Statement::ClassStatement(it) => it,
        _ => unreachable!(),
    };

    let symbol = compiler.symbol_table.borrow_mut().define(&clazz.name.value);

    compiler.enter_scope(ScopeInfo {
        file_name: clazz.token.file.as_str().into(),
        scope_name: format!(
            "<Class (Name {} Line {} Column {})>",
            &clazz.name.value, clazz.token.line, clazz.token.column
        )
        .into(),
    });

    // 编译类中的代码
    if let Err(msg) = compiler.compile_stmt(Statement::BlockStatement(clazz.block)) {
        return Err(CompileError::from_none_token(format!(
            "error compile class: \n{msg}"
        )));
    }

    // 拿到当前类作用域所有的符号
    let symbols = compiler.symbol_table.borrow().store.clone();

    // (name, symbol) k, v
    let symbols_len = symbols.len() * 2;

    // 将字段名 (原 name) 压栈, 之后将符号所代表的值压栈
    for (name, symbol) in symbols {
        let field = Object::AntString(AntString::new(name));
        let field_index = compiler.add_constant(field) as u16;

        compiler.emit(OP_CONSTANTS, vec![field_index]);

        compiler.load_symbol(&symbol);
    }

    // 构造 class
    let name_constant_index =
        compiler.add_constant(Object::AntString(clazz.name.value.clone().into()));

    compiler.emit(
        OP_CLASS,
        vec![symbols_len as u16, name_constant_index as u16],
    );

    if compiler.last_instruction_is(OP_POP) {
        compiler.remove_last_pop_to(OP_RETURN_VALUE, &vec![]);
    }

    if !compiler.last_instruction_is(OP_RETURN_VALUE) {
        compiler.emit(OP_RETURN_VALUE, vec![]);
    }

    // 离开作用域
    let ins = compiler.leave_scope();

    let compiled_function = CompiledFunction {
        #[cfg(feature = "debug")]
        id: next_id(),
        instructions: ins.borrow().clone().into(),
        local_count: symbols_len / 2,
        param_count: 0,
        scope_info: ScopeInfo {
            file_name: clazz.token.file.as_str().into(),
            scope_name: format!(
                "<Class (Name {} Line {} Column {})>",
                &clazz.name.value, clazz.token.line, clazz.token.column
            )
            .into(),
        },
    };

    // 将类构造函数 (并非 new) 压栈
    let constant_index = compiler.add_constant(Object::CompiledFunction(compiled_function)) as u16;

    compiler.emit(OP_CLOSURE, vec![constant_index, 0u16]);

    // 立即调用
    compiler.emit(OP_CALL, vec![0u16]);

    // 设置 Global / Local
    compiler.emit(
        if symbol.scope == SymbolScope::Global {
            OP_SET_GLOBAL
        } else {
            OP_SET_LOCAL
        },
        vec![symbol.index as u16],
    );

    Ok(())
}
