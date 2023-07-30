use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::errors::MyError;
use crate::routes::tag::TagWeb;

use super::{is_valid_required_field, is_optional_field_or_default};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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

pub async fn create(db: &Pool<Postgres>, data: &TagWeb) -> Result<i32, actix_web::Error> {
    let name = is_valid_required_field(&data.name)?;
    let parent_tag = is_valid_required_field(&data.parent_tag)?;
    let up_votes = is_valid_required_field(&data.up_votes)?;
    let down_votes = is_valid_required_field(&data.down_votes)?;
    let crowd_sourced = is_valid_required_field(&data.crowd_sourced)?;
    let icon = is_optional_field_or_default(&data.icon, "".to_string())?;
    let division = is_valid_required_field(&data.division)?;
    let business_area = is_valid_required_field(&data.business_area)?;
    let looking_for = is_valid_required_field(&data.looking_for)?;
    let offering = is_valid_required_field(&data.offering)?;
    let language = is_valid_required_field(&data.language)?;
    let fair_area = is_valid_required_field(&data.fair_area)?;

    let query_result = sqlx::query!("INSERT INTO tags (name, parent_tag, up_votes, down_votes, crowd_sourced, icon, division, business_area, looking_for, offering, language, fair_area) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) returning id;",
name, parent_tag, up_votes, down_votes, crowd_sourced, icon, division, business_area, looking_for, offering, language, fair_area)
        .fetch_one(db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn update(db: Pool<Postgres>, data: TagWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let tag = sqlx::query_as!(TagDB, "SELECT * FROM tags where id = $1", id)
        .fetch_one(&db)
        .await
        .map_err(MyError::SQLxError)?;

    let name = data.name.as_ref();
    let parent_tag = data.parent_tag.as_ref();
    let up_votes = data.up_votes.as_ref();
    let down_votes = data.down_votes.as_ref();
    let crowd_sourced = data.crowd_sourced.as_ref();
    let icon = data.icon.as_ref();
    let division = data.division.as_ref();
    let business_area = data.business_area.as_ref();
    let looking_for = data.looking_for.as_ref();
    let offering = data.offering.as_ref();
    let language = data.language.as_ref();
    let fair_area = data.fair_area.as_ref();

    let query_result =
        sqlx::query!("UPDATE tags SET name = $1, parent_tag = $2, up_votes = $3, down_votes = $4, crowd_sourced = $5, icon = $6, division = $7, business_area = $8, looking_for = $9, offering = $10, language = $11, fair_area = $12 where id = $13 returning id",
                     if name.is_some() {name.unwrap()} else {&tag.name},
                     if parent_tag.is_some() {parent_tag.unwrap()} else {&tag.parent_tag},
                     if up_votes.is_some() {up_votes.unwrap()} else {&tag.up_votes},
                     if down_votes.is_some() {down_votes.unwrap()} else {&tag.down_votes},
                     if crowd_sourced.is_some() {crowd_sourced.unwrap()} else {&tag.crowd_sourced},
                     if icon.is_some() {icon.unwrap()} else {&tag.icon},
                     if division.is_some() {division.unwrap()} else {&tag.division},
                     if business_area.is_some() {business_area.unwrap()} else {&tag.business_area},
                     if looking_for.is_some() {looking_for.unwrap()} else {&tag.looking_for},
                     if offering.is_some() {offering.unwrap()} else {&tag.offering},
                     if language.is_some() {language.unwrap()} else {&tag.language},
                     if fair_area.is_some() {fair_area.unwrap()} else {&tag.fair_area},
                     data.id)
        .fetch_one(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn delete(db: Pool<Postgres>, id: i32) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM tags WHERE id = $1", id)
        .execute(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}

pub async fn get_all(db: Pool<Postgres>) -> Result<Vec<TagDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(TagDB, "SELECT * FROM tags")
        .fetch_all(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_id(db: Pool<Postgres>, id: i32) -> Result<TagDB, actix_web::Error> {
    let query_result = sqlx::query_as!(TagDB, "SELECT * FROM tags where id = $1", id)
        .fetch_one(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}
