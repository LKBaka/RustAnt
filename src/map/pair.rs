
#[derive(PartialEq, Eq)]
pub struct Pair<K, V> {
    pub key: K,
    pub value: V,
}

impl<K: Clone, V: Clone> Clone for Pair<K, V> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            value: self.value.clone()
        }
    }
}

impl<K: PartialEq + Eq, V: PartialEq + Eq> Pair<K, V> {
    pub fn new(key: K, value: V) -> Pair<K, V> {
        Pair { key, value }
    }
}