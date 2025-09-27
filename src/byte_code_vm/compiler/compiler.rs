use std::{cell::RefCell, fmt::Display, mem, rc::Rc};

use crate::{
    ast::{
        ast::{INode, Node, Program, TypeNameGetter},
        expr::Expression,
        stmt::Statement,
    },
    big_dec,
    builtin::builtin_map::BUILTIN_MAP_INDEX,
    byte_code_vm::{
        code::code::{
            make, Instructions, OpCode, OP_ARRAY, OP_CALL, OP_CONSTANTS, OP_CURRENT_CLOSURE, OP_FALSE, OP_GET_BUILTIN, OP_GET_FIELD, OP_GET_FREE, OP_GET_GLOBAL, OP_GET_LOCAL, OP_INDEX, OP_JUMP, OP_LOAD_MODULE, OP_NONE, OP_POP, OP_RETURN_VALUE, OP_SET_FIELD, OP_SET_GLOBAL, OP_SET_INDEX, OP_SET_LOCAL, OP_TEST_PRINT, OP_TRUE
        },
        compiler::{
            compile_handlers::{
                compile_call_expression::compile_call_expression, compile_class::compile_class,
                compile_function_expression::compile_function_expression,
                compile_hash_literal::compile_hash_literal,
                compile_if_expression::compile_if_expression,
                compile_infix_expression::compile_infix_expression,
                compile_prefix_expression::compile_prefix_expression,
                compile_while_statement::compile_while_statement,
            },
            constant_pool::{CONSTANT_POOL_0_256, I64_CONSTANT_POOL_0_256},
            symbol_table::symbol_table::{Symbol, SymbolScope, SymbolTable},
        },
        constants::{FAKE_OFFSET_JUMP, FIELD_POOL},
        scope_info::ScopeInfo,
    },
    obj_enum::object::Object,
    object::{ant_double::AntDouble, ant_i64::AntI64, ant_int::AntInt, ant_string::AntString},
    rc_ref_cell,
    token::token::Token,
};

#[derive(Debug, Clone)]
pub struct CompilationScope {
    pub scope_info: ScopeInfo,
    pub instructions: Rc<RefCell<Instructions>>,
    pub last_instruction: EmittedInstruction,
    pub previous_instruction: EmittedInstruction,
}

#[derive(Debug, Clone)]
pub struct CompilationScopeBuilder {
    pub instructions: Rc<RefCell<Instructions>>,
    pub last_instruction: EmittedInstruction,
    pub previous_instruction: EmittedInstruction,
}

impl Default for CompilationScopeBuilder {
    fn default() -> Self {
        Self {
            instructions: rc_ref_cell!(vec![]),
            last_instruction: EmittedInstruction::default(),
            previous_instruction: EmittedInstruction::default(),
        }
    }
}

impl CompilationScopeBuilder {
    pub fn build(self, scope_info: ScopeInfo) -> CompilationScope {
        CompilationScope::new(
            scope_info,
            self.instructions,
            self.last_instruction,
            self.previous_instruction,
        )
    }
}

