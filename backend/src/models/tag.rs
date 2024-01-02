use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct TagWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub category: Option<i32>,
}

impl Default for TagWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            icon: Default::default(),
            category: Default::default(), 
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Name,
    Icon,
    Category,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct TagDB {
    pub id: i32,
    pub name: String,
    pub icon: String,
    pub category: i32,
}
