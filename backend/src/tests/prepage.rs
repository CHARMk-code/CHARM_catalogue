use sqlx::{Pool, Postgres, Error};

use crate::{routes, services};

#[sqlx::test(fixtures("Prepages"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error>{
    //Setup
    let initial_db_prepage = services::prepage::PrepageDB {
        id: 1,
        name: "First prepage".to_string(),
        image: "1.png".to_string(),
        active: true,
        mobile: true,
        side: "Left".to_string(),
        page: 1,
    };

    // What's tested
    let result = services::prepage::get_by_id(db.clone(),initial_db_prepage.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_prepage, result.unwrap());
    Ok(())
}

#[sqlx::test()]
async fn get_by_id_when_no_matching_prepage_should_fail(db: Pool<Postgres>) -> Result<(), Error>{
    // Setup
    let initial_row_amount = sqlx::query!("SELECT count(*) FROM prepages").fetch_all(&db).await?;

    let invalid_id = i32::try_from(initial_row_amount.first().unwrap().count.unwrap()).unwrap() + 1;
    
    // What's tested
    let queried_prepage = services::prepage::get_by_id(db.clone(),invalid_id).await;
    assert!(queried_prepage.is_err(), "Should fail when querying for nonexisting id");

    Ok(())
}

#[sqlx::test(fixtures("Prepages"))]
async fn creating_a_valid_prepage_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_prepages = services::prepage::get_all(db.clone()).await.unwrap();

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
    let created_query_result = services::prepage::create(db.clone(), new_prepage.clone()).await;
    assert!(created_query_result.is_ok(), "Should not fail on creation of new row");
    
    let new_prepages = services::prepage::get_all(db.clone()).await.unwrap();
    let new_created_prepage: &services::prepage::PrepageDB = new_prepages.iter()
        .filter(|r| &r.id == created_query_result.as_ref().unwrap())
        .collect::<Vec<&services::prepage::PrepageDB>>().first().unwrap();
    let new_other_prepages: Vec<services::prepage::PrepageDB> = new_prepages
        .iter()
        .cloned()
        .filter(|r| &r.id != created_query_result.as_ref().unwrap())
        .collect();

    let expected_prepage = services::prepage::PrepageDB {
        id: created_query_result.unwrap(),
        name: "New page".to_string(),
        image: "new.png".to_string(),
        active: true,
        mobile: false,
        side: "Left".to_string(),
        page: 3
    };

    assert_eq!(&expected_prepage, new_created_prepage, "Making sure the prepage has been properly created in the database");
    assert_eq!(initial_prepages.len() + 1, new_prepages.len(), "One row should have been added to table");
    assert_eq!(new_other_prepages, initial_prepages,"Other rows should NOT have been changed");

    Ok(())
}

#[sqlx::test(fixtures("Prepages"))]
async fn valid_update_on_existing_prepage_should_update_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {

    // Setup
    let initial_prepages = services::prepage::get_all(db.clone()).await.unwrap();

    let initial_first_prepage = initial_prepages.first().unwrap();
    let initial_other_prepages = initial_prepages.iter().filter(|r| r.id != initial_first_prepage.id).collect::<Vec<&services::prepage::PrepageDB>>();

    let first_prepage_update = routes::prepage::PrepageWeb {
        id: Some(initial_first_prepage.id),
        name: Some("Updated First".to_string()),
        image: Some("new_1.png".to_string()),
        active: None,
        mobile: Some(false),
        side: None,
        page: Some(3)
    };


    // What's tested
    
    // Check output validity
    let update_query_result = services::prepage::update(db.clone(), first_prepage_update).await;
    assert!(update_query_result.is_ok(), "Update should not return an error");
    assert_eq!(update_query_result.unwrap(), initial_first_prepage.id, "Update should return the id of the updated row");
    
    // Check updates of prepage table
    let updated_prepages = services::prepage::get_all(db.clone()).await.unwrap();
    let updated_first_prepage = updated_prepages.iter().cloned().filter(|r| r.id == initial_first_prepage.id).next().unwrap();
    let updated_other_prepages = updated_prepages.iter().filter(|r| r.id != initial_first_prepage.id).collect::<Vec<&services::prepage::PrepageDB>>();

    assert_eq!(
        updated_first_prepage, 
        services::prepage::PrepageDB {
        id: initial_first_prepage.id,
        name: "Updated First".to_string(),
        image: "new_1.png".to_string(),
        active: true,
        mobile: false,
        side: "Left".to_string(),
        page: 3
        },
    "The updated sure the prepage has been properly updated in the database");
    assert_eq!(initial_other_prepages, updated_other_prepages, "Update should not affect other rows");

    Ok(())
}


#[sqlx::test(fixtures("Prepages"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    // Setup
    let initial_prepages = services::prepage::get_all(db.clone()).await.unwrap();
   
    let initial_first_prepage = initial_prepages.first().unwrap();
    let removed_id = initial_first_prepage.id;

   
    // What's tested
    let remove_query_result = services::prepage::delete(db.clone(), removed_id.clone()).await;
    assert!(remove_query_result.is_ok());
    assert_eq!(remove_query_result.unwrap(), 1, "Should affect one row");
   
    
    let removed_prepage = services::prepage::get_by_id(db.clone(), removed_id.clone()).await;
    assert!(removed_prepage.is_err(), "Database query should fail for removed id");
    
    //Check that prepage has been removed
    let remaining_prepage_rows = sqlx::query!("SELECT id FROM prepages").fetch_all(&db).await?;
    assert!(remaining_prepage_rows.iter().all(|r| r.id != removed_id.clone()), "Should not return removed id when querying remaining rows");
    assert_eq!(remaining_prepage_rows.len()+1, initial_prepages.len(), "Remaining rows +1 should be equal to initial number of rows" );
    
    Ok(())
}
