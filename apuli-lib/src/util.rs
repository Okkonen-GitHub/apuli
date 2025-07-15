use core::hash::Hash;
use std::collections::HashMap;

pub fn cache_insert<K, V>(cache: &mut HashMap<K, Vec<V>>, k: K, v: V)
where
    K: Eq,
    K: Hash,
    V: Clone,
{
    if cache.contains_key(&k) {
        let mut positions = cache.get(&k).unwrap().clone();
        positions.push(v);
        cache.insert(k, positions);
    } else {
        cache.insert(k, vec![v]);
    }
}
