use std::collections;

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    String(String),
    Map(Map<String, Value>),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Map<K, V> {
    map: MapImpl<K, V>,
}
type MapImpl<K, V> = collections::BTreeMap<K, V>;

impl Map<String, Value> {
    pub fn new() -> Self {
        Map { map: MapImpl::new() }
    }
}

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = collections::btree_map::IntoIter<K, V>;

    fn into_iter(self) -> collections::btree_map::IntoIter<K, V> {
        self.map.into_iter()
    }
}
