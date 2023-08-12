use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct FeedbackWeb {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub meta: Option<String>,
    pub received: Option<DateTime<Utc>>,
    pub important: Option<bool>,
    pub archived: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct FeedbackDB {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub meta: String,
    pub received: DateTime<Utc>,
    pub important: bool,
    pub archived: bool,
}
