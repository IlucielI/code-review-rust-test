use std::collections::HashMap;
use std::sync::RwLock;

pub struct TaskService {
    store: RwLock<HashMap<String, String>>,
}

impl TaskService {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_task(&self, id: String, title: String) {
        if let Ok(mut lock) = self.store.write() {
            lock.insert(id, title);
        }
    }

    pub fn get_task(&self, id: &str) -> Option<String> {
        self.store.read().ok()?.get(id).cloned()
    }
}
