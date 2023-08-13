use sqlx::{Error, Pool, Postgres};

use crate::models::shortcut::ShortcutDB;
use crate::models::shortcut::ShortcutWeb;
use crate::services;

#[sqlx::test(fixtures("Shortcuts"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_shortcut = ShortcutDB {
        id: 1,
        name: "Favorites".to_string(),
        description: "Your favorite companies".to_string(),
        link: "/search?favorites=true".to_string(),
        icon: "mdi-star".to_string(),
    };

    // What's tested
    let result = services::shortcut::get_by_id(&db, initial_db_shortcut.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_shortcut, result.unwrap());
    Ok(())
}

#[sqlx::test()]
async fn get_by_id_when_no_matching_shortcut_should_fail(db: Pool<Postgres>) -> Result<(), Error> {
    // Setup
    let initial_row_amount = sqlx::query!("SELECT count(*) FROM shortcuts")
        .fetch_all(&db)
        .await?;

    let invalid_id = i32::try_from(initial_row_amount.first().unwrap().count.unwrap()).unwrap() + 1;

    // What's tested
    let queried_shortcut = services::shortcut::get_by_id(&db, invalid_id).await;
    assert!(
        queried_shortcut.is_err(),
        "Should fail when querying for nonexisting id"
    );

    Ok(())
}

#[sqlx::test(fixtures("Shortcuts"))]
async fn creating_a_valid_shortcut_should_create_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    //Setup
    let initial_shortcuts = services::shortcut::get_all(&db).await.unwrap();

    let new_shortcut = ShortcutWeb {
        id: None,
        name: Some("New shortcut".to_string()),
        description: Some("New shortcut description".to_string()),
        link: Some("link/to/somewhere".to_string()),
        icon: Some("Updated_icon".to_string()),
    };

    // What's tested
    let created_query_result = services::shortcut::create(&db, &new_shortcut).await;
    assert!(
        created_query_result.is_ok(),
        "Should not fail on creation of new row"
    );

    let new_shortcuts = services::shortcut::get_all(&db).await.unwrap();
    let new_created_shortcut: &ShortcutDB = new_shortcuts
        .iter()
        .filter(|r| &r.id == created_query_result.as_ref().unwrap())
        .collect::<Vec<&ShortcutDB>>()
        .first()
        .unwrap();
    let new_other_shortcuts: Vec<ShortcutDB> = new_shortcuts
        .iter()
        .cloned()
        .filter(|r| &r.id != created_query_result.as_ref().unwrap())
        .collect();

    let expected_shortcut = ShortcutDB {
        id: created_query_result.unwrap(),
        name: "New shortcut".to_string(),
        description: "New shortcut description".to_string(),
        link: "link/to/somewhere".to_string(),
        icon: "Updated_icon".to_string(),
    };

    assert_eq!(
        &expected_shortcut, new_created_shortcut,
        "Making sure the shortcut has been properly created in the database"
    );
    assert_eq!(
        initial_shortcuts.len() + 1,
        new_shortcuts.len(),
        "One row should have been added to table"
    );
    assert_eq!(
        new_other_shortcuts, initial_shortcuts,
        "Other rows should NOT have been changed"
    );

    Ok(())
}

#[sqlx::test(fixtures("Shortcuts"))]
async fn valid_update_on_existing_shortcut_should_update_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_shortcuts = services::shortcut::get_all(&db).await.unwrap();

    let initial_first_shortcut = initial_shortcuts.first().unwrap();
    let initial_other_shortcuts = initial_shortcuts
        .iter()
        .filter(|r| r.id != initial_first_shortcut.id)
        .collect::<Vec<&ShortcutDB>>();

    let first_shortcut_update = ShortcutWeb {
        id: Some(initial_first_shortcut.id),
        name: Some("Updated_Favorites".to_string()),
        description: Some("Updated_Description".to_string()),
        link: Some("Updated_Link".to_string()),
        icon: Some("Updated_icon".to_string()),
    };

    // What's tested

    // Check output validity
    let update_query_result = services::shortcut::update(&db, &first_shortcut_update).await;
    assert!(
        update_query_result.is_ok(),
        "Update should not return an error"
    );
    assert_eq!(
        update_query_result.unwrap(),
        initial_first_shortcut.id,
        "Update should return the id of the updated row"
    );

    // Check updates of shortcut table
    let updated_shortcuts = services::shortcut::get_all(&db).await.unwrap();
    let updated_first_shortcut = updated_shortcuts
        .iter()
        .cloned()
        .filter(|r| r.id == initial_first_shortcut.id)
        .next()
        .unwrap();
    let updated_other_shortcuts = updated_shortcuts
        .iter()
        .filter(|r| r.id != initial_first_shortcut.id)
        .collect::<Vec<&ShortcutDB>>();

    assert_eq!(
        updated_first_shortcut,
        ShortcutDB {
            id: initial_first_shortcut.id,
            name: "Updated_Favorites".to_string(),
            description: "Updated_Description".to_string(),
            link: "Updated_Link".to_string(),
            icon: "Updated_icon".to_string()
        },
        "The updated sure the shortcut has been properly updated in the database"
    );
    assert_eq!(
        initial_other_shortcuts, updated_other_shortcuts,
        "Update should not affect other rows"
    );

    Ok(())
}

#[sqlx::test(fixtures("Shortcuts"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_shortcuts = services::shortcut::get_all(&db).await.unwrap();

    let initial_first_shortcut = initial_shortcuts.first().unwrap();
    let removed_id = initial_first_shortcut.id;

    // What's tested
    let remove_query_result = services::shortcut::delete(&db, removed_id.clone()).await;
    assert!(remove_query_result.is_ok());
    assert_eq!(remove_query_result.unwrap(), 1, "Should affect one row");

    let removed_shortcut = services::shortcut::get_by_id(&db, removed_id.clone()).await;
    assert!(
        removed_shortcut.is_err(),
        "Database query should fail for removed id"
    );

    //Check that shortcut has been removed
    let remaining_shortcut_rows = sqlx::query!("SELECT id FROM shortcuts")
        .fetch_all(&db)
        .await?;
    assert!(
        remaining_shortcut_rows
            .iter()
            .all(|r| r.id != removed_id.clone()),
        "Should not return removed id when querying remaining rows"
    );
    assert_eq!(
        remaining_shortcut_rows.len() + 1,
        initial_shortcuts.len(),
        "Remaining rows +1 should be equal to initial number of rows"
    );

    Ok(())
}
