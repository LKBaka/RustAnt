use std::any::Any;

use indexmap::IndexMap;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::id_counter::next_id;
use crate::object::object::{IAntObject, ObjectType, ARRAY, HASH_MAP, STRING};

#[derive(Clone)]
pub struct AntHashMap {
    id: usize,
    pub map: indexmap::IndexMap<Object, Object>,
}

impl IAntObject for AntHashMap {
    fn get_type(&self) -> ObjectType {
        HASH_MAP.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(
            self.map.clone()
        )
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn inspect(&self) -> String {
        format!(
            "{{{}}}",
            self.map
                .iter()
                .map(
                    |(k, v)| format!(
                        "{}: {}",
                        if k.get_type() != STRING {
                            k.inspect()
                        } else {
                            format!("\"{}\"", k.inspect())
                        },
                        if v.get_type() != STRING {
                            v.inspect()
                        } else {
                            format!("\"{}\"", v.inspect())
                        },
                    )
                )
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
            || if let Some(m) = other.as_any().downcast_ref::<AntHashMap>() {
                m.map == self.map
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntHashMap);

impl From<IndexMap<Object, Object>> for AntHashMap {
    fn from(map: IndexMap<Object, Object>) -> Self {
        AntHashMap {
            id: next_id(),
            map,
        }
    }
}
