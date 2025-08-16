use std::{any::{Any, TypeId}, cell::RefCell, mem, rc::Rc};

use hashbrown::HashMap as HashBrownMap; 

use crate::{ast::{ast::{ExpressionStatement, Node, Program}, expressions::{array_literal::ArrayLiteral, assignment_expression::AssignmentExpression, boolean_literal::BooleanLiteral, call_expression::CallExpression, double_literal::DoubleLiteral, function_expression::FunctionExpression, identifier::Identifier, if_expression::IfExpression, index_expression::IndexExpression, infix_expression::InfixExpression, integer_literal::IntegerLiteral, prefix_expression::PrefixExpression, return_expression::ReturnExpression, string_literal::StringLiteral, test_print_expression::TestPrintExpression, tuple_expression::TupleExpression}, statements::{block_statement::BlockStatement, let_statement::LetStatement, while_statement::WhileStatement}}, byte_code_vm::{code::code::{make, Instructions, OpCode, OP_ARRAY, OP_CONSTANTS, OP_FALSE, OP_GET_GLOBAL, OP_GET_LOCAL, OP_INDEX, OP_RETURN_VALUE, OP_SET_GLOBAL, OP_SET_LOCAL, OP_TEST_PRINT, OP_TRUE}, compiler::{compile_handlers::{compile_call_expression::compile_call_expression, compile_function_expression::compile_function_expression, compile_if_expression::compile_if_expression, compile_infix_expression::compile_infix_expression, compile_prefix_expression::compile_prefix_expression, compile_while_statement::compile_while_statement}, symbol_table::symbol_table::{SymbolScope, SymbolTable}}}, convert_type, object::{ant_double::AntDouble, ant_int::AntInt, ant_string::AntString, object::Object}, rc_ref_cell, struct_type_id};

#[derive(Debug, Clone)]
pub struct CompilationScope {
    pub instructions: Rc<RefCell<Instructions>>,
    pub last_instruction: EmittedInstruction,
    pub previous_instruction: EmittedInstruction,
}

impl Default for CompilationScope {
    fn default() -> Self {
        Self {
            instructions: rc_ref_cell!(vec![]),
            last_instruction: EmittedInstruction::default(),
            previous_instruction: EmittedInstruction::default(),
        }
    }
}

