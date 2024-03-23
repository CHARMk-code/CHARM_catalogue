use sqlx::{Pool, Postgres};

use crate::{
    errors::MyError,
    models::tag_category::{TagCategoryDB, TagCategoryWeb},
};

use super::{is_optional_field_or_default, is_valid_required_field};

pub async fn create(db: &Pool<Postgres>, data: &TagCategoryWeb) -> Result<i32, actix_web::Error> {
    let name = is_valid_required_field(&data.name)?;

    let query_result = sqlx::query!("INSERT INTO tag_category (name) VALUES ($1) returning id;",
name)
        .fetch_one(db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn update(db: &Pool<Postgres>, data: TagCategoryWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let tag_category = sqlx::query_as!(TagCategoryDB, "SELECT * FROM tag_category where id = $1", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    let name = data.name.as_ref();

    let query_result =
        sqlx::query!("UPDATE tag_category SET name = $1 where id = $2 returning id",
                     if name.is_some() {name.unwrap()} else {&tag_category.name},
                     data.id)
        .fetch_one(db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn delete(db: &Pool<Postgres>, id: i32) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM tag_category WHERE id = $1", id)
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}

pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<TagCategoryDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(TagCategoryDB, "SELECT * FROM tag_category")
        .fetch_all(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_id(db: &Pool<Postgres>, id: i32) -> Result<TagCategoryDB, actix_web::Error> {
    let query_result = sqlx::query_as!(TagCategoryDB, "SELECT * FROM tag_category where id = $1", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}
