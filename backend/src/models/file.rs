use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct FileWeb {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub image: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct FileDB {
    pub id: Uuid,
    pub name: String,
    pub namespace: String,
    pub image: bool,
}
