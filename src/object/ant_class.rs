use std::any::Any;
use std::collections::HashMap;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::id_counter::next_id;
use crate::object::object::{IAntObject, ObjectType, CLASS};

#[derive(Clone)]
pub struct AntClass {
    pub id: usize,
    pub map: HashMap<String, Object>,
}

impl AntClass {
    fn inspect_with_indent(&self, indent_level: usize) -> String {
        if self.map.is_empty() {
            return "class {}".to_string();
        }

        let indent = "  ".repeat(indent_level);
        let inner_indent = "  ".repeat(indent_level + 1);
        
        let entries: Vec<String> = self.map
            .iter()
            .map(|(key, value)| {
                let formatted_value = match value {
                    Object::AntString(s) => format!("\"{}\"", s.value),
                    Object::AntClass(class) => class.inspect_with_indent(indent_level + 1),
                    _ => value.inspect(),
                };
                format!("{}{}: {}", inner_indent, key, formatted_value)
            })
            .collect();
        if entries.is_empty() {
            "class {}".to_string()
        } else {
            format!(
                "class {{\n{}\n{}}}",
                entries.join(",\n"),
                indent
            )
        }
    }
}

impl IAntObject for AntClass {
    fn get_type(&self) -> ObjectType {
        CLASS.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        unimplemented!()
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn inspect(&self) -> String {
        self.inspect_with_indent(0)
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        match (other as &dyn Any).downcast_ref::<AntClass>() {
            Some(it) => it.map == self.map,
            None => false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntClass);

impl From<HashMap<String, Object>> for AntClass {
    fn from(value: HashMap<String, Object>) -> Self {
        Self {
            id: next_id(),
            map: value
        }
    }
}