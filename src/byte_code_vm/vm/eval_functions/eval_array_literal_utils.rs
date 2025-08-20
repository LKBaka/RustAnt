use std::{cell::RefCell, rc::Rc};

use crate::object::{ant_array::AntArray, object::Object};

pub fn build_array(
    stack: &Vec<Rc<RefCell<Object>>>,
    start_index: usize,
    end_index: usize,
) -> AntArray {
    let mut items = Vec::with_capacity(end_index - start_index);

    for i in start_index..end_index {
        if items.len() <= i {
            items.push(stack[i].borrow().clone());
        } else {
            items[i - start_index] = stack[i].borrow().clone();
        }
    }

    AntArray::from(items)
}
