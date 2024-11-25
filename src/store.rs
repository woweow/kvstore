use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use serde_json::Value as JsonValue;
use crate::types::Value;

pub struct KvStore {
    data: HashMap<String, Value>,
    file_path: Option<String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            file_path: Some("kv_store.json".to_string()),
        }
    }

    pub fn with_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            data: HashMap::new(),
            file_path: Some(path.as_ref().to_str().unwrap().to_string()),
        }
    }

    pub fn in_memory() -> Self {
        Self {
            data: HashMap::new(),
            file_path: None,
        }
    }

    pub fn set(&mut self, key: String, value: String, ttl: Option<u64>) -> Result<(), String> {
        let data = serde_json::from_str(&value).unwrap_or(JsonValue::String(value));
        
        let expires_at = ttl.map(|seconds| {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() + seconds
        });

        self.data.insert(key, Value { data, expires_at });
        self.save_to_file();
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        self.remove_expired();
        self.data.get(key).map(|v| v.data.to_string())
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.remove_expired();
        let result = self.data.remove(key).map(|v| v.data.to_string());
        self.save_to_file();
        result
    }

    pub fn get_all(&self) -> Vec<(String, Value)> {
        self.data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn remove_expired(&mut self) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.data.retain(|_, value| {
            value.expires_at.map_or(true, |expires_at| expires_at > now)
        });
        
        self.save_to_file();
    }

    fn save_to_file(&self) {
        if let Some(path) = &self.file_path {
            let json = serde_json::to_string(&self.data).unwrap();
            fs::write(path, json).unwrap();
        }
    }

    fn load_from_file(&mut self) {
        if let Some(path) = &self.file_path {
            if let Ok(contents) = fs::read_to_string(path) {
                if let Ok(data) = serde_json::from_str(&contents) {
                    self.data = data;
                }
            }
        }
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
} 