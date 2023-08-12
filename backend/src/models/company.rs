use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct CompanyWeb {
    pub id: Option<i32>,
    pub last_updated: Option<DateTime<Utc>>,
    pub active: Option<bool>,
    pub charmtalk: Option<bool>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub unique_selling_point: Option<String>,
    pub summer_job_description: Option<String>, // TODO: Allow publishing of generic job listings on the
    pub summer_job_link: Option<String>,        // company page
    pub summer_job_deadline: Option<DateTime<Utc>>,
    pub contacts: Option<String>,
    pub contact_email: Option<String>,
    pub employees_world: Option<i32>,
    pub employees_sweden: Option<i32>,
    pub website: Option<String>,
    pub talk_to_us_about: Option<String>,
    pub logo: Option<String>,
    pub map_image: Option<i32>,
    pub booth_number: Option<i32>,
    pub tags: Option<Vec<i32>>,
}

impl Default for CompanyWeb {
    fn default() -> Self {
        Self {
            id: Default::default(),
            last_updated: Default::default(),
            active: Default::default(),
            charmtalk: Default::default(),
            name: Default::default(),
            description: Default::default(),
            unique_selling_point: Default::default(),
            summer_job_description: Default::default(),
            summer_job_link: Default::default(),
            summer_job_deadline: Default::default(),
            contacts: Default::default(),
            contact_email: Default::default(),
            employees_world: Default::default(),
            employees_sweden: Default::default(),
            website: Default::default(),
            talk_to_us_about: Default::default(),
            logo: Default::default(),
            map_image: Default::default(),
            booth_number: Default::default(),
            tags: Default::default(),
        }
    }
}

#[derive(EnumIter, EnumString, Display, Debug, PartialEq, Eq, Hash)]
pub enum RequiredField {
    Id,
    Active,
    Charmtalk,
    Name,
    Description,
    Uniquesellingpoint,
    Summerjobdescription,
    Summerjoblink,
    Summerjobdeadline,
    Contacts,
    Contactemail,
    Employeesworld,
    Employeessweden,
    Website,
    Talktousabout,
    Logo,
    Mapimage,
    Boothnumber,
    Tags,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct CompanyDB {
    pub id: i32,
    pub last_updated: DateTime<Utc>,
    pub active: bool,
    pub charmtalk: bool,
    pub name: String,
    pub description: String,
    pub unique_selling_point: String,
    pub summer_job_description: String, // Allow publishing of generic job listings on the
    pub summer_job_link: String,        // company page
    pub summer_job_deadline: DateTime<Utc>,
    pub contacts: String,
    pub contact_email: String,
    pub employees_world: i32,
    pub employees_sweden: i32,
    pub website: String,
    pub talk_to_us_about: String,
    pub logo: String,
    pub map_image: i32,
    pub booth_number: i32,
    pub tags: Option<Vec<i32>>,
}
