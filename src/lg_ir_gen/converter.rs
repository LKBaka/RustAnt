use std::{
    any::{Any, TypeId},
    collections::{BTreeMap, HashMap},
};

use crate::{
    ast::{
        ast::{ExpressionStatement, INode, Program},
        expressions::{integer64_literal::Int64Literal, integer_literal::IntegerLiteral},
    },
    lg_ir_gen::convert_functions::{
        convert_expression_statement::convert_expression_statement,
        convert_i64_literal::convert_i64_literal,
    },
};

use indexmap::IndexMap;
use lg_rust_binding::ir::{
    IRConstantPool, IRModule,
    base::{IRControlFlowGraph, IRGlobalDataSection},
};

pub type LgIrConvertHandler = fn(&mut LgIrConverter, &dyn INode) -> Result<(), String>;

pub struct LgIrConverter {
    program: Program,
    ir_module: IRModule,
    convert_map: HashMap<TypeId, LgIrConvertHandler>,
}

impl LgIrConverter {
    pub fn new(program: Program) -> Self {
        let ir_module = IRModule {
            structures: IndexMap::new(),
            constant_pool: Box::new(IRConstantPool { entries: vec![] }),
            global_data_section: Box::new(IRGlobalDataSection { data: vec![] }),
            global_init_section: Box::new(IRControlFlowGraph {
                basic_blocks: IndexMap::new(),
                out_edges: BTreeMap::new(),
                in_edges: BTreeMap::new(),
            }),
            functions: IndexMap::new(),
            name2vtable_keys: IndexMap::new(),
            name2itable_keys: IndexMap::new(),
            entry_point: None,
        };

        let mut m: HashMap<TypeId, LgIrConvertHandler> = HashMap::with_capacity(10);

        m.insert(
            TypeId::of::<ExpressionStatement>(),
            convert_expression_statement,
        );
        m.insert(TypeId::of::<Int64Literal>(), convert_i64_literal);

        Self {
            program,
            ir_module,
            convert_map: m,
        }
    }

    pub fn convert(&mut self, node: &dyn INode) -> Result<(), String> {
        let id = (node as &dyn Any).type_id();

        let id_name = node.type_name();

        if let Some(handler) = self.convert_map.get(&id) {
            if let Err(msg) = handler(self, node) {
                return Err(format!("convert failed for node type {}: {}", id_name, msg));
            }
        } else {
            return Err(format!("no convert handler for node type: {}", id_name));
        }

        Ok(())
    }

    pub fn start_convert(&mut self) -> Result<(), String> {
        for stmt in self.program.statements.clone() {
            let result = self.convert(&stmt);
            if let Err(msg) = result {
                return Err(format!("conversion failed: {}", msg));
            }
        }

        Ok(())
    }

    pub fn ir_module(&mut self) -> &mut IRModule {
        &mut self.ir_module
    }
}
