use sqlx::{Pool, Postgres, Error};

use crate::services;
use crate::routes;

#[sqlx::test(fixtures("Layouts"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error>{
    //Setup
    let initial_db_layout = services::layout::LayoutDB {
        id: 1,
        image: "LEFT".to_string(),
        active: true,
        placement: 0,
    };

    // What's tested
    let result = services::layout::get_by_id(db.clone(),initial_db_layout.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_layout, result.unwrap());
    Ok(())
}

#[sqlx::test()]
async fn get_by_id_when_no_matching_layout_should_fail(db: Pool<Postgres>) -> Result<(), Error>{
    // Setup
    let initial_row_amount = sqlx::query!("SELECT count(*) FROM layouts").fetch_all(&db).await?;

    let invalid_id = i32::try_from(initial_row_amount.first().unwrap().count.unwrap()).unwrap() + 1;
    
    // What's tested
    let queried_layout = services::layout::get_by_id(db.clone(),invalid_id).await;
    assert!(queried_layout.is_err(), "Should fail when querying for nonexisting id");

    Ok(())
}

#[sqlx::test(fixtures("Layouts"))]
async fn creating_a_valid_layout_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_layouts = services::layout::get_all(db.clone()).await.unwrap();

    let new_layout = routes::layout::LayoutWeb {
        id: None,
        image: Some("New layout".to_string()),
        active: Some(false),
        placement: Some(2),

    };

    // What's tested
    let created_query_result = services::layout::create(db.clone(), new_layout.clone()).await;
    assert!(created_query_result.is_ok(), "Should not fail on creation of new row");
    
    let new_layouts = services::layout::get_all(db.clone()).await.unwrap();
    let new_created_layout: &services::layout::LayoutDB = new_layouts.iter()
        .filter(|r| &r.id == created_query_result.as_ref().unwrap())
        .collect::<Vec<&services::layout::LayoutDB>>().first().unwrap();
    let new_other_layouts: Vec<services::layout::LayoutDB> = new_layouts
        .iter()
        .cloned()
        .filter(|r| &r.id != created_query_result.as_ref().unwrap())
        .collect();

    let expected_layout = services::layout::LayoutDB {
        id: created_query_result.unwrap(),
        image: "New layout".to_string(),
        active: false,
        placement: 2,
    };

    assert_eq!(&expected_layout, new_created_layout, "Making sure the layout has been properly created in the database");
    assert_eq!(initial_layouts.len() + 1, new_layouts.len(), "One row should have been added to table");
    assert_eq!(new_other_layouts, initial_layouts,"Other rows should NOT have been changed");

    Ok(())
}

#[sqlx::test(fixtures("Layouts"))]
async fn valid_update_on_existing_layout_should_update_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {

    // Setup
    let initial_layouts = services::layout::get_all(db.clone()).await.unwrap();

    let initial_first_layout = initial_layouts.first().unwrap();
    let initial_other_layouts = initial_layouts.iter().filter(|r| r.id != initial_first_layout.id).collect::<Vec<&services::layout::LayoutDB>>();

    let first_layout_update = routes::layout::LayoutWeb {
        id: Some(initial_first_layout.id),
        image: Some("Updated LEFT".to_string()),
        active: Some(true),
        placement: Some(0)
    };


    // What's tested
    
    // Check output validity
    let update_query_result = services::layout::update(db.clone(), first_layout_update).await;
    assert!(update_query_result.is_ok(), "Update should not return an error");
    assert_eq!(update_query_result.unwrap(), initial_first_layout.id, "Update should return the id of the updated row");
    
    // Check updates of layout table
    let updated_layouts = services::layout::get_all(db.clone()).await.unwrap();
    let updated_first_layout = updated_layouts.iter().cloned().filter(|r| r.id == initial_first_layout.id).next().unwrap();
    let updated_other_layouts = updated_layouts.iter().filter(|r| r.id != initial_first_layout.id).collect::<Vec<&services::layout::LayoutDB>>();

    assert_eq!(
        updated_first_layout, 
        services::layout::LayoutDB {
        id: initial_first_layout.id,
        image: "Updated LEFT".to_string(),
        active: true,
        placement: 0, 
        },
    "The updated sure the layout has been properly updated in the database");
    assert_eq!(initial_other_layouts, updated_other_layouts, "Update should not affect other rows");

    Ok(())
}


#[sqlx::test(fixtures("Layouts"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    // Setup
    let initial_layouts = services::layout::get_all(db.clone()).await.unwrap();
   
    let initial_first_layout = initial_layouts.first().unwrap();
    let removed_id = initial_first_layout.id;

   
    // What's tested
    let remove_query_result = services::layout::delete(db.clone(), removed_id.clone()).await;
    assert!(remove_query_result.is_ok());
    assert_eq!(remove_query_result.unwrap(), 1, "Should affect one row");
   
    
    let removed_layout = services::layout::get_by_id(db.clone(), removed_id.clone()).await;
    assert!(removed_layout.is_err(), "Database query should fail for removed id");
    
    //Check that layout has been removed
    let remaining_layout_rows = sqlx::query!("SELECT id FROM layouts").fetch_all(&db).await?;
    assert!(remaining_layout_rows.iter().all(|r| r.id != removed_id.clone()), "Should not return removed id when querying remaining rows");
    assert_eq!(remaining_layout_rows.len()+1, initial_layouts.len(), "Remaining rows +1 should be equal to initial number of rows" );
    
    Ok(())
}