impl CompilationScope {
    pub fn new(
        scope_info: ScopeInfo,
        instructions: Rc<RefCell<Instructions>>,
        last_instruction: EmittedInstruction,
        previous_instruction: EmittedInstruction,
    ) -> Self {
        Self {
            scope_info,
            instructions,
            last_instruction,
            previous_instruction,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EmittedInstruction {
    pub op: OpCode,
    pub pos: usize,
}

impl EmittedInstruction {
    pub fn new(op: OpCode, pos: usize) -> Self {
        EmittedInstruction { op, pos }
    }
}

#[derive(Debug, Clone)]
pub struct ByteCode {
    pub instructions: Instructions,
    pub constants: Vec<Rc<RefCell<Object>>>,
    pub main_info: ScopeInfo,
}

impl ByteCode {
    pub fn new(
        instructions: Instructions,
        constants: Vec<Rc<RefCell<Object>>>,
        info: ScopeInfo,
    ) -> Self {
        Self {
            instructions,
            constants,
            main_info: info,
        }
    }
}

#[derive(Debug)]
pub struct CompileError {
    pub message: String,
    pub token: Option<Token>,
}

impl CompileError {
    pub fn from(message: String, token: Option<Token>) -> Self {
        Self { message, token }
    }

    pub fn from_none_token(message: String) -> Self {
        Self {
            message,
            token: None,
        }
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token {
            Some(token) => write!(
                f,
                "{}\n(at line: {}, at column: {})",
                self.message, token.line, token.column
            ),

            None => write!(f, "{}", self.message),
        }
    }
}

pub struct Compiler {
    constants: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,

    pub break_command_pos: Vec<usize>,
    pub continue_command_pos: Vec<usize>,

    pub symbol_table: Rc<RefCell<SymbolTable>>,

    pub scopes: Vec<CompilationScope>,
    pub scope_index: usize,

    pub file_name: Rc<str>,
}

impl Compiler {
    pub fn init_builtin_map(table: Rc<RefCell<SymbolTable>>) {
        let mut table_mut = table.borrow_mut();

        for (i, name) in BUILTIN_MAP_INDEX.iter().enumerate() {
            table_mut.define_builtin(i, name);
        }
    }

    pub fn new(file_name: Rc<str>) -> Self {
        let main_scope: CompilationScope = CompilationScopeBuilder::default().build(ScopeInfo {
            file_name: file_name.clone(),
            scope_name: "__main__".into(),
        });

        let symbol_table = rc_ref_cell!(SymbolTable::new());

        Self::init_builtin_map(symbol_table.clone());

        Self {
            constants: rc_ref_cell!(vec![]),
            break_command_pos: vec![],
            continue_command_pos: vec![],
            symbol_table,
            scope_index: 0,
            scopes: vec![main_scope],
            file_name,
        }
    }

    pub fn with_state(
        symbol_table: Rc<RefCell<SymbolTable>>,
        constants: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,
        file_name: Rc<str>,
    ) -> Self {
        let main_scope = CompilationScopeBuilder::default().build(ScopeInfo {
            file_name: file_name.clone(),
            scope_name: "__main__".into(),
        });

        Self {
            constants,
            break_command_pos: vec![],
            continue_command_pos: vec![],
            symbol_table,
            scope_index: 0,
            scopes: vec![main_scope],
            file_name,
        }
    }

    pub fn compile_expr(&mut self, node: Expression) -> Result<(), CompileError> {
        match node {
            Expression::TupleExpression(mut tuple_expr) => {
                if tuple_expr.expressions.len() == 1 {
                    return self.compile_expr(*tuple_expr.expressions.remove(0));
                }

                Ok(())
            }

            Expression::IntegerLiteral(integer_literal) => {
                use num_traits::ToPrimitive;

                // 常量池优化
                let integer = if integer_literal.value > big_dec!(0)
                    && integer_literal.value < big_dec!(257)
                {
                    CONSTANT_POOL_0_256[integer_literal.value.to_usize().unwrap()].clone()
                } else {
                    Object::AntInt(AntInt::from(integer_literal.value))
                };

                let constant_index = self.add_constant(integer);
                self.emit(OP_CONSTANTS, vec![constant_index as u16]);

                Ok(())
            }

            Expression::Int64Literal(integer_literal) => {
                // 常量池优化
                let integer = if integer_literal.value > 0
                    && integer_literal.value < 257
                {
                    I64_CONSTANT_POOL_0_256[integer_literal.value as usize].clone()
                } else {
                    Object::AntI64(AntI64::from(integer_literal.value))
                };

                let constant_index = self.add_constant(integer);
                self.emit(OP_CONSTANTS, vec![constant_index as u16]);

                Ok(())
            }

            Expression::StringLiteral(str_literal) => {
                let string = Object::AntString(AntString::new(str_literal.value));

                let constant_index = self.add_constant(string);
                self.emit(OP_CONSTANTS, vec![constant_index as u16]);

                Ok(())
            }

            Expression::BooleanLiteral(boolean_literal) => {
                self.emit(
                    if boolean_literal.value {
                        OP_TRUE
                    } else {
                        OP_FALSE
                    },
                    vec![],
                );

                Ok(())
            }

            Expression::DoubleLiteral(double_literal) => {
                let double = Object::AntDouble(AntDouble::from(double_literal.value));

                let constant_index = self.add_constant(double);
                self.emit(OP_CONSTANTS, vec![constant_index as u16]);

                Ok(())
            }

            Expression::NoneLiteral(_) => {
                self.emit(OP_NONE, vec![]);

                Ok(())
            }

            Expression::AssignmentExpression(assign_expr) => {
                let result = self.compile_expr(*assign_expr.value);

                if let Err(msg) = result {
                    return Err(CompileError::from_none_token(format!(
                        "error compile assignment value: {msg}"
                    )));
                }

                match *assign_expr.left {
                    Expression::Identifier(ident) => {
                        let symbol = if let Some(it) =
                            self.symbol_table.borrow_mut().resolve(&ident.value)
                        {
                            it
                        } else {
                            return Err(CompileError::from(
                                format!("undefined identifier: {}.", ident.value),
                                Some(ident.token),
                            ));
                        };

                        self.emit(
                            if symbol.scope == SymbolScope::Global {
                                OP_SET_GLOBAL
                            } else {
                                OP_SET_LOCAL
                            },
                            vec![symbol.index as u16],
                        );
                    }

                    Expression::IndexExpression(index_expr) => {
                        if let Err(msg) = self.compile_expr(*index_expr.index) {
                            return Err(CompileError::from_none_token(format!(
                                "error compile index: {msg}"
                            )));
                        }

                        if let Err(msg) = self.compile_expr(*index_expr.expr) {
                            return Err(CompileError::from_none_token(format!(
                                "error compile target: {msg}"
                            )));
                        }

                        self.emit(OP_SET_INDEX, vec![]);
                    }

                    Expression::ObjectMemberExpression(obj_member) => {
                        if let Expression::Identifier(field) = *obj_member.right {
                            let field_index = self.add_field(&field.value) as u16;

                            if let Err(msg) = self.compile_expr(*obj_member.left) {
                                return Err(CompileError::from_none_token(format!(
                                    "error compile object: {msg}"
                                )));
                            }

                            self.emit(OP_SET_FIELD, vec![field_index]);
                        }
                    }

                    _ => {
                        return Err(CompileError::from(
                            String::from(
                                "cannot assign to literal here. Maybe you meant '==' instead of '='?",
                            ),
                            Some(assign_expr.left.token()),
                        ));
                    }
                }

                Ok(())
            }

            Expression::Identifier(ident) => {
                let symbol = self.symbol_table.borrow_mut().resolve(&ident.value);

                if let Some(symbol) = symbol {
                    self.load_symbol(&symbol);

                    Ok(())
                } else {
                    Err(CompileError::from(
                        format!("undefined identifier: {}.", ident.value,),
                        Some(ident.token),
                    ))
                }
            }

            Expression::ArrayLiteral(arr) => {
                let arr_len = arr.items.len();

                for expr in arr.items {
                    let compile_result = self.compile_expr(*expr);
                    if let Err(msg) = compile_result {
                        return Err(CompileError::from_none_token(format!(
                            "error compile array item: {msg}"
                        )));
                    }
                }

                self.emit(OP_ARRAY, vec![arr_len as u16]);

                Ok(())
            }

            Expression::IndexExpression(index_expr) => {
                if let Err(msg) = self.compile_expr(*index_expr.expr) {
                    return Err(CompileError::from_none_token(format!(
                        "error compile left expression: {msg}"
                    )));
                }

                if let Err(msg) = self.compile_expr(*index_expr.index) {
                    return Err(CompileError::from_none_token(format!(
                        "error compile index: {msg}"
                    )));
                }

                self.emit(OP_INDEX, vec![]);

                Ok(())
            }

            Expression::ReturnExpression(return_expr) => {
                // 第一件事 检查作用域
                if self.symbol_table.borrow().outer.is_none() {
                    return Err(CompileError::from(
                        format!("cannot return outside function"),
                        Some(return_expr.token()),
                    ));
                }

                if let Err(msg) = self.compile_expr(*return_expr.value) {
                    return Err(CompileError::from_none_token(format!(
                        "error compile return value: {msg}"
                    )));
                }

                self.emit(OP_RETURN_VALUE, vec![]);

                Ok(())
            }

            Expression::ObjectMemberExpression(obj_member_expr) => {
                if let Err(msg) = self.compile_expr(*obj_member_expr.left) {
                    return Err(CompileError::from_none_token(format!(
                        "error compile object: {msg}"
                    )));
                }

                let field = if let Expression::Identifier(it) = *obj_member_expr.right {
                    it
                } else {
                    return Err(CompileError::from(
                        format!("expected an identifier of object member"),
                        Some(obj_member_expr.right.token()),
                    ));
                };

                let field_index = self.add_field(&field.value) as u16;

                self.emit(OP_GET_FIELD, vec![field_index]);

                Ok(())
            }

            Expression::BreakExpression(_) => {
                let pos = self.emit(OP_JUMP, vec![FAKE_OFFSET_JUMP]);
                self.break_command_pos.push(pos);

                Ok(())
            }

            Expression::ContinueExpression(_) => {
                let pos = self.emit(OP_JUMP, vec![FAKE_OFFSET_JUMP]);
                self.continue_command_pos.push(pos);

                Ok(())
            }

            Expression::TestPrintExpression(test_print_expr) => {
                if let Err(msg) = self.compile_expr(*test_print_expr.value) {
                    return Err(CompileError::from_none_token(format!(
                        "error compile return value: {msg}"
                    )));
                }

                self.emit(OP_TEST_PRINT, vec![]);

                Ok(())
            }

            Expression::Decorator(decorator) => {
                if let &Expression::Identifier(_) = &*decorator.decorator {
                    if let Err(msg) = self.compile_expr(*decorator.decorator) {
                        return Err(CompileError::from_none_token(format!(
                            "error compile decorator: {msg}"
                        )));
                    }

                    if let Statement::LetStatement(let_stmt) = decorator.to_decorate {
                        let symbol = self.symbol_table.borrow_mut().define(&let_stmt.name.value);

                        if let Err(msg) = self.compile_expr(*let_stmt.value) {
                            return Err(CompileError::from_none_token(format!(
                                "error compile decorate expression: {msg}"
                            )));
                        }

                        self.emit(OP_CALL, vec![1u16]);

                        self.emit(
                            if symbol.scope == SymbolScope::Global {
                                OP_SET_GLOBAL
                            } else {
                                OP_SET_LOCAL
                            },
                            vec![symbol.index as u16],
                        );

                        return Ok(());
                    }

                    if let Err(msg) = self.compile_stmt(decorator.to_decorate) {
                        return Err(CompileError::from_none_token(format!(
                            "error compile decorate expression: {msg}"
                        )));
                    }

                    self.emit(OP_CALL, vec![1u16]);
                } else if let Expression::CallExpression(mut it) = *decorator.decorator {
                    if let Statement::LetStatement(ref let_stmt) = decorator.to_decorate {
                        it.args.insert(
                            0,
                            match decorator.to_decorate {
                                Statement::ExpressionStatement(expr_stmt) => expr_stmt,
                                _ => panic!(),
                            }
                            .expression
                            .unwrap(),
                        );

                        if let Err(msg) = self.compile_expr(Expression::CallExpression(it)) {
                            return Err(CompileError::from_none_token(format!(
                                "error compile decorator: {msg}"
                            )));
                        }

                        let symbol = self.symbol_table.borrow_mut().define(&let_stmt.name.value);

                        self.emit(
                            if symbol.scope == SymbolScope::Global {
                                OP_SET_GLOBAL
                            } else {
                                OP_SET_LOCAL
                            },
                            vec![symbol.index as u16],
                        );

                        return Ok(());
                    }

                    it.args.insert(
                        0,
                        match decorator.to_decorate {
                            Statement::ExpressionStatement(expr_stmt) => expr_stmt,
                            _ => panic!(),
                        }
                        .expression
                        .unwrap(),
                    );

                    if let Err(msg) = self.compile_expr(Expression::CallExpression(it)) {
                        return Err(CompileError::from_none_token(format!(
                            "error compile decorator: {msg}"
                        )));
                    }
                }

                Ok(())
            }

            Expression::CallExpression(expr) => {
                compile_call_expression(self, Node::Expression(Expression::CallExpression(expr)))
            }
            Expression::FunctionExpression(expr) => compile_function_expression(
                self,
                Node::Expression(Expression::FunctionExpression(expr)),
            ),
            Expression::HashLiteral(expr) => {
                compile_hash_literal(self, Node::Expression(Expression::HashLiteral(expr)))
            }
            Expression::IfExpression(expr) => {
                compile_if_expression(self, Node::Expression(Expression::IfExpression(expr)))
            }
            Expression::InfixExpression(expr) => {
                compile_infix_expression(self, Node::Expression(Expression::InfixExpression(expr)))
            }
            Expression::PrefixExpression(expr) => compile_prefix_expression(
                self,
                Node::Expression(Expression::PrefixExpression(expr)),
            ),

            _ => Err(CompileError::from(
                format!("no compile handler for node: {}", node.type_name()),
                Some(node.token()),
            )),
        }
    }

    pub fn compile_stmt(&mut self, node: Statement) -> Result<(), CompileError> {
        match node {
            Statement::BlockStatement(block) => {
                for stmt in block.statements {
                    self.compile_stmt(stmt)?;
                }

                Ok(())
            }

            Statement::ExpressionStatement(expr_stmt) => {
                if let Some(expr) = expr_stmt.expression {
                    let need_skip_pop = match &*expr {
                        Expression::FunctionExpression(_) => true,
                        Expression::Decorator(_) => true,
                        Expression::AssignmentExpression(_) => true,
                        _ => false,
                    };

                    self.compile_expr(*expr)?;

                    if need_skip_pop {
                        return Ok(());
                    }
                    self.emit(OP_POP, vec![]);
                }

                Ok(())
            }

            Statement::LetStatement(let_stmt) => {
                let symbol = self.symbol_table.borrow_mut().define(&let_stmt.name.value);

                let result = self.compile_expr(*let_stmt.value);

                if let Err(msg) = result {
                    return Err(CompileError::from_none_token(format!(
                        "error compile let statement: {msg}"
                    )));
                }

                self.emit(
                    if symbol.scope == SymbolScope::Global {
                        OP_SET_GLOBAL
                    } else {
                        OP_SET_LOCAL
                    },
                    vec![symbol.index as u16],
                );

                Ok(())
            }

            Statement::UseStatement(use_statement) => {
                let mod_name_index = self.add_constant(Object::AntString(AntString::new(
                    use_statement.name.value.clone(),
                )));

                self.emit(OP_LOAD_MODULE, vec![mod_name_index as u16]);

                let name = if let Some(name) = &use_statement.alias {
                    &name.value
                } else {
                    &use_statement.name.value
                };

                let symbol = self.symbol_table.borrow_mut().define(&name);

                self.emit(
                    if self.symbol_table.borrow().outer.is_none() {
                        OP_SET_GLOBAL
                    } else {
                        OP_SET_LOCAL
                    },
                    vec![symbol.index as u16],
                );

                Ok(())
            }

            Statement::ClassStatement(stmt) => {
                compile_class(self, Node::Statement(Statement::ClassStatement(stmt)))
            }
            Statement::WhileStatement(stmt) => {
                compile_while_statement(self, Node::Statement(Statement::WhileStatement(stmt)))
            }
        }
    }

    #[inline(always)]
    pub fn load_symbol(&mut self, symbol: &Symbol) {
        match symbol.scope {
            SymbolScope::Global => self.emit(OP_GET_GLOBAL, vec![symbol.index as u16]),
            SymbolScope::Local => self.emit(OP_GET_LOCAL, vec![symbol.index as u16]),
            SymbolScope::Free => self.emit(OP_GET_FREE, vec![symbol.index as u16]),
            SymbolScope::Function => self.emit(OP_CURRENT_CLOSURE, vec![]),
            SymbolScope::Builtin => self.emit(OP_GET_BUILTIN, vec![symbol.index as u16]),
        };
    }

    pub fn enter_scope(&mut self, scope_info: ScopeInfo) {
        self.symbol_table = rc_ref_cell!(SymbolTable::with_outer(self.symbol_table.clone()));

        let scope = CompilationScopeBuilder::default().build(scope_info);

        self.scopes.push(scope);

        self.scope_index += 1;
    }

    pub fn leave_scope(&mut self) -> Rc<RefCell<Instructions>> {
        let instructions = self.current_instructions();

        // self.scopes = self.scopes[0..self.scopes.len() - 1].to_vec();
        self.scopes.pop();

        self.scope_index -= 1;

        let outer = self
            .symbol_table
            .borrow()
            .outer
            .clone()
            .expect("expected an outer");

        self.symbol_table = outer;

        instructions
    }

    pub fn add_field(&self, field: &str) -> usize {
        if let Ok(i) = FIELD_POOL.lock().unwrap().binary_search(&field.to_owned()) {
            return i;
        }

        FIELD_POOL.lock().unwrap().push(field.into());

        FIELD_POOL.lock().unwrap().len() - 1
    }

    pub fn current_instructions(&self) -> Rc<RefCell<Instructions>> {
        self.scopes[self.scope_index].instructions.clone()
    }

    fn replace_instruction(&mut self, pos: usize, new_instruction: Instructions) {
        let current_instructions = self.current_instructions();

        let mut target = current_instructions.borrow_mut(); // 获取可变引用
        target[pos..pos + new_instruction.len()].copy_from_slice(new_instruction.as_slice());
    }

    pub fn change_operand(&mut self, op_pos: usize, operand: u16) {
        let current_instructions = self.current_instructions();

        let mut target = current_instructions.borrow_mut();

        let op = target[op_pos];
        let new_instruction = make(op, &[operand].to_vec());
        let len = new_instruction.len();

        target[op_pos..op_pos + len].copy_from_slice(new_instruction.as_slice());
    }

    pub fn last_instruction_is(&mut self, op: OpCode) -> bool {
        if self.current_instructions().borrow().is_empty() {
            return false;
        }

        self.scopes[self.scope_index].last_instruction.op == op
    }

    pub fn remove_last_pop(&mut self) {
        let last_instruction = self.scopes[self.scope_index].last_instruction;
        let previous_instruction = self.scopes[self.scope_index].previous_instruction;

        let old = self.current_instructions();
        let new = old.clone().borrow_mut()[0..(last_instruction.pos)].to_vec();

        self.scopes[self.scope_index].instructions = rc_ref_cell!(new);
        self.scopes[self.scope_index].last_instruction = previous_instruction;
    }

    pub fn remove_last_pop_to(&mut self, op: OpCode, operands: &Vec<u16>) {
        let last_pos = self.scopes[self.scope_index].last_instruction.pos;

        self.replace_instruction(last_pos, make(op, operands));

        self.scopes[self.scope_index].last_instruction.op = op
    }

    pub fn add_constant(&mut self, obj: Object) -> usize {
        let mut constant_borrow_mut = self.constants.borrow_mut();
        constant_borrow_mut.push(rc_ref_cell!(obj));
        constant_borrow_mut.len() - 1 // this number is the index of the constant, use it as a identifier for constant
    }

    pub fn emit(&mut self, op: OpCode, operands: Vec<u16>) -> usize {
        let ins = make(op, &operands);
        let pos = self.add_instruction(ins);

        self.set_last_instruction(op, pos);

        pos // return the position of the instruction in the instructions vector
    }

    pub fn set_last_instruction(&mut self, op: OpCode, pos: usize) {
        let previous = mem::take(&mut self.scopes[self.scope_index].last_instruction);
        let last = EmittedInstruction::new(op, pos);

        self.scopes[self.scope_index].previous_instruction = previous;
        self.scopes[self.scope_index].last_instruction = last;
    }

    pub fn add_instruction(&mut self, ins: Vec<u8>) -> usize {
        let pos_new_instruction = self.scopes[self.scope_index].instructions.borrow().len();
        self.scopes[self.scope_index]
            .instructions
            .borrow_mut()
            .extend(ins);

        pos_new_instruction // return the position of the new instruction in the instructions vector
    }

    pub fn start_compile(&mut self, program: Program) -> Result<(), CompileError> {
        for stmt in program.statements {
            self.compile_stmt(stmt)?;
        }

        Ok(())
    }

    pub fn bytecode(&mut self) -> ByteCode {
        ByteCode::new(
            self.current_instructions().borrow().clone(),
            self.constants.borrow().clone(),
            ScopeInfo {
                file_name: self.file_name.clone(),
                scope_name: Rc::from("__main__"),
            },
        )
    }

    pub fn traceback_string(&self) -> String {
        let indent = "  ";

        format!(
            "traceback (most recent call last):\n{indent}{}",
            self.scopes
                .iter()
                .map(|scope| format!(
                    "file \"{}\", in {}",
                    scope.scope_info.file_name, scope.scope_info.scope_name,
                ))
                .collect::<Vec<String>>()
                .join(&format!("\n{indent}"))
        )
    }
}
