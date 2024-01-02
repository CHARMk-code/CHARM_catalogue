use sqlx::{Pool, Postgres};

use crate::{
    errors::MyError,
    models::map::{FairMapDB, FairMapWeb},
};

use super::is_valid_required_field;

pub async fn create(db: &Pool<Postgres>, data: &FairMapWeb) -> Result<i32, actix_web::Error> {
    let name = is_valid_required_field(&data.name)?;
    let background = is_valid_required_field(&data.background)?;
    let map_data = is_valid_required_field(&data.map_data)?;

    let query_result = sqlx::query!(
        "INSERT INTO fair_maps (name, background, map_data) VALUES ($1, $2, $3) returning id;",
        name,
        background,
        map_data
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn update(db: &Pool<Postgres>, data: &FairMapWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let map = sqlx::query_as!(FairMapDB, "SELECT * FROM fair_maps where id = $1", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    let name = data.name.as_ref();
    let background = data.background.as_ref();
    let map_data = data.map_data.as_ref();

    let query_result = sqlx::query!(
        "UPDATE fair_maps SET name = $1, background = $2, map_data = $3 where id = $4 returning id",
        if name.is_some() {
            name.unwrap()
        } else {
            &map.name
        },
        if background.is_some() {
            background.unwrap()
        } else {
            &map.background
        },
        if map_data.is_some() {
            map_data.unwrap()
        } else {
            &map.map_data
        },
        data.id
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn delete(db: &Pool<Postgres>, id: i32) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM fair_maps WHERE id = $1", id)
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}

pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<FairMapDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(FairMapDB, "SELECT * FROM fair_maps")
        .fetch_all(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_id(db: &Pool<Postgres>, id: i32) -> Result<FairMapDB, actix_web::Error> {
    let query_result = sqlx::query_as!(FairMapDB, "SELECT * FROM fair_maps where id = $1", id)
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}
