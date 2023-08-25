use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JSONBlobWeb {
    pub name: Option<String>,
    pub blob: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct JSONBlobDB {
    pub id: i32,
    pub name: String,
    pub blob: serde_json::Value,
}
