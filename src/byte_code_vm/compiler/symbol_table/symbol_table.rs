use std::{cell::RefCell, fmt::Debug, rc::Rc};

use hashbrown::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SymbolScope {
    Global, Local
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize
}

impl Symbol {
    pub fn new(
        name: String,
        scope: SymbolScope,
        index: usize
    ) -> Self {
        Self {
            name,
            scope,
            index,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolTable {
    pub outer: Option<Rc<RefCell<SymbolTable>>>,
    store: hashbrown::HashMap<String, Symbol>,
    func_store: hashbrown::HashMap<String, Vec<Symbol>>,
    pub num_definitions: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { 
            outer: None,
            store: HashMap::new(),
            func_store: HashMap::new(),
            num_definitions: 0
        }
    }

    pub fn with_outer(outer: Rc<RefCell<SymbolTable>>) -> Self {
        Self {
            outer: Some(outer),
            store: HashMap::new(),
            func_store: HashMap::new(),
            num_definitions: 0
        }
    }

    pub fn store(&self) -> hashbrown::HashMap<String, Symbol> {
        self.store.clone()
    }

    pub fn func_store(&self) -> hashbrown::HashMap<String, Vec<Symbol>> {
        self.func_store.clone()
    }

    pub fn define(&mut self, name: &str) -> Symbol {
        let symbol = Symbol::new(
            name.into(), 
            if self.outer.is_some() { SymbolScope::Local } else { SymbolScope::Global }, 
            self.num_definitions
        );

        self.store.insert(name.into(), symbol.clone());

        self.num_definitions += 1;

        symbol
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        if let Some(it) = self.store.get(name) {
            return Some(it.clone());
        }

        if let Some(vec) = self.func_store.get(name) {
            return Some(vec[0].clone());
        }

        if let Some(outer) = &self.outer {
            let borrow_outer = outer.borrow();
            return borrow_outer.resolve(name);
        }

        None
    }
}