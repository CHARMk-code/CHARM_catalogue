use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use actix_multipart::form::json::Json;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct FairMapWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub background: Option<String>,
    pub styling: Option<serde_json::Value>
}

impl Default for FairMapWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            background: Default::default(),
            styling: Default::default()
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Name,
    Background,
    Styling
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct FairMapDB {
    pub id: i32,
    pub name: String,
    pub background: String,
    //pub mapGeometry: Option<Vec<i32>>
    pub styling: serde_json::Value
}

impl Hash for FairMapDB {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
        self.name.hash(state);
        self.background.hash(state);
        self.styling.as_str().hash(state);
    }
}

/*
#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    MapRef,
    Postion,
    Category,
    RefId,
    Styling
}
*/

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct MapGeomartyDB {
    pub id: i32,
    pub map_ref: i32,
    pub postion: Vec<i32>,
    pub category: String,
    pub refId: i32,
    pub styling: serde_json::Value
}


impl Hash for MapGeomartyDB {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
        self.map_ref.hash(state);
        self.postion.hash(state);
        self.category.hash(state);
        self.refId.hash(state);
        self.styling.as_str().hash(state);
    }
}
