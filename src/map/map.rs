use crate::map::pair::Pair;

#[derive(PartialEq, Eq)]
pub struct Map<K, V> {
    pub pairs: Vec<Pair<K, V>>
}

impl<K: Clone, V: Clone> Clone for Map<K, V> {
    fn clone(&self) -> Self {
        let pairs: Vec<Pair<K, V>> = self.pairs.clone();

        Self {
            pairs
        }
    }
}

impl<K: Clone + Eq, V: Clone + Eq> Map<K, V> {
    pub fn new() -> Map<K, V> {
        Map { pairs: vec![] }
    }

    pub fn clear(&mut self) {
        self.pairs = vec![];
    }

    pub fn add(&mut self, key: K, value: V) {
        self.pairs.push(Pair::new(key.clone(), value.clone()));
    }

    pub fn get(&self, k: &K) -> Option<V> {
        let pos = self.pairs.iter().position(|pair| &pair.key == k);

        if pos.is_some() {
            return Some(self.pairs[pos.unwrap()].value.clone())
        }

        None
    }

    pub fn remove(&mut self, k: K){
        let pos = self.pairs.iter().position(|pair| pair.key == k);

        if pos.is_some() {
            self.pairs.remove(pos.unwrap());
        }
    }

    pub fn set(&mut self, k: K, v: V) {
        if let Some(pos) = self.pairs.iter().position(|pair| pair.key == k) {
            self.pairs[pos].value = v;
        }
    }

    pub fn keys(&self) -> Vec<K> {
        let mut keys: Vec<K> = Vec::new();

        for pair in &self.pairs {
            keys.push(pair.key.clone());
        }

        keys.clone()
    }

    pub fn values(&self) -> Vec<V> {
        let mut values: Vec<V> = Vec::new();

        for pair in &self.pairs {
            values.push(pair.value.clone());
        }

        values.clone()
    }

    pub fn contains_key(&self, k: &K) -> bool {
        for key in self.keys() {
            if k == &key {
                return true
            }
        }

        false
    }

    pub fn eq(&self, other: Map<K, V>) -> bool {
        if other.pairs.len() != self.pairs.len() {
            return false
        }

        let mut equal = vec![];

        for i in 0..self.pairs.len() {
            equal.push(other.pairs[i] == self.pairs[i])
        }

        equal.iter().all(|equal| {*equal})
    }

    pub fn filter(&self, predicate: impl Fn(&K, &V) -> bool) -> Self {
        let mut filtered_map = Map::new();
        for pair in &self.pairs {
            if predicate(&pair.key, &pair.value) {
                filtered_map.add(pair.key.clone(), pair.value.clone());
            }
        }
        filtered_map
    }
}

#[macro_export]
macro_rules! map {
    // 创建一个空的 map
    () => {
        Map::new()
    };
    // 创建一个包含多个键值对的 Map
    ($($key:expr, $value:expr),* $(,)?) => {
        {
            use crate::map::pair::Pair;
            use crate::map::map::Map;
            
            let mut m = Map::new();
            $(
                m.pairs.push(Pair::new($key, $value));
            )*
            
            m
        }
    };
}