use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct RuntimeInfo {
    pub file_name: Rc<str>,
    pub scope_name: Rc<str>,
}