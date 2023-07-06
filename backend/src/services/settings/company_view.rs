use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Pool, Postgres};

use crate::{errors::MyError, routes::settings::company_view::CompanyCardWeb};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CompanyCardDB {
    pub id: i32,
    pub name: String,
    pub text: String,
    pub active: bool,
}

pub async fn get_by_id(db: Pool<Postgres>, id: i32) -> Result<CompanyCardDB, actix_web::Error> {
    let cards = query_as!(CompanyCardDB, "SELECT * FROM company_cards where id = $1", id)
        .fetch_one(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(cards)
}

pub async fn get_all(db: Pool<Postgres>) -> Result<Vec<CompanyCardDB>, actix_web::Error> {
    let cards = query_as!(CompanyCardDB, "SELECT * FROM company_cards")
        .fetch_all(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(cards)
}

pub async fn update(db: Pool<Postgres>, data: CompanyCardWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let card = sqlx::query_as!(
        CompanyCardDB,
        "SELECT * FROM company_cards where id = $1",
        id
    )
    .fetch_one(&db)
    .await
    .map_err(MyError::SQLxError)?;

    let id = data.id.as_ref();
    let name = data.name.as_ref();
    let text = data.text.as_ref();
    let active = data.active.as_ref();

    let query_result = sqlx::query!(
        "UPDATE company_cards SET name = $1, text = $2, active = $3 where id = $4 returning id",
        if name.is_some() {
            name.unwrap()
        } else {
            &card.name
        },
        if text.is_some() {
            text.unwrap()
        } else {
            &card.text
        },
        if active.is_some() {
            active.unwrap()
        } else {
            &card.active
        },
        data.id
    )
    .fetch_one(&db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

// TODO: Make this company_card reset function actually do something smart
pub async fn reset(db: Pool<Postgres>) -> Result<(), actix_web::Error> {
    let text = vec![
        "Logo".to_string(),
        "Name".to_string(),
        "Description".to_string(),
        "Contacts".to_string(),
        "Did you know".to_string(),
        "Divisions".to_string(),
        "Business Areas".to_string(),
        "Offering".to_string(),
        "Looking for".to_string(),
        "Website".to_string(),
        "Map".to_string(),
        "Summer job".to_string(),
        "Notes".to_string(),
        "CHARMtalks".to_string(),
        "Language".to_string(),
        "Fair Areas".to_string(),
    ];

    let name = vec![
        "logo".to_string(),
        "name".to_string(),
        "desc".to_string(),
        "contacts".to_string(),
        "didyouknow".to_string(),
        "tag_divisions".to_string(),
        "tag_business_areas".to_string(),
        "tag_offering".to_string(),
        "tag_looking_for".to_string(),
        "website".to_string(),
        "map".to_string(),
        "summerjob".to_string(),
        "notes".to_string(),
        "CHARMtalks".to_string(),
        "language".to_string(),
        "fair_area".to_string(),
    ];

    let active = vec![true; 16];

    query!("DELETE FROM company_cards").execute(&db)
        .await
        .map_err(MyError::SQLxError)?;
    // Reset the id counter every time this is done so that the IDs wont increment on each reset
    // probably not needed but frontend might get duplicates otherwise
    query!("ALTER SEQUENCE company_cards_id_seq RESTART WITH 1").execute(&db)
        .await
        .map_err(MyError::SQLxError)?;

    query!(
        "INSERT INTO company_cards (name, text, active) 
            (SELECT * FROM UNNEST($1::varchar[], $2::varchar[], $3::boolean[]))",
        &name,
        &text,
        &active
    ).execute(&db).await.map_err(MyError::SQLxError)?;

    Ok(())
}
