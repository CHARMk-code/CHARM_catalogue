use sqlx::{Error, Pool, Postgres};

use crate::{routes, services};

use chrono::DateTime;

#[sqlx::test(fixtures("Feedback"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_feedback = services::feedback::FeedbackDB {
        id: 1,
        title: "Feedback 1".to_string(),
        text: "This is some great feedback".to_string(),
        meta: "{}".to_string(),
        received: DateTime::parse_from_str("2023-06-25 21:00:00+02", "%F %T%#z")
            .unwrap()
            .into(),
        important: false,
        archived: false,
    };

    // What's tested
    let result = services::feedback::get_by_id(db.clone(), initial_db_feedback.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_feedback, result.unwrap());
    Ok(())
}

#[sqlx::test()]
async fn get_by_id_when_no_matching_feedback_should_fail(db: Pool<Postgres>) -> Result<(), Error> {
    // Setup
    let initial_row_amount = sqlx::query!("SELECT count(*) FROM feedback")
        .fetch_all(&db)
        .await?;

    let invalid_id = i32::try_from(initial_row_amount.first().unwrap().count.unwrap()).unwrap() + 1;

    // What's tested
    let queried_feedback = services::feedback::get_by_id(db.clone(), invalid_id).await;
    assert!(
        queried_feedback.is_err(),
        "Should fail when querying for nonexisting id"
    );

    Ok(())
}

#[sqlx::test(fixtures("Feedback"))]
async fn creating_a_valid_feedback_should_create_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    //Setup
    let initial_feedbacks = services::feedback::get_all(db.clone()).await.unwrap();

    let new_feedback = routes::feedback::FeedbackWeb {
        id: None,
        title: Some("New Feedback".to_string()),
        text: Some("New but bad feedback".to_string()),
        meta: Some("{}".to_string()),
        received: Some(
            DateTime::parse_from_str("2023-06-25 23:00:00+02", "%F %T%#z")
                .unwrap()
                .into(),
        ),
        important: Some(false),
        archived: Some(false),
    };

    // What's tested
    let created_query_result = services::feedback::create(db.clone(), new_feedback.clone()).await;
    assert!(
        created_query_result.is_ok(),
        "Should not fail on creation of new row"
    );

    let new_feedbacks = services::feedback::get_all(db.clone()).await.unwrap();
    let new_created_feedback: &services::feedback::FeedbackDB = new_feedbacks
        .iter()
        .filter(|r| &r.id == created_query_result.as_ref().unwrap())
        .collect::<Vec<&services::feedback::FeedbackDB>>()
        .first()
        .unwrap();
    let new_other_feedbacks: Vec<services::feedback::FeedbackDB> = new_feedbacks
        .iter()
        .cloned()
        .filter(|r| &r.id != created_query_result.as_ref().unwrap())
        .collect();

    let expected_feedback = services::feedback::FeedbackDB {
        id: created_query_result.unwrap(),
        title: "New Feedback".to_string(),
        text: "New but bad feedback".to_string(),
        meta: "{}".to_string(),
        received: DateTime::parse_from_str("2023-06-25 23:00:00+02", "%F %T%#z")
            .unwrap()
            .into(),
        important: false,
        archived: false,
    };

    assert_eq!(
        &expected_feedback, new_created_feedback,
        "Making sure the feedback has been properly created in the database"
    );
    assert_eq!(
        initial_feedbacks.len() + 1,
        new_feedbacks.len(),
        "One row should have been added to table"
    );
    assert_eq!(
        new_other_feedbacks, initial_feedbacks,
        "Other rows should NOT have been changed"
    );

    Ok(())
}

#[sqlx::test(fixtures("Feedback"))]
async fn valid_update_on_existing_feedback_should_update_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_feedbacks = services::feedback::get_all(db.clone()).await.unwrap();

    let initial_first_feedback = initial_feedbacks.first().unwrap();
    let initial_other_feedbacks = initial_feedbacks
        .iter()
        .filter(|r| r.id != initial_first_feedback.id)
        .collect::<Vec<&services::feedback::FeedbackDB>>();

    let first_feedback_update = routes::feedback::FeedbackWeb {
        id: Some(initial_first_feedback.id),
        title: Some("Updated Feedback 1".to_string()),
        text: None,
        meta: None,
        received: None,
        important: Some(true),
        archived: None,
    };

    // What's tested

    // Check output validity
    let update_query_result = services::feedback::update(db.clone(), first_feedback_update).await;
    assert!(
        update_query_result.is_ok(),
        "Update should not return an error"
    );
    assert_eq!(
        update_query_result.unwrap(),
        initial_first_feedback.id,
        "Update should return the id of the updated row"
    );

    // Check updates of feedback table
    let updated_feedbacks = services::feedback::get_all(db.clone()).await.unwrap();
    let updated_first_feedback = updated_feedbacks
        .iter()
        .cloned()
        .filter(|r| r.id == initial_first_feedback.id)
        .next()
        .unwrap();
    let updated_other_feedbacks = updated_feedbacks
        .iter()
        .filter(|r| r.id != initial_first_feedback.id)
        .collect::<Vec<&services::feedback::FeedbackDB>>();

    assert_eq!(
        updated_first_feedback,
        services::feedback::FeedbackDB {
            id: initial_first_feedback.id,
            title: "Updated Feedback 1".to_string(),
            text: "This is some great feedback".to_string(),
            meta: "{}".to_string(),
            received: DateTime::parse_from_str("2023-06-25 21:00:00+02", "%F %T%#z")
                .unwrap()
                .into(),
            important: true,
            archived: false
        },
        "The updated sure the feedback has been properly updated in the database"
    );
    assert_eq!(
        initial_other_feedbacks, updated_other_feedbacks,
        "Update should not affect other rows"
    );

    Ok(())
}

#[sqlx::test(fixtures("Feedback"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_feedbacks = services::feedback::get_all(db.clone()).await.unwrap();

    let initial_first_feedback = initial_feedbacks.first().unwrap();
    let removed_id = initial_first_feedback.id;

    // What's tested
    let remove_query_result = services::feedback::delete(db.clone(), removed_id.clone()).await;
    assert!(remove_query_result.is_ok());
    assert_eq!(remove_query_result.unwrap(), 1, "Should affect one row");

    let removed_feedback = services::feedback::get_by_id(db.clone(), removed_id.clone()).await;
    assert!(
        removed_feedback.is_err(),
        "Database query should fail for removed id"
    );

    //Check that feedback has been removed
    let remaining_feedback_rows = sqlx::query!("SELECT id FROM feedback")
        .fetch_all(&db)
        .await?;
    assert!(
        remaining_feedback_rows
            .iter()
            .all(|r| r.id != removed_id.clone()),
        "Should not return removed id when querying remaining rows"
    );
    assert_eq!(
        remaining_feedback_rows.len() + 1,
        initial_feedbacks.len(),
        "Remaining rows +1 should be equal to initial number of rows"
    );

    Ok(())
}
