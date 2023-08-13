use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ShortcutDB {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub link: String,
    pub icon: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct ShortcutWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub icon: Option<String>,
}

impl Default for ShortcutWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            description: Default::default(),
            link: Default::default(),
            icon: Default::default(),
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Name,
    Description,
    Link,
    Icon,
}
