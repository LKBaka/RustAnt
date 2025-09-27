use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeInfo {
    pub file_name: Rc<str>,
    pub scope_name: Rc<str>,
}