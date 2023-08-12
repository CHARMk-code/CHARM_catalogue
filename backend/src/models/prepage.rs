use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct PrepageWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub active: Option<bool>,
    pub mobile: Option<bool>,
    pub side: Option<String>,
    pub page: Option<i32>,
}

impl Default for PrepageWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            image: Default::default(),
            active: Default::default(),
            mobile: Default::default(),
            side: Default::default(),
            page: Default::default(),
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Name,
    Image,
    Active,
    Mobile,
    Side,
    Page,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct PrepageDB {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub active: bool,
    pub mobile: bool,
    pub side: String,
    pub page: i32,
}
