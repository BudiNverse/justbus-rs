use cht::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

#[cfg(test)]
mod test {
    use crate::hashmap::Cache;
    use std::thread;
    use std::time::Duration;

    /// Simple test to check if values
    /// get replaced and whether the returned value is not expired
    #[test]
    fn hm_test() {
        let duration = Duration::from_secs(1);
        let hm = Cache::with_ttl(duration.clone());

        // insert an entry that will expire in 1s
        hm.insert(32, "hello_32");
        thread::sleep(duration.clone());
        assert_eq!(hm.get(32), None);
        println!("{:?}", hm.get(32));

        // check if value with same key is replaced
        hm.insert(32, "hello_32_replaced");
        thread::sleep(Duration::from_millis(10));
        assert_eq!(hm.get(32), Some("hello_32_replaced"));
        println!("{:?}", hm.get(32));
    }
}

#[derive(Clone)]
struct InternalEntry<V> {
    value: V,
    expiration: Instant,
}

impl<V> InternalEntry<V> {
    pub fn new(value: V, expiration: Instant) -> Self {
        InternalEntry { value, expiration }
    }

    fn is_expired(&self) -> bool {
        Instant::now() > self.expiration
    }

    pub fn get(self) -> Option<V> {
        if self.is_expired() {
            None
        } else {
            Some(self.value)
        }
    }
}

pub struct Cache<K: Hash + Eq, V> {
    map: HashMap<K, InternalEntry<V>>,
    ttl: Duration,
}
impl<K: Hash + Eq, V: Clone> Cache<K, V> {
    pub fn with_ttl(ttl: Duration) -> Self {
        Cache {
            map: HashMap::<K, InternalEntry<V>>::new(),
            ttl,
        }
    }

    pub fn with_ttl_and_size(ttl: Duration, capacity: usize) -> Self {
        Cache {
            map: HashMap::<K, InternalEntry<V>>::with_capacity(capacity),
            ttl,
        }
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.map
            .insert(key, InternalEntry::new(value, Instant::now() + self.ttl))
            .map(|f| f.value)
    }

    pub fn get(&self, key: K) -> Option<V> {
        self.map.get(&key).and_then(|f| f.get())
    }
}
