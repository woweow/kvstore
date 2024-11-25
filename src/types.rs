use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub data: JsonValue,
    pub expires_at: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub directory: PathBuf,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("storage"),
        }
    }
} 