impl CompilationScope {
    pub fn new(
        instructions: Rc<RefCell<Instructions>>,
        last_instruction: EmittedInstruction,
        previous_instruction: EmittedInstruction,
    ) -> Self {
        Self {
            instructions,
            last_instruction,
            previous_instruction,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EmittedInstruction {
    pub op: OpCode,
    pub pos: usize
}

impl Default for EmittedInstruction {
    fn default() -> Self {
        Self { op: 0, pos: 0 }
    }
}

impl EmittedInstruction {
    pub fn new(op: OpCode, pos: usize) -> Self {
        EmittedInstruction { op, pos }
    }
}

#[derive(Debug)]
pub struct ByteCode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

impl ByteCode {
    pub fn new(instructions: Instructions, constants: Vec<Object>) -> Self {
        Self { instructions, constants }
    }
}

pub type CompileHandler = fn(&mut Compiler, Box<dyn Node>) -> Result<(), String>;

pub struct Compiler {
    constants: Rc<RefCell<Vec<Object>>>,
    
    compile_map: HashBrownMap<TypeId, CompileHandler>,
    pub symbol_table: Rc<RefCell<SymbolTable>>,

    pub scopes: Vec<CompilationScope>,
    pub scope_index: usize,
}

impl Compiler {
    pub fn init_compile_map(m: &mut HashBrownMap<TypeId, CompileHandler>) {
        m.insert(struct_type_id!(InfixExpression), compile_infix_expression);
        m.insert(struct_type_id!(PrefixExpression), compile_prefix_expression);
        m.insert(struct_type_id!(IfExpression), compile_if_expression);
        m.insert(struct_type_id!(FunctionExpression), compile_function_expression);
        m.insert(struct_type_id!(CallExpression), compile_call_expression);
        m.insert(struct_type_id!(WhileStatement), compile_while_statement);
    }

    pub fn new() -> Self {
        let main_scope = CompilationScope::new(
            rc_ref_cell!(vec![]),
            EmittedInstruction::default(),
            EmittedInstruction::default(),
        );

        let mut m: HashBrownMap<TypeId, CompileHandler> = HashBrownMap::with_capacity(8);

        Self::init_compile_map(&mut m);

        Self {
            constants: rc_ref_cell!(vec![]),
            compile_map: m,
            symbol_table: rc_ref_cell!(SymbolTable::new()),
            scope_index: 0,
            scopes: vec![main_scope]
        }
    }

    pub fn with_state(symbol_table: Rc<RefCell<SymbolTable>>, constants: Rc<RefCell<Vec<Object>>>) -> Self {
        let main_scope = CompilationScope::new(
            rc_ref_cell!(vec![]),
            EmittedInstruction::default(),
            EmittedInstruction::default(),
        );

        let mut m: HashBrownMap<TypeId, CompileHandler> = HashBrownMap::with_capacity(8);

        Self::init_compile_map(&mut m);

        Self {
            constants,
            compile_map: m,
            symbol_table,
            scope_index: 0,
            scopes: vec![main_scope]
        }
    }

    pub fn compile(&mut self, node: Box<dyn Node>) -> Result<(), String> {
        let node_id = (node.as_ref() as &dyn Any).type_id();

        match node_id {
            id if id == struct_type_id!(BlockStatement) => {
                let block = convert_type!(BlockStatement, node);

                for stmt in block.statements {
                    let result = self.compile(stmt);
                    
                    if result.is_err() {
                        return result
                    }
                }

                Ok(())
            }

            id if id == struct_type_id!(ExpressionStatement) => {
                let expr_stmt = convert_type!(ExpressionStatement, node);

                if let Some(expr) = expr_stmt.expression {
                    let result = self.compile(expr);
                    if result.is_err() {return result};
                }

                Ok(())
            }

            id if id == struct_type_id!(TupleExpression) => {
                let mut tuple_expr = convert_type!(TupleExpression, node);

                if tuple_expr.expressions.len() == 1 {
                    return self.compile(tuple_expr.expressions.remove(0));
                }

                Ok(())
            }

            id if id == struct_type_id!(IntegerLiteral) => {
                let mut integer_literal = convert_type!(IntegerLiteral, node);

                let integer: Object = Box::new(AntInt::from(mem::take(&mut integer_literal.value)));

                let constant_index = self.add_constant(integer);
                self.emit(OP_CONSTANTS, vec![constant_index as u16]);
 
                Ok(())
            }

            id if id == struct_type_id!(DoubleLiteral) => {
                let mut double_literal = convert_type!(DoubleLiteral, node);

                let double: Object = Box::new(AntDouble::from(mem::take(&mut double_literal.value)));

                let constant_index = self.add_constant(double);
                self.emit(OP_CONSTANTS, vec![constant_index as u16]);
 
                Ok(())
            }
            
            id if id == struct_type_id!(StringLiteral) => {
                let str_literal = convert_type!(StringLiteral, node);

                let string: Object = Box::new(AntString::new(str_literal.value));

                let constant_index = self.add_constant(string);
                self.emit(OP_CONSTANTS, vec![constant_index as u16]);

                Ok(())
            }

            id if id == struct_type_id!(BooleanLiteral) => {
                let boolean_literal = convert_type!(BooleanLiteral, node);

                self.emit(
                    if boolean_literal.value {OP_TRUE} else {OP_FALSE}, 
                    vec![]
                );

                Ok(())
            }

            id if id == struct_type_id!(LetStatement) => {
                let let_stmt = convert_type!(LetStatement, node);

                let result = self.compile(let_stmt.value);

                if let Err(msg) = result {
                    return Err(format!("error compile let statement: {}", msg))
                }

                let symbol = self.symbol_table.borrow_mut().define(&let_stmt.name.value);
                self.emit(
                    if symbol.scope == SymbolScope::Global { OP_SET_GLOBAL } else { OP_SET_LOCAL }, 
                    vec![symbol.index as u16]
                );

                Ok(())
            }

            id if id == struct_type_id!(AssignmentExpression) => {
                let assign_expr = convert_type!(AssignmentExpression, node);

                let result = self.compile(assign_expr.value);

                if let Err(msg) = result {
                    return Err(format!("error compile assignment expression: {}", msg))
                }

                if let Some(ident) = (assign_expr.left as Box<dyn Any>).downcast_ref::<Identifier>() {
                    let symbol = if let Some(it) = 
                        self.symbol_table.borrow_mut().resolve(&ident.value)
                    {
                        it
                    } else {
                        return Err(format!(
                            "undefined identifier: {}. at line: {}, at file: {}",
                            ident.token.line, 
                            ident.token.file, 
                            ident.value
                        ))    
                    };

                    self.emit(
                        if symbol.scope == SymbolScope::Global { OP_SET_GLOBAL } else { OP_SET_LOCAL }, 
                        vec![symbol.index as u16]
                    );
                }

                Ok(())
            }

            id if id == struct_type_id!(Identifier) => {
                let ident = convert_type!(Identifier, node);

                let symbol = { self.symbol_table.borrow().resolve(&ident.value) };

                if let Some(symbol) = symbol {
                    self.emit(
                        if symbol.scope == SymbolScope::Global { OP_GET_GLOBAL } else { OP_GET_LOCAL }, 
                        vec![symbol.index as u16]
                    );

                    Ok(())
                } else {
                    return Err(format!(
                        "undefined identifier: {}. at line: {}, at file: {}",
                        ident.value,
                        ident.token.line, 
                        ident.token.file, 
                    )) 
                }
            }

            id if id == struct_type_id!(ArrayLiteral) => {
                let arr = convert_type!(ArrayLiteral, node);

                let arr_len = arr.items.len();

                for expr in arr.items {
                    let compile_result =  self.compile(expr);
                    if let Err(msg) = compile_result {
                        return Err(format!("error compile array item: {msg}"))
                    }
                }

                self.emit(OP_ARRAY, vec![arr_len as u16]);

                Ok(())
            }

            id if id == TypeId::of::<IndexExpression>() => {
                let index_expr = convert_type!(IndexExpression, node);

                if let Err(msg) = self.compile(index_expr.expr) {
                    return Err(format!("error compile left expression: {msg}"))
                }

                if let Err(msg) = self.compile(index_expr.index) {
                    return Err(format!("error compile index: {msg}"))
                }

                self.emit(OP_INDEX, vec![]);

                Ok(())
            }

            id if id == TypeId::of::<ReturnExpression>() => {
                let return_expr = convert_type!(ReturnExpression, node);

                if let Err(msg) = self.compile(return_expr.value) {
                    return Err(format!("error compile return value: {msg}"))
                }

                self.emit(OP_RETURN_VALUE, vec![]);

                Ok(())
            }

            id if id == TypeId::of::<TestPrintExpression>() => {
                let test_print_expr = convert_type!(TestPrintExpression, node);

                if let Err(msg) = self.compile(test_print_expr.value) {
                    return Err(format!("error compile return value: {msg}"))
                }

                self.emit(OP_TEST_PRINT, vec![]);

                Ok(())
            }

            _ => {
                if let Some(handler) = self.compile_map.get(&node_id) {
                    handler(self, node)
                } else {
                    Err(format!("no compile handler for node type: {}", node.type_name()))
                }

            }
        }
    }

    pub fn enter_scope(&mut self) {
        self.symbol_table = rc_ref_cell!(SymbolTable::with_outer(self.symbol_table.clone()));

        let scope = CompilationScope::default();

        self.scopes.push(scope);

        self.scope_index += 1;
    }

    pub fn leave_scope(&mut self) -> Rc<RefCell<Instructions>> {
        let instructions = self.current_instructions();

        self.scopes = self.scopes[0..self.scopes.len() - 1].to_vec();

        self.scope_index -= 1;

        let outer = self.symbol_table
            .borrow()
            .outer
            .clone()
            .expect("expected an outer");
        
        self.symbol_table = outer;

        instructions
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

        let op = target[op_pos].clone();
        let new_instruction = make(op, &[operand].to_vec());
        let len = new_instruction.len();

        target[op_pos .. op_pos + len].copy_from_slice(new_instruction.as_slice());
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
        self.constants.borrow_mut().push(obj);
        self.constants.borrow().len() - 1 // this number is the index of the constant, use it as a identifier for constant
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
        self.scopes[self.scope_index].instructions.borrow_mut().extend(ins);

        pos_new_instruction // return the position of the new instruction in the instructions vector
    }

    pub fn start_compile(&mut self, program: Program) -> Result<(), String> {
        for stmt in program.statements {
            let result = self.compile(stmt);
            
            if result.is_err() {
                return result
            }
        }

        Ok(())
    }

    pub fn bytecode(&mut self) -> ByteCode {
        ByteCode::new(
            self.current_instructions().borrow().clone(),
            self.constants.borrow().clone()
        )
    }
}