use sqlx::{Error, Pool, Postgres};

use crate::{routes, services};

#[sqlx::test(fixtures("Tags"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_tag = services::tag::TagDB {
        id: 1,
        name: "A".to_string(),
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
        fair_area: false,
    };

    // What's tested
    let result = services::tag::get_by_id(db.clone(), initial_db_tag.id).await;
    assert!(result.is_ok());
    assert_eq!(initial_db_tag, result.unwrap());
    Ok(())
}

#[sqlx::test()]
async fn get_by_id_when_no_matching_tag_should_fail(db: Pool<Postgres>) -> Result<(), Error> {
    // Setup
    let initial_row_amount = sqlx::query!("SELECT count(*) FROM tags")
        .fetch_all(&db)
        .await?;

    let invalid_id = i32::try_from(initial_row_amount.first().unwrap().count.unwrap()).unwrap() + 1;

    // What's tested
    let queried_tag = services::tag::get_by_id(db.clone(), invalid_id).await;
    assert!(
        queried_tag.is_err(),
        "Should fail when querying for nonexisting id"
    );

    Ok(())
}

#[sqlx::test(fixtures("Tags"))]
async fn creating_a_valid_tag_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_tags = services::tag::get_all(db.clone()).await.unwrap();

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
        fair_area: Some(true),
    };

    // What's tested
    let created_query_result = services::tag::create(&db, &new_tag).await;
    assert!(
        created_query_result.is_ok(),
        "Should not fail on creation of new row"
    );

    let new_tags = services::tag::get_all(db.clone()).await.unwrap();
    let new_created_tag: &services::tag::TagDB = new_tags
        .iter()
        .filter(|r| &r.id == created_query_result.as_ref().unwrap())
        .collect::<Vec<&services::tag::TagDB>>()
        .first()
        .unwrap();
    let new_other_tags: Vec<services::tag::TagDB> = new_tags
        .iter()
        .cloned()
        .filter(|r| &r.id != created_query_result.as_ref().unwrap())
        .collect();

    let expected_tag = services::tag::TagDB {
        id: created_query_result.unwrap(),
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
        fair_area: true,
    };

    assert_eq!(
        &expected_tag, new_created_tag,
        "Making sure the tag has been properly created in the database"
    );
    assert_eq!(
        initial_tags.len() + 1,
        new_tags.len(),
        "One row should have been added to table"
    );
    assert_eq!(
        new_other_tags, initial_tags,
        "Other rows should NOT have been changed"
    );

    Ok(())
}

#[sqlx::test(fixtures("Tags"))]
async fn valid_update_on_existing_tag_should_update_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_tags = services::tag::get_all(db.clone()).await.unwrap();

    let initial_first_tag = initial_tags.first().unwrap();
    let initial_other_tags = initial_tags
        .iter()
        .filter(|r| r.id != initial_first_tag.id)
        .collect::<Vec<&services::tag::TagDB>>();

    let first_tag_update = routes::tag::TagWeb {
        id: Some(initial_first_tag.id),
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
        fair_area: None,
    };

    // What's tested

    // Check output validity
    let update_query_result = services::tag::update(db.clone(), first_tag_update).await;
    assert!(
        update_query_result.is_ok(),
        "Update should not return an error"
    );
    assert_eq!(
        update_query_result.unwrap(),
        initial_first_tag.id,
        "Update should return the id of the updated row"
    );

    // Check updates of tag table
    let updated_tags = services::tag::get_all(db.clone()).await.unwrap();
    let updated_first_tag = updated_tags
        .iter()
        .cloned()
        .filter(|r| r.id == initial_first_tag.id)
        .next()
        .unwrap();
    let updated_other_tags = updated_tags
        .iter()
        .filter(|r| r.id != initial_first_tag.id)
        .collect::<Vec<&services::tag::TagDB>>();

    assert_eq!(
        updated_first_tag,
        services::tag::TagDB {
            id: initial_first_tag.id,
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
        },
        "The updated sure the tag has been properly updated in the database"
    );
    assert_eq!(
        initial_other_tags, updated_other_tags,
        "Update should not affect other rows"
    );

    Ok(())
}

#[sqlx::test(fixtures("Tags"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_tags = services::tag::get_all(db.clone()).await.unwrap();

    let initial_first_tag = initial_tags.first().unwrap();
    let removed_id = initial_first_tag.id;

    // What's tested
    let remove_query_result = services::tag::delete(db.clone(), removed_id.clone()).await;
    assert!(remove_query_result.is_ok());
    assert_eq!(remove_query_result.unwrap(), 1, "Should affect one row");

    let removed_tag = services::tag::get_by_id(db.clone(), removed_id.clone()).await;
    assert!(
        removed_tag.is_err(),
        "Database query should fail for removed id"
    );

    //Check that tag has been removed
    let remaining_tag_rows = sqlx::query!("SELECT id FROM tags").fetch_all(&db).await?;
    assert!(
        remaining_tag_rows
            .iter()
            .all(|r| r.id != removed_id.clone()),
        "Should not return removed id when querying remaining rows"
    );
    assert_eq!(
        remaining_tag_rows.len() + 1,
        initial_tags.len(),
        "Remaining rows +1 should be equal to initial number of rows"
    );

    Ok(())
}
