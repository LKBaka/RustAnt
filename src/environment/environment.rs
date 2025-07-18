use std::cell::RefCell;
use std::cmp::PartialEq;
use std::ops::Deref;
use std::rc::Rc;

use crate::environment::data::Data;
use crate::map;
use crate::map::map::Map;
use crate::object::object::{Object, FUNCTION};
use crate::object::utils::{create_error, create_error_with_name};

#[derive(Clone)]
pub struct Environment {
    pub map: Map<String, Data>,
    pub func_map: Map<String, Vec<Data>>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new_with_outer(outer: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            map: map!(),
            func_map: map!(),
            outer: Some(outer),
        }
    }

    pub fn new() -> Environment {
        Environment {
            map: map!(),
            func_map: map!(),
            outer: None,
        }
    }

    pub fn drop_all(&mut self) {
        self.map.clear();
        self.func_map.clear();
    }

    pub fn remove_data(&mut self, value: Data) {
        self.map.pairs.retain(|pair| pair.value != value);

        for func_pair in &mut self.func_map.pairs {
            func_pair.value.retain(|v| *v != value);
        }

        self.func_map.pairs.retain(|pair| !pair.value.is_empty());
    }

    pub fn remove_obj(&mut self, value: &Object) {
        self.map.pairs.retain(|pair| !((&pair.value.data) == value));

        for func_pair in &mut self.func_map.pairs {
            func_pair.value.retain(|v| !(&v.data == value));
        }

        self.func_map.pairs.retain(|pair| !pair.value.is_empty());
    }

    pub fn create(&mut self, key: &str, value: Data) -> Option<Object> {
        if self.map.contains_key(&key.to_string()) {
            return Some(
                create_error_with_name("NameError", format!("identifier '{}' already exists", key))
            );
        }

        if value.data.get_type() != FUNCTION {
            self.map.add(key.to_string(), value);
            return None;
        }

        if self.func_map.contains_key(&key.to_string()) {
            let mut data_array = self.func_map.get(&key.to_string()).unwrap().clone();
            data_array.push(value);
            
            self.func_map.set(key.to_string(), data_array);
        } else {
            self.func_map.add(key.to_string(), vec![value]);
        }

        None
    }

    pub fn set(&mut self, key: &str, value: Data) -> Option<Object> {
        if self.map.contains_key(&key.to_string()) {
            if self.map.get(&key.to_string()).unwrap().info.readonly {
                return Some(create_error_with_name(
                    "ReadOnlyError", format!("'{}' is readonly", key)
                ))
            }

            self.map.set(key.to_string(), value);
            return None;
        }

        Some(
            create_error(format!("cannot find variable '{}'", key))
        )
    }

    pub fn set_value(&mut self, key: &str, value: Object) -> Option<Object> {
        if self.map.contains_key(&key.to_string()) {
            let data = self.get_data(key);

            if let Some(it) = &data && it.info.readonly {
                return Some(create_error_with_name(
                    "ReadOnlyError", format!("'{}' is readonly", key)
                ))
            } else if let Some(mut it) = data {
                it.data = value;

                self.map.set(key.to_string(), it);
            }

            return None;
        }

        Some(
            create_error(format!("cannot find variable '{}'", key))
        )
    }

    pub fn get_data(&self, key: &str) -> Option<Data> {
        if self.map.contains_key(&key.to_string()) {
            return self.map.get(&key.to_string())
        }

        if self.func_map.contains_key(&key.to_string()) {
            return Option::from(self.func_map.get(&key.to_string()).unwrap()[0].clone())
        }

        if let Some(outer) = &self.outer {
            return outer.borrow().get_data(key)
        }

        None
    }

    pub fn get_values(&self, key: &str) -> Option<Vec<Object>> {
        if self.map.contains_key(&key.to_string()) {
            return Some(vec![self.map.get(&key.to_string()).unwrap().data.clone()])
        }

        if self.func_map.contains_key(&key.to_string()) {
            let mut values = vec![];
            let data_array = self.func_map.get(&key.to_string()).unwrap();

            for data in data_array {
                values.push(data.data.clone());
            }

            return Some(values)
        }

        if let Some(outer) = &self.outer {
            return outer.borrow().get_values(key)
        }

        None
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        if self.map.contains_key(&key.to_string()) {
            return Some(self.map.get(&key.to_string()).unwrap().data.clone())
        }

        if self.func_map.contains_key(&key.to_string()) {
            return Some(self.func_map.get(&key.to_string()).unwrap()[0].data.clone())
        }

        if let Some(outer) = &self.outer {
            return outer.borrow().get(key)
        }


        None
    }

    pub fn in_place_fusion(&mut self, other: &Environment) {
        for pair in &other.map.pairs {
            if self.map.contains_key(&pair.key) {
                self.set(&pair.key, pair.value.clone());
            } else {
                self.create(&pair.key, pair.value.clone());
            }
        }
    }

    pub fn fusion(&self, other: Environment) -> Environment {
        let mut env = self.clone();

        for pair in &other.map.pairs {
            if env.map.contains_key(&pair.key) {
                env.set(&pair.key, pair.value.clone());
            } else {
                env.create(&pair.key, pair.value.clone());
            }
        }

        env
    }

    pub fn to_string(&self) -> String {
        let mut s = String::from("[");

        let mut str_list = vec![];
        for pair in self.map.pairs.clone() {
            str_list.push(format!("{}: {}", pair.key, pair.value.to_string()));
        }

        s.push_str(str_list.join(", ").deref());
        s.push_str("]");

        s
    }
}

impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        other.map == self.map && other.func_map == self.func_map.clone()
    }
}

impl Eq for Environment {}