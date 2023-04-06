use sqlx::{Pool, MySql, Error};


#[sqlx::test(fixtures("Shortcuts"))]
async fn test_update_shortcut(db: Pool<MySql>) -> Result<(), Error> {
    use crate::routes::shortcut;
    
    // Setup
    let initial_db_shortcut = sqlx::query!("SELECT * from shortcuts where name=?", "Favorites" )
        .fetch_one(&db).await?;

    let updated_shortcut = shortcut::ShortcutWeb {
        id: Some(initial_db_shortcut.id),
        name: Some("Updated_Favorites".to_string()),
        desc: Some("Updated_Description".to_string()),
        link: Some("Updated_Link".to_string()),
        icon: Some("Updated_icon".to_string())
    };

    
    // What's tested
    let updated_id = shortcut::update_shortcut(db.clone(), updated_shortcut).await.unwrap().into_inner();
    

    // Checking it's correct
    let expected_shortcut = shortcut::ShortcutDB {
        id: initial_db_shortcut.id,
        name: "Updated_Favorites".to_string(),
        desc: "Updated_Description".to_string(),
        link: "Updated_Link".to_string(),
        icon: "Updated_icon".to_string()
    };

    let updated_db_shortcut = sqlx::query_as!(shortcut::ShortcutDB,"SELECT * FROM shortcuts where id=?", updated_id)
        .fetch_one(&db).await?;

    assert_eq!(expected_shortcut, updated_db_shortcut, "Making sure the shortcut has been properly updated in the database");

    Ok(())
}

#[sqlx::test(fixtures("Shortcuts"))]
async fn test_create_shortcut(db: Pool<MySql>) -> Result<(), Error> {
    use crate::routes::shortcut;
    //Setup
    let new_shortcut = shortcut::ShortcutWeb {
        id: None,
        name: Some("New shortcut".to_string()),
        desc: Some("New shortcut description".to_string()),
        link: Some("link/to/somewhere".to_string()),
        icon: Some("Updated_icon".to_string())
    };
    
    // What's tested

    let created_id = shortcut::create_shortcut(db.clone(), new_shortcut.clone()).await.unwrap().into_inner();

    let created_db_shortcut = sqlx::query_as!(shortcut::ShortcutDB,"SELECT * FROM shortcuts where id=?", created_id)
        .fetch_one(&db).await?;
    
    // Checking it's correct
    let expected_shortcut = shortcut::ShortcutDB {
        id: created_id.try_into().expect("Should be a convertable number"),
        name: new_shortcut.name.unwrap(),
        desc: new_shortcut.desc.unwrap(),
        link: new_shortcut.link.unwrap(),
        icon: new_shortcut.icon.unwrap() 
    };

    assert_eq!(expected_shortcut, created_db_shortcut, "Making sure the shortcut has been properly created in the database");

    Ok(())
}

