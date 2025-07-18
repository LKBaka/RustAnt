
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

impl<K: PartialEq, V: PartialEq> Pair<K, V> {
    pub fn new(key: K, value: V) -> Pair<K, V> {
        Pair { key, value }
    }

    // pub fn to_string(self: &Pair<K, V>) -> String {
    //     format!("{}: {}", self.key.to_string(), self.value.to_string())
    // }

    pub fn eq(&self, other: Pair<K, V>) -> bool {
        other.key == self.key && other.value == self.value
    }
}