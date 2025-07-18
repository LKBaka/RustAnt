use std::{cell::RefCell, rc::Rc};

use crate::environment::environment::Environment;


pub type RcRefCellEnv = Rc<RefCell<Environment>>;
