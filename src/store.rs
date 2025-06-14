use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Store {
    storage: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            storage: HashMap::new(),
        }
    }

    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<&String> {
        self.storage.get(key.as_ref())
    }

    pub fn set<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> Option<String> {
        self.storage.insert(key.as_ref().to_string(), value.as_ref().to_string())
    }

    pub fn del<K: AsRef<str>>(&mut self, key: K) -> Option<String> {
        self.storage.remove(key.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_set_get_delete_and_get() {
        let mut store = Store::new();
        let key = "the key";
        let val = "the val";
        store.set(&key, &val);

        assert_eq!(store.get(key).map(|s| s.as_str()), Some(val));

        store.del(&key);

        assert_eq!(store.get(key).map(|s| s.as_str()), None);
    }

    #[test]
    fn test_overwrite_set() {
        let mut store = Store::new();

        store.set("key1", "hello");
        assert_eq!(store.get("key1").map(|s| s.as_str()), Some("hello"));

        store.set("key1", "new value");
        assert_eq!(store.get("key1").map(|s| s.as_str()), Some("new value"));
    }

    #[test]
    fn test_del_with_non_existent_key() {
        let mut store = Store::new();
        assert_eq!(store.del("hello"), None);
    }
}
