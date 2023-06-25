use sqlx::{Pool, Postgres, Error};

use crate::services;
use crate::routes;

#[sqlx::test(fixtures("Shortcuts"))]
async fn valid_update_on_existing_shortcut_should_update_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {

    // Setup
    let initial_db_shortcut = sqlx::query!("SELECT * from shortcuts where name = $1", "Favorites" )
        .fetch_one(&db).await?;

    let updated_shortcut = routes::shortcut::ShortcutWeb {
        id: Some(initial_db_shortcut.id),
        name: Some("Updated_Favorites".to_string()),
        description: Some("Updated_Description".to_string()),
        link: Some("Updated_Link".to_string()),
        icon: Some("Updated_icon".to_string())
    };


    // What's tested
    let updated_id = services::shortcut::update(db.clone(), updated_shortcut).await.unwrap();


    // Checking it's correct
    let expected_shortcut = services::shortcut::ShortcutDB {
        id: initial_db_shortcut.id,
        name: "Updated_Favorites".to_string(),
        description: "Updated_Description".to_string(),
        link: "Updated_Link".to_string(),
        icon: "Updated_icon".to_string()
    };

    let updated_db_shortcut = sqlx::query_as!(services::shortcut::ShortcutDB,"SELECT * FROM shortcuts where id = $1", updated_id)
        .fetch_one(&db).await?;

    assert_eq!(expected_shortcut, updated_db_shortcut, "Making sure the shortcut has been properly updated in the database");

    Ok(())
}

#[sqlx::test(fixtures("Shortcuts"))]
async fn creating_a_valid_shortcut_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let new_shortcut = routes::shortcut::ShortcutWeb {
        id: None,
        name: Some("New shortcut".to_string()),
        description: Some("New shortcut description".to_string()),
        link: Some("link/to/somewhere".to_string()),
        icon: Some("Updated_icon".to_string())
    };

    // What's tested
    let created_id = services::shortcut::create(db.clone(), new_shortcut.clone()).await.unwrap();

    let created_db_shortcut = sqlx::query_as!(services::shortcut::ShortcutDB,"SELECT * FROM shortcuts where id = $1", created_id)
        .fetch_one(&db).await?;

    // Checking it's correct
    let expected_shortcut = services::shortcut::ShortcutDB {
        id: created_id.try_into().expect("Should be a convertable number"),
        name: new_shortcut.name.unwrap(),
        description: new_shortcut.description.unwrap(),
        link: new_shortcut.link.unwrap(),
        icon: new_shortcut.icon.unwrap() 
    };

    assert_eq!(expected_shortcut, created_db_shortcut, "Making sure the shortcut has been properly created in the database");

    Ok(())
}

#[sqlx::test(fixtures("Shortcuts"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_shortcut = sqlx::query!("SELECT * from shortcuts where name = $1", "Favorites" )
        .fetch_one(&db).await?;

    // What's tested
    let affected_rows = services::shortcut::delete(db.clone(), initial_db_shortcut.id).await;
    assert!(affected_rows.is_ok());
    assert_eq!(affected_rows.unwrap(), 1);
    
    let remaining_db_shortcut = sqlx::query_as!(services::shortcut::ShortcutDB,"SELECT * FROM shortcuts where id = $1", initial_db_shortcut.id)
        .fetch_one(&db).await;
    assert!(remaining_db_shortcut.is_err());
        
    // Check that the other 2 rows of the fixture are left
    let remaing_rows = services::shortcut::get_all(db.clone()).await;
    assert!(remaing_rows.is_ok());
    assert_eq!(remaing_rows.unwrap().len(), 2);


    Ok(())
}

#[sqlx::test(fixtures("Shortcuts"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error>{
    //Setup
    let initial_db_shortcut = sqlx::query_as!(services::shortcut::ShortcutDB,"SELECT * from shortcuts where name = $1", "Favorites" )
        .fetch_one(&db).await?;

    // What's tested
    let result = services::shortcut::get_by_id(db.clone(),initial_db_shortcut.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_shortcut, result.unwrap());
    Ok(())
}


#[sqlx::test()]
async fn get_by_id_when_no_matching_shortcut_should_fail(db: Pool<Postgres>) -> Result<(), Error>{
    // What's tested
    let result = services::shortcut::get_by_id(db.clone(),7).await;
    assert!(result.is_err());

    Ok(())
}
