use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct LayoutWeb {
    pub id: Option<i32>,
    pub image: Option<String>,
    pub active: Option<bool>,
    pub placement: Option<i32>,
}

impl Default for LayoutWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            image: Default::default(),
            active: Default::default(),
            placement: Default::default(),
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Image,
    Active,
    Placement,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct LayoutDB {
    pub id: i32,
    pub image: String,
    pub active: bool,
    pub placement: i32,
}
