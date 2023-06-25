use sqlx::{Pool, Postgres, Error};

use crate::{routes, services};

#[sqlx::test(fixtures("Prepages"))]
async fn valid_update_on_existing_prepage_should_update_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {

    // Setup
    let initial_db_prepage = sqlx::query!("SELECT * from prepages where name = $1", "First prepage")
        .fetch_one(&db).await?;

    let updated_prepage = routes::prepage::PrepageWeb {
        id: Some(initial_db_prepage.id),
        name: Some("Updated First".to_string()),
        image: Some("new_1.png".to_string()),
        active: None,
        mobile: Some(false),
        side: None,
        page: Some(3)
    };


    // What's tested
    let updated_id = services::prepage::update(db.clone(), updated_prepage).await.unwrap();


    // Checking it's correct
    let expected_prepage = services::prepage::PrepageDB {
        id: initial_db_prepage.id,
        name: "Updated First".to_string(),
        image: "new_1.png".to_string(),
        active: true,
        mobile: false,
        side: "Left".to_string(),
        page: 3
    };

    let updated_db_prepage = sqlx::query_as!(services::prepage::PrepageDB, "SELECT * FROM prepages where id = $1", updated_id)
        .fetch_one(&db).await?;

    assert_eq!(expected_prepage, updated_db_prepage, "Making sure the prepage has been properly updated in the database");

    Ok(())
}

#[sqlx::test(fixtures("Prepages"))]
async fn creating_a_valid_prepage_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let new_prepage = routes::prepage::PrepageWeb {
        id: None,
        name: Some("New page".to_string()),
        image: Some("new.png".to_string()),
        active: Some(true),
        mobile: Some(false),
        side: Some("Left".to_string()),
        page: Some(3)

    };

    // What's tested
    let created_id = services::prepage::create(db.clone(), new_prepage.clone()).await.unwrap();

    let created_db_prepage = sqlx::query_as!(services::prepage::PrepageDB,"SELECT * FROM prepages where id = $1", created_id)
        .fetch_one(&db).await?;

    // Checking it's correct
    let expected_prepage = services::prepage::PrepageDB {
        id: created_id,
        name: "New page".to_string(),
        image: "new.png".to_string(),
        active: true,
        mobile: false,
        side: "Left".to_string(),
        page: 3
    };

    assert_eq!(expected_prepage, created_db_prepage, "Making sure the prepage has been properly created in the database");

    Ok(())
}

#[sqlx::test(fixtures("Prepages"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_prepage = sqlx::query!("SELECT * from prepages where name = $1", "First prepage" )
        .fetch_one(&db).await?;

    // What's tested
    let affected_rows = services::prepage::delete(db.clone(), initial_db_prepage.id).await;
    assert!(affected_rows.is_ok());
    assert_eq!(affected_rows.unwrap(), 1);

    let remaining_db_prepage = sqlx::query_as!(services::prepage::PrepageDB,"SELECT * FROM prepages where id = $1", initial_db_prepage.id)
        .fetch_one(&db).await;
    assert!(remaining_db_prepage.is_err());

    // Check that the other 2 rows of the fixture are left
    let remaing_rows = services::prepage::get_all(db.clone()).await;
    assert!(remaing_rows.is_ok());
    assert_eq!(remaing_rows.unwrap().len(), 2);


    Ok(())
}

#[sqlx::test(fixtures("Prepages"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error>{
    //Setup
    let initial_db_prepage = sqlx::query_as!(services::prepage::PrepageDB,"SELECT * from prepages where name = $1", "First prepage" )
        .fetch_one(&db).await?;

    // What's tested
    let result = services::prepage::get_by_id(db.clone(),initial_db_prepage.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_prepage, result.unwrap());
    Ok(())
}


#[sqlx::test()]
async fn get_by_id_when_no_matching_prepage_should_fail(db: Pool<Postgres>) -> Result<(), Error>{
    // What's tested
    let result = services::prepage::get_by_id(db.clone(),7).await;
    assert!(result.is_err());

    Ok(())
}
