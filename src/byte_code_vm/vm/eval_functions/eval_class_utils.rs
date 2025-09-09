use std::{cell::RefCell, rc::Rc};

use crate::{obj_enum::object::Object, object::ant_class::AntClass};

pub fn build_class(
    stack: &Vec<Rc<RefCell<Object>>>,
    start_index: usize,
    end_index: usize,
) -> AntClass {
    let mut m = hashbrown::HashMap::with_capacity(end_index - start_index);

    for i in (start_index..end_index).step_by(2) {
        let k  = stack[i].borrow().clone();
        let v  = stack[i + 1].borrow().clone();

        m.insert(k, v);
    }

    AntClass::from(m)
}
