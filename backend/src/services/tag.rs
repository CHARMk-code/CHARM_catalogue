use sqlx::{Pool, Postgres};

use crate::{
    errors::MyError,
    models::tag::{TagDB, TagWeb},
};

use super::{is_optional_field_or_default, is_valid_required_field};

pub async fn create(db: &Pool<Postgres>, data: &TagWeb) -> Result<i32, actix_web::Error> {
    let name = is_valid_required_field(&data.name)?;
    let icon = is_optional_field_or_default(&data.icon, "".to_string())?;
    let category = is_valid_required_field(&data.category)?;

    let query_result = sqlx::query!("INSERT INTO tags (name, icon, category) VALUES ($1, $2, $3) returning id;",
name, icon, category)
        .fetch_one(db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn update(db: &Pool<Postgres>, data: TagWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let tag = sqlx::query_as!(TagDB, "SELECT * FROM tags where id = $1", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    let name = data.name.as_ref();
    let icon = data.icon.as_ref();
    let category = data.category.as_ref();

    let query_result =
        sqlx::query!("UPDATE tags SET name = $1, icon = $2, category = $3 where id = $4 returning id",
                     if name.is_some() {name.unwrap()} else {&tag.name},
                     if icon.is_some() {icon.unwrap()} else {&tag.icon},
                     if category.is_some() {category.unwrap()} else {&tag.category},
                     data.id)
        .fetch_one(db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn delete(db: &Pool<Postgres>, id: i32) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM tags WHERE id = $1", id)
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}

pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<TagDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(TagDB, "SELECT * FROM tags")
        .fetch_all(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_id(db: &Pool<Postgres>, id: i32) -> Result<TagDB, actix_web::Error> {
    let query_result = sqlx::query_as!(TagDB, "SELECT * FROM tags where id = $1", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}
