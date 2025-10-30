use std::rc::Rc;

use super::object::ObjectType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeHint {
    pub types: Vec<ObjectType>,
}

impl TypeHint {
    pub fn new(types: Vec<ObjectType>) -> Self {
        Self { types }
    }

    pub fn has_type(&self, object_type: &ObjectType) -> bool {
        self.types.contains(object_type)
    }

    pub fn push_type(&self, object_type: ObjectType) -> Self {
        let mut new_types = self.types.clone();

        if !new_types.contains(&object_type) {
            new_types.push(object_type);
        }

        Self { types: new_types }
    }

    pub fn add_type(&mut self, object_type: ObjectType) {
        if !self.types.contains(&object_type) {
            self.types.push(object_type);
        }
    }

    pub fn remove_type(&mut self, object_type: &ObjectType) {
        self.types.retain(|t| t != object_type);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeHintMap {
    pub map: std::collections::HashMap<Rc<str>, TypeHint>,
}

impl TypeHintMap {
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn from_map(map: std::collections::HashMap<Rc<str>, TypeHint>) -> Self {
        Self { map }
    }

    pub fn push_hint(&self, key: Rc<str>, hint: TypeHint) -> Self {
        let mut map = self.map.clone();

        map.insert(key, hint);

        Self { map }
    }

    pub fn add_hint(&mut self, key: Rc<str>, hint: TypeHint) {
        self.map.insert(key, hint);
    }

    pub fn get_hint(&self, key: &str) -> Option<&TypeHint> {
        self.map.get(key)
    }

    pub fn remove_hint(&mut self, key: &str) {
        self.map.remove(key);
    }
}

impl IntoIterator for &'static TypeHintMap {
    type Item = (&'static Rc<str>, &'static TypeHint);
    type IntoIter = TypeHintIterator;

    fn into_iter(self) -> Self::IntoIter {
        TypeHintIterator::new(self)
    }
}

pub struct TypeHintIterator {
    inner: std::collections::hash_map::Iter<'static, Rc<str>, TypeHint>,
}

impl TypeHintIterator {
    pub fn new(map: &'static TypeHintMap) -> Self {
        Self {
            inner: map.map.iter(),
        }
    }
}

impl Iterator for TypeHintIterator {
    type Item = (&'static Rc<str>, &'static TypeHint);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, v))
    }
}

#[macro_export]
macro_rules! type_hint {
    ($($type:expr),*) => {
        TypeHint::new(vec![$($type.into()),*])
    };
}

#[macro_export]
macro_rules! type_hint_map {
    ($($key:expr => $value:expr),*) => {
        TypeHintMap::from_map(std::collections::HashMap::from([
            $(($key.into(), $value)),*
        ]))
    };
}
