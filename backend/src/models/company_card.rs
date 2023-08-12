use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CompanyCardWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub text: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CompanyCardDB {
    pub id: i32,
    pub name: String,
    pub text: String,
    pub active: bool,
}
