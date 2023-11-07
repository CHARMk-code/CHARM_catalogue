use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use actix_multipart::form::json::Json;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct FairMapWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub background: Option<String>,
    pub map_data: Option<serde_json::Value>
}

impl Default for FairMapWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            background: Default::default(),
            map_data: Default::default()
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Name,
    Background,
    MapData
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct FairMapDB {
    pub id: i32,
    pub name: String,
    pub background: String,
    pub map_data: serde_json::Value
}

impl Hash for FairMapDB {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
        self.name.hash(state);
        self.background.hash(state);
        self.map_data.as_str().hash(state);
    }
}