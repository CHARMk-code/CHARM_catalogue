use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct TagWeb {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub parent_tag: Option<i32>, //TODO Remove these 3 columns
    pub up_votes: Option<i32>,
    pub down_votes: Option<i32>,
    pub crowd_sourced: Option<bool>,
    pub icon: Option<String>,
    pub division: Option<bool>,
    pub business_area: Option<bool>,
    pub looking_for: Option<bool>,
    pub offering: Option<bool>,
    pub language: Option<bool>,
    pub fair_area: Option<bool>,
}

impl Default for TagWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            parent_tag: Some(0),
            up_votes: Some(0),
            down_votes: Some(0),
            crowd_sourced: Some(false),
            icon: Default::default(),
            division: Default::default(),
            business_area: Default::default(),
            looking_for: Default::default(),
            offering: Default::default(),
            language: Default::default(),
            fair_area: Default::default(),
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Name,
    Icon,
    Division,
    Businessarea,
    Lookingfor,
    Offering,
    Language,
    Fairarea,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct TagDB {
    pub id: i32,
    pub name: String,
    pub parent_tag: i32, //TODO Remove these 3 columns
    pub up_votes: i32,
    pub down_votes: i32,
    pub crowd_sourced: bool,
    pub icon: String,
    pub division: bool,
    pub business_area: bool,
    pub looking_for: bool,
    pub offering: bool,
    pub language: bool,
    pub fair_area: bool,
}
