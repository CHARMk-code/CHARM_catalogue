use sqlx::{Pool, Postgres, Error};

use crate::{routes, services};

use chrono::DateTime;
use chrono::offset::Utc;

#[sqlx::test(fixtures("Feedback"))]
async fn valid_update_on_existing_feedback_should_update_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {

    // Setup
    let initial_db_feedback = sqlx::query!("SELECT * from feedback where title = $1", "Feedback 1")
        .fetch_one(&db).await?;

    let updated_feedback = routes::feedback::FeedbackWeb {
        id: Some(initial_db_feedback.id),
        title: Some("Updated Feedback 1".to_string()),
        text: None,
        meta: None,
        received: None,
        important: Some(true),
        archived: None 
    };


    // What's tested
    let updated_id = services::feedback::update(db.clone(), updated_feedback).await.unwrap();

    // Checking it's correct
    let expected_feedback = services::feedback::FeedbackDB {
        id: initial_db_feedback.id,
        title: "Updated Feedback 1".to_string(),
        text: "This is some great feedback".to_string(),
        meta: "{}".to_string(),
        received: DateTime::parse_from_str("2023-06-25 21:00:00+02", "%F %T%#z").unwrap().into(),
        important: true,
        archived: false 
    };

    let updated_db_feedback = sqlx::query_as!(services::feedback::FeedbackDB, "SELECT * FROM feedback where id = $1", updated_id)
        .fetch_one(&db).await?;

    assert_eq!(expected_feedback, updated_db_feedback, "Making sure the feedback has been properly updated in the database");

    Ok(())
}

#[sqlx::test(fixtures("Feedback"))]
async fn creating_a_valid_feedback_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let new_feedback = routes::feedback::FeedbackWeb {
        id: None,
        title: Some("New Feedback".to_string()),
        text: Some("New but bad feedback".to_string()),
        meta: Some("{}".to_string()),
        received: Some(DateTime::parse_from_str("2023-06-25 23:00:00+02", "%F %T%#z").unwrap().into()),
        important: Some(false),
        archived: Some(false)

    };

    // What's tested
    let created_id = services::feedback::create(db.clone(), new_feedback.clone()).await.unwrap();

    let created_db_feedback = sqlx::query_as!(services::feedback::FeedbackDB,"SELECT * FROM feedback where id = $1", created_id)
        .fetch_one(&db).await?;

    // Checking it's correct
    let expected_feedback = services::feedback::FeedbackDB {
        id: created_id,
        title: "New Feedback".to_string(),
        text: "New but bad feedback".to_string(),
        meta: "{}".to_string(),
        received: DateTime::parse_from_str("2023-06-25 23:00:00+02", "%F %T%#z").unwrap().into(),
        important: false,
        archived: false
    };

    assert_eq!(expected_feedback, created_db_feedback, "Making sure the feedback has been properly created in the database");

    Ok(())
}

#[sqlx::test(fixtures("Feedback"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_feedback = sqlx::query!("SELECT * from feedback where title = $1", "Feedback 1" )
        .fetch_one(&db).await?;

    // What's tested
    let affected_rows = services::feedback::delete(db.clone(), initial_db_feedback.id).await;
    assert!(affected_rows.is_ok());
    assert_eq!(affected_rows.unwrap(), 1);

    let remaining_db_feedback = sqlx::query_as!(services::feedback::FeedbackDB,"SELECT * FROM feedback where id = $1", initial_db_feedback.id)
        .fetch_one(&db).await;
    assert!(remaining_db_feedback.is_err());

    // Check that the other 2 rows of the fixture are left
    let remaing_rows = services::feedback::get_all(db.clone()).await;
    assert!(remaing_rows.is_ok());
    assert_eq!(remaing_rows.unwrap().len(), 2);


    Ok(())
}

#[sqlx::test(fixtures("Feedback"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error>{
    //Setup
    let initial_db_feedback = sqlx::query_as!(services::feedback::FeedbackDB,"SELECT * from feedback where title = $1", "Feedback 1" )
        .fetch_one(&db).await?;

    // What's tested
    let result = services::feedback::get_by_id(db.clone(),initial_db_feedback.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_feedback, result.unwrap());
    Ok(())
}


#[sqlx::test()]
async fn get_by_id_when_no_matching_feedback_should_fail(db: Pool<Postgres>) -> Result<(), Error>{
    // What's tested
    let result = services::feedback::get_by_id(db.clone(),7).await;
    assert!(result.is_err());

    Ok(())
}
