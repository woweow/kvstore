use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Value {
    pub data: String,
    pub expires_at: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KvStore {
    #[serde(skip)]
    file_path: String,
    store: HashMap<String, Value>,
}

impl KvStore {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        fs::create_dir_all("storage")?;
        let store_path = Path::new("storage").join("kv_store.json");
        
        let store = if store_path.exists() {
            let contents = fs::read_to_string(&store_path)?;
            serde_json::from_str(&contents).unwrap_or_else(|_| HashMap::new())
        } else {
            HashMap::new()
        };

        let mut kv_store = KvStore {
            store,
            file_path: store_path.to_string_lossy().to_string(),
        };
        
        kv_store.cleanup_expired();
        Ok(kv_store)
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).and_then(|value| {
            if self.is_expired(value) {
                None
            } else {
                Some(value.data.clone())
            }
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn Error>> {
        self.set_with_ttl(key, value, None)
    }

    pub fn set_with_ttl(&mut self, key: String, value: String, ttl_seconds: Option<u64>) -> Result<(), Box<dyn Error>> {
        let expires_at = ttl_seconds.map(|ttl| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + ttl
        });

        self.store.insert(key, Value { 
            data: value, 
            expires_at 
        });
        self.save()?;
        Ok(())
    }

    pub fn delete(&mut self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let result = self.store.remove(key).map(|v| v.data);
        self.save()?;
        Ok(result)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let store_path = Path::new("storage").join("kv_store.json");
        if let Ok(serialized) = serde_json::to_string(&self.store) {
            fs::write(store_path, serialized)?;
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<(String, String)> {
        self.store
            .iter()
            .filter(|(_, value)| !self.is_expired(value))
            .map(|(k, v)| (k.clone(), v.data.clone()))
            .collect()
    }

    fn is_expired(&self, value: &Value) -> bool {
        if let Some(expires_at) = value.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            expires_at <= now
        } else {
            false
        }
    }

    fn cleanup_expired(&mut self) {
        let expired_keys: Vec<String> = self.store
            .iter()
            .filter(|(_, value)| self.is_expired(value))
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            self.store.remove(&key);
        }
        
        if let Err(e) = self.save() {
            eprintln!("Failed to save during cleanup: {}", e);
        }
    }

    // Helper method to get TTL information
    pub fn get_ttl(&self, key: &str) -> Option<u64> {
        self.store.get(key).and_then(|value| {
            if self.is_expired(value) {
                None
            } else {
                value.expires_at.map(|expires_at| {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    expires_at.saturating_sub(now)
                })
            }
        })
    }

    pub fn get_all(&self) -> Vec<(String, Value)> {
        self.store
            .iter()
            .filter(|(_, value)| !self.is_expired(value))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }
} 