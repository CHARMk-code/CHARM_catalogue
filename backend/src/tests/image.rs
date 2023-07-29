use sqlx::{Error, Pool, Postgres};

use crate::{
    routes, services,
    tests::{
        create_dummy_file, create_test_folders, remove_test_folders, TEST_STORAGE, TEST_UPLOAD,
    },
};
use std::path::{Path, PathBuf};

use uuid::{uuid, Uuid};

#[sqlx::test(fixtures("Files"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_file = services::image::FileDB {
        id: uuid!("00000000-0000-0000-0000-000000000001"),
        name: "File1".to_string(),
        namespace: "images".to_string(),
        image: true,
    };

    // What's tested
    let result = services::image::get_by_id(&db, &initial_db_file.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_file, result.unwrap());
    Ok(())
}

#[sqlx::test()]
async fn get_by_id_when_no_matching_image_should_fail(db: Pool<Postgres>) -> Result<(), Error> {
    // Setup
    let invalid_id = Uuid::new_v4();

    // What's tested
    let queried_layout = services::image::get_by_id(&db, &invalid_id).await;
    assert!(
        queried_layout.is_err(),
        "Should fail when querying for nonexisting id"
    );

    Ok(())
}

#[sqlx::test(fixtures("Files"))]
async fn creating_a_valid_image_file_should_create_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    //Setup
    let initial_files = services::image::get_all(&db).await.unwrap();

    let test_path = create_test_folders()?;

    let new_images: Vec<PathBuf> = vec!["New file".into()];
    for image in &new_images {
        // create the dummy file on the disk
        create_dummy_file(&test_path, TEST_UPLOAD, image.as_os_str().to_str().unwrap())?;
    }

    // What's tested
    let created_query_result = services::image::create(
        &db,
        "images",
        new_images,
        &test_path.join(TEST_UPLOAD),
        &test_path.join(TEST_STORAGE),
    )
    .await;

    assert!(
        created_query_result.is_ok(),
        "Should not fail on creation of new row:\n{:?}",
        created_query_result
    );

    let new_files = services::image::get_all(&db).await.unwrap();
    let new_created_files: &services::image::FileDB = new_files
        .iter()
        .filter(|r| &vec![r.id] == created_query_result.as_ref().unwrap())
        .collect::<Vec<&services::image::FileDB>>()
        .first()
        .unwrap();
    let new_other_files: Vec<services::image::FileDB> = new_files
        .iter()
        .cloned()
        .filter(|r| &vec![r.id] != created_query_result.as_ref().unwrap())
        .collect();

    let expected_files = services::image::FileDB {
        id: created_query_result.unwrap().iter().next().unwrap().clone(),
        name: "New file".to_string(),
        namespace: "images".to_string(),
        image: true,
    };

    assert_eq!(
        &expected_files, new_created_files,
        "Making sure the layout has been properly created in the database"
    );
    assert_eq!(
        initial_files.len() + 1,
        new_files.len(),
        "One row should have been added to table"
    );
    assert_eq!(
        new_other_files, initial_files,
        "Other rows should NOT have been changed"
    );

    let expected_file_path = test_path.join(TEST_STORAGE).join(expected_files.id.to_string());
    assert!(
        Path::new(&expected_file_path).is_file(),
        "The created file ({:?}) should exist in the storage folder",
        expected_file_path
    );

    remove_test_folders(&test_path)?;

    Ok(())
}

#[sqlx::test(fixtures("Files"))]
async fn valid_update_on_existing_image_file_should_update_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_images = services::image::get_all(&db).await.unwrap();

    let initial_first_image = initial_images.first().unwrap();
    let initial_other_images = initial_images
        .iter()
        .filter(|r| r.id != initial_first_image.id)
        .collect::<Vec<&services::image::FileDB>>();

    let first_image_update = routes::image::FileWeb {
        id: Some(initial_first_image.id),
        name: Some("Updated file name".to_string()),
        namespace: None,
        image: None,
    };

    // What's tested

    // Check output validity
    let update_query_result = services::image::update(&db, &first_image_update).await;
    assert!(
        update_query_result.is_ok(),
        "Update should not return an error"
    );
    assert_eq!(
        update_query_result.unwrap(),
        initial_first_image.id,
        "Update should return the id of the updated row"
    );

    // Check updates of image table
    let updated_images = services::image::get_all(&db).await.unwrap();
    let updated_first_image = updated_images
        .iter()
        .cloned()
        .filter(|r| r.id == initial_first_image.id)
        .next()
        .unwrap();
    let updated_other_images = updated_images
        .iter()
        .filter(|r| r.id != initial_first_image.id)
        .collect::<Vec<&services::image::FileDB>>();

    assert_eq!(
        updated_first_image,
        services::image::FileDB {
            id: initial_first_image.id,
            name: "Updated file name".to_string(),
            namespace: "images".to_string(),
            image: true,
        },
        "The updated sure the image has been properly updated in the database"
    );
    assert_eq!(
        initial_other_images, updated_other_images,
        "Update should not affect other rows"
    );

    Ok(())
}

#[sqlx::test(fixtures("Files"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_images = services::image::get_all(&db).await.unwrap();

    let test_path = create_test_folders()?;
    //Create dummy files on disk for file representations mocked in DB
    for image in &initial_images {
        create_dummy_file(&test_path, TEST_STORAGE, image.id.to_string().as_str())?;
    }
    let storage_path = test_path.join(TEST_STORAGE);

    let initial_first_image = initial_images.first().unwrap();
    let removed_id = initial_first_image.id;

    // What's tested
    let remove_query_result = services::image::delete(&db, &removed_id, &storage_path).await;
    assert!(
        remove_query_result.is_ok(),
        "Removing row with id should not fail: {:?}",
        remove_query_result
    );
    assert_eq!(remove_query_result.unwrap(), 1, "Should affect one row");

    let removed_image = services::image::get_by_id(&db, &removed_id).await;
    assert!(
        removed_image.is_err(),
        "Database query should fail for removed id"
    );

    //Check that image has been removed
    let remaining_image_rows = sqlx::query!("SELECT id FROM files").fetch_all(&db).await?;
    assert!(
        remaining_image_rows
            .iter()
            .all(|r| r.id != removed_id.clone()),
        "Should not return removed id when querying remaining rows"
    );
    assert_eq!(
        remaining_image_rows.len() + 1,
        initial_images.len(),
        "Remaining rows +1 should be equal to initial number of rows"
    );

    remove_test_folders(&test_path)?;

    Ok(())
}
