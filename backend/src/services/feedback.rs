use sqlx::{Pool, Postgres};

use crate::{
    errors::MyError,
    models::feedback::{FeedbackDB, FeedbackWeb},
};

use super::is_valid_required_field;

pub async fn create(db: Pool<Postgres>, data: FeedbackWeb) -> Result<i32, actix_web::Error> {
    let title = is_valid_required_field(&data.title)?;
    let text = is_valid_required_field(&data.text)?;
    let meta = is_valid_required_field(&data.meta)?;
    let received = is_valid_required_field(&data.received)?;
    let important = is_valid_required_field(&data.important)?;
    let archived = is_valid_required_field(&data.archived)?;

    let query_result = sqlx::query!("INSERT INTO feedback (title, text, meta, received, important, archived) VALUES ($1, $2, $3, $4, $5, $6) returning id;",
title, text, meta, received, important, archived)
        .fetch_one(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn update(db: Pool<Postgres>, data: FeedbackWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let feedback = sqlx::query_as!(FeedbackDB, "SELECT * FROM feedback where id = $1", id)
        .fetch_one(&db)
        .await
        .map_err(MyError::SQLxError)?;

    let title = data.title.as_ref();
    let text = data.text.as_ref();
    let meta = data.meta.as_ref();
    let received = data.received.as_ref();
    let important = data.important.as_ref();
    let archived = data.archived.as_ref();

    let query_result =
        sqlx::query!("UPDATE feedback SET title = $1, text = $2, meta = $3, received = $4, important = $5, archived = $6 where id = $7 returning id",
                     if title.is_some() {title.unwrap()} else {&feedback.title},
                     if text.is_some() {text.unwrap()} else {&feedback.text},
                     if meta.is_some() {meta.unwrap()} else {&feedback.meta},
                     if received.is_some() {received.unwrap()} else {&feedback.received},
                     if important.is_some() {important.unwrap()} else {&feedback.important},
                     if archived.is_some() {archived.unwrap()} else {&feedback.archived},
                     data.id)
        .fetch_one(&db).await.map_err(MyError::SQLxError)?;

    Ok(query_result.id)
}

pub async fn delete(db: Pool<Postgres>, id: i32) -> Result<u64, actix_web::Error> {
    let query_result = sqlx::query!("DELETE FROM feedback WHERE id = $1", id)
        .execute(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result.rows_affected())
}

pub async fn get_all(db: Pool<Postgres>) -> Result<Vec<FeedbackDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(FeedbackDB, "SELECT * FROM feedback")
        .fetch_all(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}

pub async fn get_by_id(db: Pool<Postgres>, id: i32) -> Result<FeedbackDB, actix_web::Error> {
    let query_result = sqlx::query_as!(FeedbackDB, "SELECT * FROM feedback where id = $1", id)
        .fetch_one(&db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(query_result)
}
