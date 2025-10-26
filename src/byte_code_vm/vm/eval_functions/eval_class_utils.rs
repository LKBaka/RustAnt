use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{obj_enum::object::Object, object::ant_class::AntClass};

pub fn build_class(
    stack: &Vec<Rc<RefCell<Object>>>,
    name: &str,
    start_index: usize,
    end_index: usize,
) -> Result<AntClass, String> {
    let mut m = HashMap::with_capacity(end_index - start_index);

    for i in (start_index..end_index).step_by(2) {
        let k  = stack[i].borrow().clone();

        let key = match k {
            Object::AntString(s) => s.value,
            _ => return Err(format!("expected an string field, got: {k:#?}"))
        };

        let v  = stack[i + 1].borrow().clone();

        m.insert(key, v);
    }

    Ok(AntClass::from((name, m)))
}
