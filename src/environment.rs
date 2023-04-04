use crate::types::{Error, Object};
use std::collections::HashMap;

pub struct Environment {
    store: HashMap<String, Box<dyn Object>>,
}

impl Environment {
    pub fn new() -> Environment {
        return Environment {
            store: HashMap::new(),
        };
    }

    pub fn update(&mut self, key: String, value: Box<dyn Object>) {
        self.store.insert(key, value);
    }

    pub fn list_keys(&self) -> Vec<&String> {
        return Vec::from_iter(self.store.keys());
    }

    pub fn has_key(&self, key: &str) -> bool {
        return self.store.contains_key(key);
    }

    pub fn get(&self, key: &str) -> Box<dyn Object> {
        let obj = self.store.get(key);
        if obj.is_none() {
            return Box::new(Error {
                message: format!("unknown identifier: {}", key),
            });
        }
        return obj.unwrap().get_box();
    }
}
