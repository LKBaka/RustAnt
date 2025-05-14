use std::cmp::PartialEq;
use std::ops::Deref;

use crate::environment::data::Data;
use crate::map;
use crate::map::map::Map;
use crate::object::object::{IAntObject, FUNCTION};
use crate::object::utils::{create_error, create_error_with_name};
use crate::gc::gc::{inc_ref, dec_ref};

#[derive(Clone)]
pub struct Environment {
    pub map: Map<String, Data>,
    pub func_map: Map<String, Vec<Data>>,
    pub outer: Option<Box<Environment>>,
}

impl Drop for Environment {
    fn drop(&mut self) {
        // 清理所有变量
        for pair in &self.map.pairs {
            dec_ref(&pair.value.data);
        }
        // 清理函数变量
        for pair in &self.func_map.pairs {
            for data in &pair.value {
                dec_ref(&data.data);
            }
        }
    }
}

impl Environment {
    pub fn new_with_outer(outer: Environment) -> Environment {
        Environment {
            map: map!(),
            func_map: map!(),
            outer: Some(Box::new(outer.clone())),
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

    pub fn depth(&self) -> i32 {
        return if self.outer.is_none() {
            0
        } else {
            1 + self.outer.clone().unwrap().depth()
        }
    }

    pub fn remove_data(&mut self, value: Data) {
        dec_ref(&value.data);

        self.map.pairs.retain(|pair| pair.value != value);

        for func_pair in &mut self.func_map.pairs {
            func_pair.value.retain(|v| *v != value);
        }

        self.func_map.pairs.retain(|pair| !pair.value.is_empty());
    }

    pub fn remove_obj(&mut self, value: Box<dyn IAntObject>) {
        dec_ref(&value);

        self.map.pairs.retain(|pair| !(pair.value.data == value.clone()));

        for func_pair in &mut self.func_map.pairs {
            func_pair.value.retain(|v| !(v.data == value.clone()));
        }

        self.func_map.pairs.retain(|pair| !pair.value.is_empty());
    }

    pub fn create(&mut self, key: &str, value: Data) -> Option<Box<dyn IAntObject>> {
        if self.map.contains_key(key.to_string()) {
            return Some(
                create_error_with_name("NameError", format!("variable \"{}\" already exists", key))
            );
        }

        inc_ref(&value.data);

        if value.data.get_type() != FUNCTION {
            self.map.add(key.to_string(), value);
            return None;
        }

        self.func_map.add(key.to_string(), vec![value]);

        None
    }

    pub fn set(&mut self, key: &str, value: Data) -> Option<Box<dyn IAntObject>> {
        if self.map.contains_key(key.to_string()) {
            self.map.set(key.to_string(), value);
            return None;
        }

        Some(
            create_error(format!("cannot find variable \"{}\"", key))
        )
    }

    pub fn set_value(&mut self, key: &str, value: Box<dyn IAntObject>) -> Option<Box<dyn IAntObject>> {
        if self.map.contains_key(key.to_string()) {
            let data = self.get_data(key);
            if let Some(mut it) = data {
                it.data = value;

                self.map.set(key.to_string(), it);
            }

            return None;
        }

        Some(
            create_error(format!("cannot find variable \"{}\"", key))
        )
    }

    pub fn get_data(&mut self, key: &str) -> Option<Data> {
        if self.map.contains_key(key.to_string()) {
            return Option::from(self.map.get(key.to_string()).clone())
        }

        if self.func_map.contains_key(key.to_string()) {
            return Option::from(self.func_map.get(key.to_string()).unwrap()[0].clone())
        }

        if self.outer.is_some() {
            let mut outer = self.outer.clone().unwrap();
            return outer.get_data(key)
        }

        None
    }

    pub fn get_values(&mut self, key: &str) -> Option<Vec<Box<dyn IAntObject>>> {
        if self.map.contains_key(key.to_string()) {
            return Some(vec![self.map.get(key.to_string()).unwrap().data.clone()])
        }

        if self.func_map.contains_key(key.to_string()) {
            let mut values = vec![];
            let data_array = self.func_map.get(key.to_string()).unwrap();

            for data in data_array {
                values.push(data.data)
            }

            return Some(values)
        }

        if self.outer.is_some() {
            let mut outer = self.outer.clone().unwrap();
            return outer.get_values(key)
        }

        None
    }

    pub fn get(&mut self, key: &str) -> Option<Box<dyn IAntObject>> {
        if self.map.contains_key(key.to_string()) {
            return Some(self.map.get(key.to_string()).unwrap().data.clone())
        }

        if self.func_map.contains_key(key.to_string()) {
            return Some(self.func_map.get(key.to_string()).unwrap()[0].data.clone())
        }

        if self.outer.is_some() {
            let mut outer = self.outer.clone().unwrap();
            return outer.get(key)
        }

        None
    }

    pub fn in_place_fusion(&mut self, other: Environment) {
        for pair in &other.map.pairs {
            if self.map.contains_key(pair.key.clone()) {
                self.set(pair.key.deref(), pair.value.clone());
            } else {
                self.create(pair.key.deref(), pair.value.clone());
            }
        }
    }

    pub fn fusion(&self, other: Environment) -> Environment {
        let mut env = self.clone();

        for pair in &other.map.pairs {
            if env.map.contains_key(pair.key.clone()) {
                env.set(pair.key.deref(), pair.value.clone());
            } else {
                env.create(pair.key.deref(), pair.value.clone());
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
        other.map.eq(self.map.clone()) && other.func_map.eq(self.func_map.clone())
    }
}

impl Eq for Environment {}