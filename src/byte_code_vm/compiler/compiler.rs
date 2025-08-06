use std::{any::{Any, TypeId}, mem};

use hashbrown::HashMap as HashBrownMap; 

use crate::{ast::{ast::{ExpressionStatement, Node, Program}, expressions::{boolean_literal::BooleanLiteral, infix_expression::InfixExpression, integer_literal::IntegerLiteral, prefix_expression::PrefixExpression, tuple_expression::TupleExpression}}, byte_code_vm::{code::code::{make, Instructions, OpCode, OP_CONSTANTS, OP_FALSE, OP_POP, OP_TRUE}, compiler::compile_handlers::{compile_infix_expression::compile_infix_expression, compile_prefix_expression::compile_prefix_expression}}, convert_type, object::{ant_int::AntInt, object::Object}, struct_type_id};

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
    instructions: Instructions,
    constants: Vec<Object>,
    compile_map: HashBrownMap<TypeId, CompileHandler>,
}

impl Compiler {
    pub fn new() -> Self {
        let mut m: HashBrownMap<TypeId, CompileHandler> = HashBrownMap::with_capacity(8);

        m.insert(struct_type_id!(InfixExpression), compile_infix_expression);
        m.insert(struct_type_id!(PrefixExpression), compile_prefix_expression);

        Self {
            instructions: vec![],
            constants: vec![],
            compile_map: m,
        }
    }

    pub fn compile(&mut self, node: Box<dyn Node>) -> Result<(), String> {
        let node_id = (node.as_ref() as &dyn Any).type_id();

        match node_id {
            id if id == TypeId::of::<ExpressionStatement>() => {
                let expr_stmt = convert_type!(ExpressionStatement, node);

                if let Some(expr) = expr_stmt.expression {
                    let result = self.compile(expr);
                    if result.is_err() {return result};

                    self.emit(OP_POP, vec![]);
                }

                Ok(())
            }

            id if id == TypeId::of::<TupleExpression>() => {
                let mut tuple_expr = convert_type!(TupleExpression, node);

                if tuple_expr.expressions.len() == 1 {
                    return self.compile(tuple_expr.expressions.remove(0));
                }

                Ok(())
            }

            id if id == TypeId::of::<IntegerLiteral>() => {
                let mut integer_literal = convert_type!(IntegerLiteral, node);

                let integer: Object = Box::new(AntInt::from(mem::take(&mut integer_literal.value)));

                let constant_index = self.add_constant(integer);
                self.emit(OP_CONSTANTS, vec![constant_index as u16]);
 
                Ok(())
            }

            id if id == TypeId::of::<BooleanLiteral>() => {
                let boolean_literal = convert_type!(BooleanLiteral, node);

                self.emit(
                    if boolean_literal.value {OP_TRUE} else {OP_FALSE}, 
                    vec![]
                );

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

    pub fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1 // this number is the index of the constant, use it as a identifier for constant
    }

    pub fn emit(&mut self, op: OpCode, operands: Vec<u16>) -> usize {
        let ins = make(op, &operands);
        let pos = self.add_instruction(ins);

        pos // return the position of the instruction in the instructions vector
    }

    pub fn add_instruction(&mut self, ins: Vec<u8>) -> usize {
        let pos_new_instruction = self.instructions.len();
        self.instructions.extend(ins);

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
            mem::take(&mut self.instructions),
            mem::take(&mut self.constants)
        )
    }
}