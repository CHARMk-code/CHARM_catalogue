use sqlx::{Error, Pool, Postgres};

use crate::{routes, services};

#[sqlx::test(fixtures("Maps"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_map = services::map::MapDB {
        id: 1,
        name: "VOLVO 1".to_string(),
        image: "volvo1.svg".to_string(),
        reference: 1,
    };

    // What's tested
    let result = services::map::get_by_id(&db, initial_db_map.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_map, result.unwrap());
    Ok(())
}

#[sqlx::test()]
async fn get_by_id_when_no_matching_map_should_fail(db: Pool<Postgres>) -> Result<(), Error> {
    // Setup
    let initial_row_amount = sqlx::query!("SELECT count(*) FROM maps")
        .fetch_all(&db)
        .await?;

    let invalid_id = i32::try_from(initial_row_amount.first().unwrap().count.unwrap()).unwrap() + 1;

    // What's tested
    let queried_map = services::map::get_by_id(&db, invalid_id).await;
    assert!(
        queried_map.is_err(),
        "Should fail when querying for nonexisting id"
    );

    Ok(())
}

#[sqlx::test(fixtures("Maps"))]
async fn creating_a_valid_map_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_maps = services::map::get_all(&db).await.unwrap();

    let new_map = routes::map::MapWeb {
        id: None,
        name: Some("Scania".to_string()),
        image: Some("scania.svg".to_string()),
        reference: Some(2), // This should be nullable
    };

    // What's tested
    let created_query_result = services::map::create(&db, &new_map).await;
    assert!(
        created_query_result.is_ok(),
        "Should not fail on creation of new row"
    );

    let new_maps = services::map::get_all(&db).await.unwrap();
    let new_created_map: &services::map::MapDB = new_maps
        .iter()
        .filter(|r| &r.id == created_query_result.as_ref().unwrap())
        .collect::<Vec<&services::map::MapDB>>()
        .first()
        .unwrap();
    let new_other_maps: Vec<services::map::MapDB> = new_maps
        .iter()
        .cloned()
        .filter(|r| &r.id != created_query_result.as_ref().unwrap())
        .collect();

    let expected_map = services::map::MapDB {
        id: created_query_result.unwrap(),
        name: "Scania".to_string(),
        image: "scania.svg".to_string(),
        reference: 2, // This should be nullable
    };

    assert_eq!(
        &expected_map, new_created_map,
        "Making sure the map has been properly created in the database"
    );
    assert_eq!(
        initial_maps.len() + 1,
        new_maps.len(),
        "One row should have been added to table"
    );
    assert_eq!(
        new_other_maps, initial_maps,
        "Other rows should NOT have been changed"
    );

    Ok(())
}

#[sqlx::test(fixtures("Maps"))]
async fn valid_update_on_existing_map_should_update_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_maps = services::map::get_all(&db).await.unwrap();

    let initial_first_map = initial_maps.first().unwrap();
    let initial_other_maps = initial_maps
        .iter()
        .filter(|r| r.id != initial_first_map.id)
        .collect::<Vec<&services::map::MapDB>>();

    let first_map_update = routes::map::MapWeb {
        id: Some(initial_first_map.id),
        name: Some("Ljusgården".to_string()),
        image: Some("ljusgården.svg".to_string()),
        reference: None,
    };

    // What's tested

    // Check output validity
    let update_query_result = services::map::update(&db, &first_map_update).await;
    assert!(
        update_query_result.is_ok(),
        "Update should not return an error"
    );
    assert_eq!(
        update_query_result.unwrap(),
        initial_first_map.id,
        "Update should return the id of the updated row"
    );

    // Check updates of map table
    let updated_maps = services::map::get_all(&db).await.unwrap();
    let updated_first_map = updated_maps
        .iter()
        .cloned()
        .filter(|r| r.id == initial_first_map.id)
        .next()
        .unwrap();
    let updated_other_maps = updated_maps
        .iter()
        .filter(|r| r.id != initial_first_map.id)
        .collect::<Vec<&services::map::MapDB>>();

    assert_eq!(
        updated_first_map,
        services::map::MapDB {
            id: initial_first_map.id,
            name: "Ljusgården".to_string(),
            image: "ljusgården.svg".to_string(),
            reference: 1
        },
        "The updated sure the map has been properly updated in the database"
    );
    assert_eq!(
        initial_other_maps, updated_other_maps,
        "Update should not affect other rows"
    );

    Ok(())
}

#[sqlx::test(fixtures("Maps"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_maps = services::map::get_all(&db).await.unwrap();

    let initial_first_map = initial_maps.first().unwrap();
    let removed_id = initial_first_map.id;

    // What's tested
    let remove_query_result = services::map::delete(&db, removed_id).await;
    assert!(remove_query_result.is_ok());
    assert_eq!(remove_query_result.unwrap(), 1, "Should affect one row");

    let removed_map = services::map::get_by_id(&db, removed_id).await;
    assert!(
        removed_map.is_err(),
        "Database query should fail for removed id"
    );

    //Check that map has been removed
    let remaining_map_rows = sqlx::query!("SELECT id FROM maps").fetch_all(&db).await?;
    assert!(
        remaining_map_rows
            .iter()
            .all(|r| r.id != removed_id.clone()),
        "Should not return removed id when querying remaining rows"
    );
    assert_eq!(
        remaining_map_rows.len() + 1,
        initial_maps.len(),
        "Remaining rows +1 should be equal to initial number of rows"
    );

    Ok(())
}
