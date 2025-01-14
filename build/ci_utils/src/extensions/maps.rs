use crate::prelude::*;

use std::collections::HashMap;



// trait Foo<'a, K, V> = FnOnce(&'a K) -> Future<Output = Result<V>>;

pub async fn get_or_insert<K, V, F, R>(map: &mut HashMap<K, V>, key: K, f: F) -> Result<&V>
where
    K: Eq + Hash,
    // TODO [mwu] It would be better if R would be allowed to live only for 'a lifetime.
    //            No idea how to express this.
    for<'a> F: FnOnce(&'a K) -> R,
    R: Future<Output = Result<V>>, {
    use std::collections::hash_map::Entry;
    match map.entry(key) {
        Entry::Occupied(occupied) => Ok(occupied.into_mut()),
        Entry::Vacant(vacant) => {
            let value = f(vacant.key()).await?;
            Ok(vacant.insert(value))
        }
    }
}
