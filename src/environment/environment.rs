use std::cmp::PartialEq;
use std::ops::Deref;

use crate::environment::data::Data;
use crate::map;
use crate::map::map::Map;
use crate::object::ant_error::AntError;
use crate::object::object::{IAntObject, FUNCTION};

#[derive(Clone)]
pub struct Environment {
    pub map: Map<String, Data>,
    pub func_map: Map<String, Vec<Data>>,
    pub outer: Option<Box<Environment>>,
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

    pub fn remove_data(&mut self, value: Data) {
        self.map.pairs.retain(|pair| pair.value != value);

        for func_pair in &mut self.func_map.pairs {
            func_pair.value.retain(|v| *v != value);
        }

        self.func_map.pairs.retain(|pair| !pair.value.is_empty());
    }

    pub fn remove_obj(&mut self, value: Box<dyn IAntObject>) {
        self.map.pairs.retain(|pair| !pair.value.data.eq(&*value));

        for func_pair in &mut self.func_map.pairs {
            func_pair.value.retain(|v| !(*v.data).eq(&*value));
        }

        self.func_map.pairs.retain(|pair| !pair.value.is_empty());
    }

    pub fn create(&mut self, key: &str, value: Data) -> Option<Box<dyn IAntObject>> {
        // 标识符已存在
        if self.map.contains_key(key.to_string()) {
            // 报错 退出
            return Some(AntError::new_with_native_value(Box::new(format!("variable \"{}\" already exists", key))));
        }

        // 判断是否为函数
        if value.data.get_type() != FUNCTION {
            // 若不是，则创建
            self.map.add(key.to_string(), value);
            return None;
        }

        self.func_map.add(key.to_string(), vec![value]);

        None
    }

    pub fn set(&mut self, key: &str, value: Data) -> Box<dyn IAntObject> {
        if self.map.contains_key(key.to_string()) {
            self.map.set(key.to_string(), value);
        }

        AntError::new_with_native_value(Box::new(format!("cannot find variable \"{}\"", key)))
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

    pub fn fusion(&self, other: Environment) -> Environment {
        let mut env = self.clone();

        for pair in other.map.pairs {
            if env.map.contains_key(pair.key.clone()) {
                env.set(pair.key.deref(), pair.value);
            } else {
                env.create(pair.key.deref(), pair.value);
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