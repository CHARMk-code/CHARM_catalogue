use sqlx::{Pool, Postgres, Error};

use crate::{routes, services};

#[sqlx::test(fixtures("Tags"))]
async fn valid_update_on_existing_tag_should_update_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {

    // Setup
    let initial_db_tag = sqlx::query!("SELECT * from tags where name = $1", "A")
        .fetch_one(&db).await?;

    let updated_tag = routes::tag::TagWeb {
        id: Some(initial_db_tag.id),
        name: Some("NEW A".to_string()),
        parent_tag: None,
        up_votes: None,
        down_votes: None,
        crowd_sourced: None,
        icon: None,
        division: None,
        business_area: None,
        looking_for: None,
        offering: None,
        language: None,
        fair_area: None
    };


    // What's tested
    let updated_id = services::tag::update(db.clone(), updated_tag).await.unwrap();


    // Checking it's correct
    let expected_tag = services::tag::TagDB {
        id: initial_db_tag.id,
        name: "NEW A".to_string(),
        parent_tag: 0,
        up_votes: 0,
        down_votes: 0,
        crowd_sourced: false,
        icon: "A.svg".to_string(),
        division: true,
        business_area: false,
        looking_for: false,
        offering: false,
        language: false,
        fair_area: false
    };

    let updated_db_tag = sqlx::query_as!(services::tag::TagDB, "SELECT * FROM tags where id = $1", updated_id)
        .fetch_one(&db).await?;

    assert_eq!(expected_tag, updated_db_tag, "Making sure the tag has been properly updated in the database");

    Ok(())
}

#[sqlx::test(fixtures("Tags"))]
async fn creating_a_valid_tag_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let new_tag = routes::tag::TagWeb {
        id: None,
        name: Some("NEW TAG".to_string()),
        parent_tag: Some(0),
        up_votes: Some(0),
        down_votes: Some(0),
        crowd_sourced: Some(false),
        icon: Some("NEW_TAG.svg".to_string()),
        division: Some(false),
        business_area: Some(false),
        looking_for: Some(false),
        offering: Some(false),
        language: Some(false),
        fair_area: Some(true)
    };

    // What's tested
    let created_id = services::tag::create(db.clone(), new_tag.clone()).await.unwrap();

    let created_db_tag = sqlx::query_as!(services::tag::TagDB,"SELECT * FROM tags where id = $1", created_id)
        .fetch_one(&db).await?;

    // Checking it's correct
    let expected_tag = services::tag::TagDB {
        id: created_id,
        name: "NEW TAG".to_string(),
        parent_tag: 0,
        up_votes: 0,
        down_votes: 0,
        crowd_sourced: false,
        icon: "NEW_TAG.svg".to_string(),
        division: false,
        business_area: false,
        looking_for: false,
        offering: false,
        language: false,
        fair_area: true
    };

    assert_eq!(expected_tag, created_db_tag, "Making sure the tag has been properly created in the database");

    Ok(())
}

#[sqlx::test(fixtures("Tags"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_tag = sqlx::query!("SELECT * from tags where name = $1", "A" )
        .fetch_one(&db).await?;

    // What's tested
    let affected_rows = services::tag::delete(db.clone(), initial_db_tag.id).await;
    assert!(affected_rows.is_ok());
    assert_eq!(affected_rows.unwrap(), 1);
    
    let remaining_db_tag = sqlx::query_as!(services::tag::TagDB,"SELECT * FROM tags where id = $1", initial_db_tag.id)
        .fetch_one(&db).await;
    assert!(remaining_db_tag.is_err());
        
    // Check that the other 2 rows of the fixture are left
    let remaing_rows = services::tag::get_all(db.clone()).await;
    assert!(remaing_rows.is_ok());
    assert_eq!(remaing_rows.unwrap().len(), 4);


    Ok(())
}

#[sqlx::test(fixtures("Tags"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error>{
    //Setup
    let initial_db_tag = sqlx::query_as!(services::tag::TagDB,"SELECT * from tags where name = $1", "A" )
        .fetch_one(&db).await?;

    // What's tested
    let result = services::tag::get_by_id(db.clone(),initial_db_tag.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_tag, result.unwrap());
    Ok(())
}


#[sqlx::test()]
async fn get_by_id_when_no_matching_tag_should_fail(db: Pool<Postgres>) -> Result<(), Error>{
    // What's tested
    let result = services::tag::get_by_id(db.clone(),7).await;
    assert!(result.is_err());

    Ok(())
}
