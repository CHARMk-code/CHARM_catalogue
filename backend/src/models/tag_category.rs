use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct TagCategoryWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    // Icon?
}

impl Default for TagCategoryWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Name,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct TagCategoryWeb {
    pub id: i32,
    pub name: String,
}
