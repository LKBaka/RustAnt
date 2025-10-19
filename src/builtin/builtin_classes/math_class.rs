use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bigdecimal::BigDecimal;
use once_cell::sync::Lazy;

use crate::{
    builtin::builtin_func::{ant_null, ant_some},
    byte_code_vm::vm::vm::Vm,
    obj_enum::object::Object,
    object::{
        ant_class::AntClass,
        ant_double::{AntDouble, sqrt_default},
        ant_native_function::create_ant_native_function,
        object::{DOUBLE, I64, IAntObject, INT},
    },
};

pub static MATH: Lazy<AntClass> = Lazy::new(|| {
    AntClass::from({
        let sqrt =
            |_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>| -> Result<Option<Object>, String> {
                let n = args[0].borrow();

                let expected_types: [&str; 3] = [INT, I64, DOUBLE];

                match &*n {
                    Object::AntI64(i) => {
                        Ok(Some(sqrt_default(&BigDecimal::from(i.value)).map_or_else(
                            || ant_null(),
                            |it| ant_some(Object::AntDouble(AntDouble::from(it))),
                        )))
                    }
                    Object::AntInt(i) => Ok(Some(sqrt_default(&i.value).map_or_else(
                        || ant_null(),
                        |it| ant_some(Object::AntDouble(AntDouble::from(it))),
                    ))),
                    Object::AntDouble(d) => Ok(Some(sqrt_default(&d.value).map_or_else(
                        || ant_null(),
                        |it| ant_some(Object::AntDouble(AntDouble::from(it))),
                    ))),

                    it => Err(format!(
                        "expected type {:#?}, got: {}",
                        expected_types,
                        it.inspect()
                    )),
                }
            };

        let mut m = HashMap::new();

        m.insert(
            "sqrt".into(),
            Object::AntNativeFunction(create_ant_native_function(None, sqrt))
        );

        m
    })
});
