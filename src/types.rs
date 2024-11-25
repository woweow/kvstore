use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub data: JsonValue,
    pub expires_at: Option<u64>,
} 