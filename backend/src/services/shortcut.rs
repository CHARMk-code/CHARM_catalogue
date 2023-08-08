use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::errors::MyError;
use crate::routes::shortcut::ShortcutWeb;

use super::{is_valid_required_field, is_optional_field_or_default};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ShortcutDB {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub link: String,
    pub icon: String,
}

pub async fn create(db: &Pool<Postgres>, data: &ShortcutWeb) -> Result<i32, actix_web::Error> {
    let name = is_valid_required_field(&data.name)?;
    let description = is_optional_field_or_default(&data.description, "".to_string())?;
    let icon = is_valid_required_field(&data.icon)?;
    let link = is_valid_required_field(&data.link)?;

    let query_result = sqlx::query!("INSERT INTO shortcuts (name, description, link, icon) VALUES ($1, $2, $3, $4) returning id",
        name, description, link, icon)
        .fetch_one(db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn update(db: &Pool<Postgres>, data: &ShortcutWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let shortcut = sqlx::query_as!(ShortcutDB, "SELECT * FROM shortcuts where id = $1 ", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    let name = data.name.as_ref();
    let description = data.description.as_ref();
    let link = data.link.as_ref();
    let icon = data.icon.as_ref();

    let query_result =
        sqlx::query!("UPDATE shortcuts SET name = $1, description = $2, link = $3, icon = $4 where id = $5 returning id",
                if name.is_some() {name.unwrap()} else {&shortcut.name},
            if description.is_some() {description.unwrap()} else {&shortcut.description},
            if link.is_some() {link.unwrap()} else {&shortcut.link},
            if icon.is_some() {icon.unwrap()} else {&shortcut.icon},
        data.id)
        .fetch_one(db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn delete(db: &Pool<Postgres>, id: i32) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM shortcuts WHERE id = $1", id)
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}

pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<ShortcutDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(ShortcutDB, "SELECT * FROM shortcuts")
        .fetch_all(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_id(db: &Pool<Postgres>, id: i32) -> Result<ShortcutDB, actix_web::Error> {
    let query_result = sqlx::query_as!(ShortcutDB, "SELECT * FROM shortcuts where id = $1 ", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}
