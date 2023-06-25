use sqlx::{Pool, Postgres, Error};

use crate::services;
use crate::routes;

#[sqlx::test(fixtures("Layouts"))]
async fn valid_update_on_existing_layout_should_update_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {

    // Setup
    let initial_db_layout = sqlx::query!("SELECT * from layouts where image = $1", "LEFT" )
        .fetch_one(&db).await?;

    let updated_layout = routes::layout::LayoutWeb {
        id: Some(initial_db_layout.id),
        image: Some("Updated LEFT".to_string()),
        active: Some(true),
        placement: Some(0)
    };


    // What's tested
    let updated_id = services::layout::update(db.clone(), updated_layout).await.unwrap();


    // Checking it's correct
    let expected_layout = services::layout::LayoutDB {
        id: initial_db_layout.id,
        image: "Updated LEFT".to_string(),
        active: true,
        placement: 0
    };

    let updated_db_layout = sqlx::query_as!(services::layout::LayoutDB,"SELECT * FROM layouts where id = $1", updated_id)
        .fetch_one(&db).await?;

    assert_eq!(expected_layout, updated_db_layout, "Making sure the layout has been properly updated in the database");

    Ok(())
}

#[sqlx::test(fixtures("Layouts"))]
async fn creating_a_valid_layout_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let new_layout = routes::layout::LayoutWeb {
        id: None,
        image: Some("New layout".to_string()),
        active: Some(false),
        placement: Some(2),
    };

    // What's tested
    let created_id = services::layout::create(db.clone(), new_layout.clone()).await.unwrap();

    let created_db_layout = sqlx::query_as!(services::layout::LayoutDB,"SELECT * FROM layouts where id = $1", created_id)
        .fetch_one(&db).await?;

    // Checking it's correct
    let expected_layout = services::layout::LayoutDB {
        id: created_id.try_into().expect("Should be a convertable number"),
        image: new_layout.image.unwrap(),
        active: new_layout.active.unwrap(),
        placement: new_layout.placement.unwrap(),
    };

    assert_eq!(expected_layout, created_db_layout, "Making sure the layout has been properly created in the database");

    Ok(())
}

#[sqlx::test(fixtures("Layouts"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_layout = sqlx::query!("SELECT * from layouts where image = $1", "LEFT" )
        .fetch_one(&db).await?;

    // What's tested
    let affected_rows = services::layout::delete(db.clone(), initial_db_layout.id).await;
    assert!(affected_rows.is_ok());
    assert_eq!(affected_rows.unwrap(), 1);
    
    let remaining_db_layout = sqlx::query_as!(services::layout::LayoutDB,"SELECT * FROM layouts where id = $1", initial_db_layout.id)
        .fetch_one(&db).await;
    assert!(remaining_db_layout.is_err());
        
    // Check that the other 2 rows of the fixture are left
    let remaing_rows = services::layout::get_all(db.clone()).await;
    assert!(remaing_rows.is_ok());
    assert_eq!(remaing_rows.unwrap().len(), 2);


    Ok(())
}

#[sqlx::test(fixtures("Layouts"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error>{
    //Setup
    let initial_db_layout = sqlx::query_as!(services::layout::LayoutDB,"SELECT * from layouts where image = $1", "LEFT" )
        .fetch_one(&db).await?;

    // What's tested
    let result = services::layout::get_by_id(db.clone(),initial_db_layout.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_layout, result.unwrap());
    Ok(())
}


#[sqlx::test()]
async fn get_by_id_when_no_matching_layout_should_fail(db: Pool<Postgres>) -> Result<(), Error>{
    // What's tested
    let result = services::layout::get_by_id(db.clone(),7).await;
    assert!(result.is_err());

    Ok(())
}
