use std::{cell::RefCell, fmt::Debug, rc::Rc};

use hashbrown::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SymbolScope {
    Global,
    Local,
    Free,
    Function,
    Builtin,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

impl Symbol {
    pub fn new(name: String, scope: SymbolScope, index: usize) -> Self {
        Self { name, scope, index }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolTable {
    pub outer: Option<Rc<RefCell<SymbolTable>>>,
    pub free_symbols: Vec<Symbol>,
    pub store: hashbrown::HashMap<String, Symbol>,
    pub num_definitions: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            outer: None,
            free_symbols: vec![],
            store: HashMap::new(),
            num_definitions: 0,
        }
    }

    pub fn with_outer(outer: Rc<RefCell<SymbolTable>>) -> Self {
        Self {
            outer: Some(outer),
            free_symbols: vec![],
            store: HashMap::new(),
            num_definitions: 0,
        }
    }

    pub fn define(&mut self, name: &str) -> Symbol {
        let symbol = Symbol::new(
            name.into(),
            if self.outer.is_some() {
                SymbolScope::Local
            } else {
                SymbolScope::Global
            },
            self.num_definitions,
        );

        self.store.insert(name.into(), symbol.clone());

        self.num_definitions += 1;

        symbol
    }

    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        if let Some(it) = self.store.get(name) {
            return Some(it.clone());
        }

        if let Some(outer) = self.outer.clone() {
            let mut borrow_outer = outer.borrow_mut();
            let result = match borrow_outer.resolve(name) {
                Some(it) => it,
                None => return None,
            };

            if result.scope == SymbolScope::Global || result.scope == SymbolScope::Builtin {
                return Some(result);
            }

            let free = self.define_free(result);
            return Some(free);
        }

        None
    }

    // this function has bug
    pub fn define_free(&mut self, original: Symbol) -> Symbol {
        self.free_symbols.push(original.clone());

        let mut symbol = original;
        symbol.index = self.free_symbols.len() - 1;
        symbol.scope = SymbolScope::Free;

        self.store.insert(symbol.name.clone(), symbol.clone());

        symbol
    }

    pub fn define_function_name(&mut self, func_name: &str) -> Symbol {
        let symbol = Symbol::new(
            func_name.into(),
            SymbolScope::Function,
            0,
        );

        self.store.insert(func_name.into(), symbol.clone());

        symbol
    }

    pub fn define_builtin(&mut self, index: usize, name: &str) -> Symbol {
        let symbol = Symbol::new(
            name.into(),
            SymbolScope::Builtin,
            index,
        );

        self.store.insert(name.into(), symbol.clone());

        symbol
    }
}
