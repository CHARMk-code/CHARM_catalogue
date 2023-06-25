use sqlx::{Pool, Postgres, Error};

use crate::{routes, services};

#[sqlx::test(fixtures("Maps"))]
async fn valid_update_on_existing_map_should_update_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {

    // Setup
    let initial_db_map = sqlx::query!("SELECT * from maps where name = $1", "VOLVO 1")
        .fetch_one(&db).await?;

    let updated_map = routes::map::MapWeb {
        id: Some(initial_db_map.id),
        name: Some("Ljusgården".to_string()),
        image: Some("ljusgården.svg".to_string()),
        reference: None,
    };


    // What's tested
    let updated_id = services::map::update(db.clone(), updated_map).await.unwrap();


    // Checking it's correct
    let expected_map = services::map::MapDB {
        id: initial_db_map.id,
        name: "Ljusgården".to_string(),
        image: "ljusgården.svg".to_string(),
        reference: 1
    };

    let updated_db_map = sqlx::query_as!(services::map::MapDB, "SELECT * FROM maps where id = $1", updated_id)
        .fetch_one(&db).await?;

    assert_eq!(expected_map, updated_db_map, "Making sure the map has been properly updated in the database");

    Ok(())
}

#[sqlx::test(fixtures("Maps"))]
async fn creating_a_valid_map_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let new_map = routes::map::MapWeb {
        id: None,
        name: Some("Scania".to_string()),
        image: Some("scania.svg".to_string()),
        reference: Some(2) // This should be nullable

    };

    // What's tested
    let created_id = services::map::create(db.clone(), new_map.clone()).await.unwrap();

    let created_db_map = sqlx::query_as!(services::map::MapDB,"SELECT * FROM maps where id = $1", created_id)
        .fetch_one(&db).await?;

    // Checking it's correct
    let expected_map = services::map::MapDB {
        id: created_id,
        name: "Scania".to_string(),
        image: "scania.svg".to_string(),
        reference: 2
    };

    assert_eq!(expected_map, created_db_map, "Making sure the map has been properly created in the database");

    Ok(())
}

#[sqlx::test(fixtures("Maps"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_map = sqlx::query!("SELECT * from maps where name = $1", "VOLVO 1" )
        .fetch_one(&db).await?;

    // What's tested
    let affected_rows = services::map::delete(db.clone(), initial_db_map.id).await;
    assert!(affected_rows.is_ok());
    assert_eq!(affected_rows.unwrap(), 1);

    let remaining_db_map = sqlx::query_as!(services::map::MapDB,"SELECT * FROM maps where id = $1", initial_db_map.id)
        .fetch_one(&db).await;
    assert!(remaining_db_map.is_err());

    // Check that the other 2 rows of the fixture are left
    let remaing_rows = services::map::get_all(db.clone()).await;
    assert!(remaing_rows.is_ok());
    assert_eq!(remaing_rows.unwrap().len(), 2);


    Ok(())
}

#[sqlx::test(fixtures("Maps"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error>{
    //Setup
    let initial_db_map = sqlx::query_as!(services::map::MapDB,"SELECT * from maps where name = $1", "VOLVO 1" )
        .fetch_one(&db).await?;

    // What's tested
    let result = services::map::get_by_id(db.clone(),initial_db_map.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_map, result.unwrap());
    Ok(())
}


#[sqlx::test()]
async fn get_by_id_when_no_matching_map_should_fail(db: Pool<Postgres>) -> Result<(), Error>{
    // What's tested
    let result = services::map::get_by_id(db.clone(),7).await;
    assert!(result.is_err());

    Ok(())
}
