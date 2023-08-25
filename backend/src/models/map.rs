use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct MapWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub reference: Option<i32>,
}

impl Default for MapWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            image: Default::default(),
            reference: Default::default(),
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Name,
    Image,
    Reference,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct MapDB {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub reference: i32,
}
