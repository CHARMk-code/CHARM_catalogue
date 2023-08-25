use sqlx::{Error, Pool, Postgres};

use crate::{
    models::blob::{JSONBlobDB, JSONBlobWeb},
    services,
};

#[sqlx::test(fixtures("Blobs"))]
async fn get_by_name_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_blob = JSONBlobDB {
        id: 1,
        name: "blob1".to_string(),
        blob: serde_json::from_str::<serde_json::Value>("{\"key1\": 1}").unwrap(),
    };

    // What's tested
    let result = services::settings::blob::get_by_name(&db, &initial_db_blob.name).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_blob, result.unwrap());
    Ok(())
}

#[sqlx::test(fixtures("Blobs"))]
async fn get_by_name_when_no_matching_blob_should_fail(db: Pool<Postgres>) -> Result<(), Error> {
    // Setup
    let invalid_name = "name not in the database";

    // What's tested
    let queried_blob = services::settings::blob::get_by_name(&db, invalid_name).await;
    assert!(
        queried_blob.is_err(),
        "Should fail when querying for nonexisting id"
    );

    Ok(())
}

#[sqlx::test(fixtures("Blobs"))]
async fn creating_a_valid_blob_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_blobs = sqlx::query_as!(JSONBlobDB, "SELECT * FROM blobs")
        .fetch_all(&db)
        .await?;

    let new_blob = JSONBlobWeb {
        name: Some("NEW BLOB".to_string()),
        blob: Some(serde_json::from_str::<serde_json::Value>("{\"new\": 12}").unwrap()),
    };

    // What's tested
    let created_query_result = services::settings::blob::create(&db, &new_blob).await;
    assert!(
        created_query_result.is_ok(),
        "Should not fail on creation of new row"
    );

    let new_blobs = sqlx::query_as!(JSONBlobDB, "SELECT * FROM blobs")
        .fetch_all(&db)
        .await?;

    let new_created_blob: &JSONBlobDB = new_blobs
        .iter()
        .filter(|r| &r.id == created_query_result.as_ref().unwrap())
        .collect::<Vec<&JSONBlobDB>>()
        .first()
        .unwrap();
    let new_other_blobs: Vec<JSONBlobDB> = new_blobs
        .iter()
        .cloned()
        .filter(|r| &r.id != created_query_result.as_ref().unwrap())
        .collect();

    let expected_blob = JSONBlobDB {
        id: created_query_result.unwrap(),
        name: "NEW BLOB".to_string(),
        blob: serde_json::from_str::<serde_json::Value>("{\"new\": 12}").unwrap(),
    };

    assert_eq!(
        &expected_blob, new_created_blob,
        "Making sure the blob has been properly created in the database"
    );
    assert_eq!(
        initial_blobs.len() + 1,
        new_blobs.len(),
        "One row should have been added to table"
    );
    assert_eq!(
        new_other_blobs, initial_blobs,
        "Other rows should NOT have been changed"
    );

    Ok(())
}

#[sqlx::test(fixtures("Blobs"))]
async fn valid_update_on_existing_blob_should_update_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_blobs = sqlx::query_as!(JSONBlobDB, "SELECT * FROM blobs")
        .fetch_all(&db)
        .await?;

    let initial_first_blob = initial_blobs.first().unwrap();
    let initial_other_blobs = initial_blobs
        .iter()
        .filter(|r| r.id != initial_first_blob.id)
        .collect::<Vec<&JSONBlobDB>>();

    let first_blob_update = JSONBlobWeb {
        name: Some(initial_first_blob.name.clone()),
        blob: Some(serde_json::from_str::<serde_json::Value>("{\"changed\": 123}").unwrap()),
    };

    // What's tested

    // Check output validity
    let update_query_result = services::settings::blob::update(&db, &first_blob_update).await;
    assert!(
        update_query_result.is_ok(),
        "Update should not return an error"
    );
    assert_eq!(
        update_query_result.unwrap(),
        initial_first_blob.id,
        "Update should return the id of the updated row"
    );

    // Check updates of blob table
    let updated_blobs = sqlx::query_as!(JSONBlobDB, "SELECT * FROM blobs")
        .fetch_all(&db)
        .await?;
    let updated_first_blob = updated_blobs
        .iter()
        .cloned().find(|r| r.id == initial_first_blob.id)
        .unwrap();
    let updated_other_blobs = updated_blobs
        .iter()
        .filter(|r| r.id != initial_first_blob.id)
        .collect::<Vec<&JSONBlobDB>>();

    assert_eq!(
        updated_first_blob,
        JSONBlobDB {
            id: initial_first_blob.id,
            name: initial_first_blob.name.clone(),
            blob: serde_json::from_str::<serde_json::Value>("{\"changed\": 123}").unwrap(),
        },
        "The updated sure the blob has been properly updated in the database"
    );
    assert_eq!(
        initial_other_blobs, updated_other_blobs,
        "Update should not affect other rows"
    );

    Ok(())
}

#[sqlx::test(fixtures("Blobs"))]
async fn delete_on_existing_name_should_remove_correct_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_blobs = sqlx::query_as!(JSONBlobDB, "SELECT * FROM blobs")
        .fetch_all(&db)
        .await?;

    let initial_first_blob = initial_blobs.first().unwrap();
    let removed_id = initial_first_blob.id;

    // What's tested
    let remove_query_result = services::settings::blob::delete_by_name(&db, &initial_first_blob.name).await;
    assert!(remove_query_result.is_ok());
    assert_eq!(remove_query_result.unwrap(), 1, "Should affect one row");

    let removed_blob = services::settings::blob::get_by_name(&db, &initial_first_blob.name).await;
    assert!(
        removed_blob.is_err(),
        "Database query should fail for removed id"
    );

    //Check that blob has been removed
    let remaining_blob_rows = sqlx::query!("SELECT id FROM blobs").fetch_all(&db).await?;
    assert!(
        remaining_blob_rows
            .iter()
            .all(|r| r.id != removed_id),
        "Should not return removed id when querying remaining rows"
    );
    assert_eq!(
        remaining_blob_rows.len() + 1,
        initial_blobs.len(),
        "Remaining rows +1 should be equal to initial number of rows"
    );

    Ok(())
}
