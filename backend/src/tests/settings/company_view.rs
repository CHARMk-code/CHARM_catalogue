use sqlx::{Error, Pool, Postgres};

use crate::{routes, services};

#[sqlx::test()]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    //Setup
    let initial_db_company_card = services::settings::company_view::CompanyCardDB {
        id: 1,
        name: "logo".to_string(),
        text: "Logo".to_string(),
        active: true,
    };

    // What's tested
    let result =
        services::settings::company_view::get_by_id(db.clone(), initial_db_company_card.id).await;
    assert!(result.is_ok(), "Get by id should not fail");
    assert_eq!(initial_db_company_card, result.unwrap());
    Ok(())
}

#[sqlx::test()]
async fn reset_should_not_change_anything_from_default(db: Pool<Postgres>) -> Result<(), Error> {
    // Setup
    let initial_company_view = services::settings::company_view::get_all(db.clone()).await;
    assert!(
        initial_company_view.is_ok(),
        "Get all copmany cards should not fail"
    );

    let reset_result = services::settings::company_view::reset(db.clone()).await;
    assert!(reset_result.is_ok(), "reset function should not fail");

    let reset_company_view = services::settings::company_view::get_all(db.clone()).await;
    assert_eq!(
        initial_company_view.unwrap(),
        reset_company_view.unwrap(),
        "Reset should not have changed any data from default"
    );

    Ok(())
}

#[sqlx::test()]
async fn valid_update_on_existing_company_view_should_update_row_in_db(
    db: Pool<Postgres>,
) -> Result<(), Error> {
    // Setup
    let initial_company_views = services::settings::company_view::get_all(db.clone())
        .await
        .unwrap();

    let initial_first_company_view = initial_company_views.first().unwrap();
    let initial_other_company_views = initial_company_views
        .iter()
        .filter(|r| r.id != initial_first_company_view.id)
        .collect::<Vec<&services::settings::company_view::CompanyCardDB>>();

    let first_company_view_update = routes::settings::company_view::CompanyCardWeb {
        id: Some(initial_first_company_view.id),
        name: Some("UPDATED Logo".to_string()),
        text: Some("UPDATED logo".to_string()),
        active: None,
    };

    // What's tested

    // Check output validity
    let update_query_result =
        services::settings::company_view::update(db.clone(), first_company_view_update).await;
    assert!(
        update_query_result.is_ok(),
        "Update should not return an error"
    );
    assert_eq!(
        update_query_result.unwrap(),
        initial_first_company_view.id,
        "Update should return the id of the updated row"
    );

    // Check updates of company_view table
    let updated_company_views = services::settings::company_view::get_all(db.clone())
        .await
        .unwrap();
    let updated_first_company_view = updated_company_views
        .iter()
        .cloned()
        .filter(|r| r.id == initial_first_company_view.id)
        .next()
        .unwrap();
    let updated_other_company_views = updated_company_views
        .iter()
        .filter(|r| r.id != initial_first_company_view.id)
        .collect::<Vec<&services::settings::company_view::CompanyCardDB>>();

    assert_eq!(
        updated_first_company_view,
        services::settings::company_view::CompanyCardDB {
            id: initial_first_company_view.id,
            name: "UPDATED Logo".to_string(),
            text: "UPDATED logo".to_string(),
            active: true,
        },
        "The updated sure the company_view has been properly updated in the database"
    );
    assert_eq!(
        initial_other_company_views, updated_other_company_views,
        "Update should not affect other rows"
    );

    Ok(())
}
