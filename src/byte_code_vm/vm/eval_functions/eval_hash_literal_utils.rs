use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;

use crate::{obj_enum::object::Object, object::ant_hash_map::AntHashMap};

pub fn build_hash_map(
    stack: &Vec<Rc<RefCell<Object>>>,
    start_index: usize,
    end_index: usize,
) -> AntHashMap {
    let mut m = IndexMap::with_capacity(end_index - start_index);

    for i in (start_index..end_index).step_by(2) {
        let k  = stack[i].borrow().clone();
        let v  = stack[i + 1].borrow().clone();

        m.insert(k, v);
    }

    AntHashMap::from(m)
}
