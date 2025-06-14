use std::collections::HashMap;

pub struct Store {
    storage: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            storage: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }

    pub fn set(&mut self, key: &String, value: &String) {
        self.storage.insert(key.to_owned(), value.to_owned());
    }

    pub fn del(&mut self, key: &String) -> Option<String> {
        self.storage.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_set_get_delete_and_get() {
        let mut store = Store::new();
        let key = "the key".to_string();
        let val = "the val".to_string();
        store.set(&key, &val);

        assert_eq!(store.get(&key), Some(&val))
    }
}
