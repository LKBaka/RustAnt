use crate::object::{ant_array::AntArray, object::Object};

pub fn build_array(stack: &Vec<Object>, start_index: usize, end_index: usize) -> AntArray {
    let mut items = Vec::with_capacity(end_index - start_index);

    for i in start_index..end_index {
        if items.len() <= i {
            items.push(stack[i].clone());
        } else {
            items[i - start_index] = stack[i].clone();
        }
    }

    AntArray::from(items)
}