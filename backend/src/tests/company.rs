use std::collections::HashSet;

use chrono::DateTime;
use sqlx::{Pool, Postgres, Error};

use crate::{routes, services};

#[sqlx::test(fixtures("Companies", "Tags", "Companies_tags"))]
async fn get_by_id_should_return_matching_row_in_db(db: Pool<Postgres>) -> Result<(), Error>{
    // Setup
    let initial_company = services::company::CompanyDB {
        id: 1,
    	last_updated: DateTime::parse_from_str("2023-06-25 21:00:00+02", "%F %T%#z").unwrap().into(),
    	active: false,
    	charmtalk: false,
    	name: "FrennFjord Consulting".to_string(),
    	description: "The company creating this catalogue".to_string(),
    	unique_selling_point: "They are old CHARM people".to_string(),
    	summer_job_description: "No summerjobs".to_string(),
    	summer_job_link: "".to_string(),
    	summer_job_deadline: DateTime::parse_from_str("2024-01-01 10:00:00+02", "%F %T%#z").unwrap().into(),
    	contacts: "Lucas Glimfjord".to_string(),
    	contact_email: "redacted".to_string(),
    	employees_world: 2,
    	employees_sweden: 2,
    	website: "frennfjord.se".to_string(),
    	talk_to_us_about: "CHARM and this Catalogue".to_string(),
    	logo: "logo.png".to_string(),
    	map_image: 0,
    	booth_number: 15,
    	tags: Some(vec![1,2]),
    };
    
    // What's tested
    let queried_company = services::company::get_by_id(db.clone(),initial_company.id).await;
    assert!(queried_company.is_ok());
    assert_eq!(initial_company, queried_company.unwrap());
    Ok(())
}

#[sqlx::test()]
async fn get_by_id_when_no_matching_company_should_fail(db: Pool<Postgres>) -> Result<(), Error>{
    // Setup
    let initial_row_amount = sqlx::query!("SELECT count(*) FROM companies").fetch_all(&db).await?;

    let invalid_id = i32::try_from(initial_row_amount.first().unwrap().count.unwrap()).unwrap() + 1;
    
    // What's tested
    let queried_company = services::company::get_by_id(db.clone(),invalid_id).await;
    assert!(queried_company.is_err(), "Should fail when querying for nonexisting id");

    Ok(())

}
#[sqlx::test(fixtures("Companies", "Tags", "Companies_tags"))]
async fn creating_a_valid_company_should_create_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct CompanyTagRel {
        company_id: i32,
        tag_id: i32,
    }
    // Setup
    let initial_companies = services::company::get_all(db.clone()).await.unwrap();
    let initial_companies_tags = sqlx::query_as!(CompanyTagRel, "SELECT * FROM companies_tags").fetch_all(&db).await?;

    let new_company = routes::company::CompanyWeb {
        id: None,
    	last_updated: Some(DateTime::parse_from_str("2023-06-26 10:00:00+02", "%F %T%#z").unwrap().into()),
    	active: Some(true),
    	charmtalk: Some(false),
    	name: Some("New company".to_string()),
    	description: Some("A new company description".to_string()),
    	unique_selling_point: Some("Nothing".to_string()),
    	summer_job_description: Some("No summerjobs".to_string()),
    	summer_job_link: Some("".to_string()),
    	summer_job_deadline: Some(DateTime::parse_from_str("2029-01-01 10:00:00+02", "%F %T%#z").unwrap().into()),
    	contacts: Some("No one".to_string()),
    	contact_email: Some("redacted".to_string()),
    	employees_world: Some(2),
    	employees_sweden: Some(2),
    	website: Some("null.se".to_string()),
    	talk_to_us_about: Some("New things".to_string()),
    	logo: Some("new_logo.png".to_string()),
    	map_image: Some(4),
    	booth_number: Some(42),
    	tags: Some([1,2].to_vec()),
    };

    // What's tested
    let created_query_result = services::company::create(db.clone(), new_company.clone()).await;
    assert!(created_query_result.is_ok(), "Should not fail on creation of new row");
    
    let new_companies = services::company::get_all(db.clone()).await.unwrap();
    let new_created_company: &services::company::CompanyDB = new_companies.iter()
        .filter(|r| &r.id == created_query_result.as_ref().unwrap())
        .collect::<Vec<&services::company::CompanyDB>>().first().unwrap();
    let new_other_companies: Vec<services::company::CompanyDB> = new_companies
        .iter()
        .cloned()
        .filter(|r| &r.id != created_query_result.as_ref().unwrap())
        .collect();

    let expected_company = services::company::CompanyDB {
        id: created_query_result.unwrap(),
    	last_updated: DateTime::parse_from_str("2023-06-26 10:00:00+02", "%F %T%#z").unwrap().into(),
    	active: true,
    	charmtalk: false,
    	name: "New company".to_string(),
    	description: "A new company description".to_string(),
    	unique_selling_point: "Nothing".to_string(),
    	summer_job_description: "No summerjobs".to_string(),
    	summer_job_link: "".to_string(),
    	summer_job_deadline: DateTime::parse_from_str("2029-01-01 10:00:00+02", "%F %T%#z").unwrap().into(),
    	contacts: "No one".to_string(),
    	contact_email: "redacted".to_string(),
    	employees_world: 2,
    	employees_sweden: 2,
    	website: "null.se".to_string(),
    	talk_to_us_about: "New things".to_string(),
    	logo: "new_logo.png".to_string(),
    	map_image: 4,
    	booth_number: 42,
    	tags: Some([1,2].to_vec()),
    };
    
    assert_eq!(&expected_company, new_created_company, "Making sure the company has been properly created in the database");
    assert_eq!(initial_companies.len() + 1, new_companies.len(), "One row should have been added to table");
    assert_eq!(new_other_companies, initial_companies,"Other rows should NOT have been changed");

    // Check company <-> tag relations are valid
    let new_companies_tags_rows = sqlx::query_as!(CompanyTagRel, "SELECT * FROM companies_tags").fetch_all(&db).await?;
    let new_tags_for_created_company : Vec<i32> = new_companies_tags_rows.iter().filter(|r| r.company_id == new_created_company.id).map(|r| r.tag_id).collect();
    let new_tags_for_other_companies : Vec<CompanyTagRel> = new_companies_tags_rows.iter().cloned().filter(|r| r.company_id != new_created_company.id).collect();

    assert_eq!(new_tags_for_created_company.len(), expected_company.tags.as_ref().unwrap().len(), "Number of created relations should match number of tags in company");
    assert_eq!(HashSet::<i32>::from_iter(new_tags_for_created_company.clone()).len(), new_tags_for_created_company.len(), "Should not be duplicates in created tags");
    assert_eq!(initial_companies_tags, new_tags_for_other_companies, "Should not change other companies tags");
    

    Ok(())
}


#[sqlx::test(fixtures("Companies", "Tags", "Companies_tags"))]
async fn valid_update_on_existing_company_should_update_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct CompanyTagRel {
        company_id: i32,
        tag_id: i32,
    }
    // Setup
    let initial_companies = services::company::get_all(db.clone()).await.unwrap();
    let initial_companies_tags = sqlx::query_as!(CompanyTagRel, "SELECT * FROM companies_tags").fetch_all(&db).await?;
   
    let initial_first_company = initial_companies.first().unwrap();
    let first_company_update = routes::company::CompanyWeb {
        id: Some(initial_first_company.id),
    	last_updated: None,
    	active: Some(true),
    	charmtalk: None,
    	name: None,
    	description: None,
    	unique_selling_point: None,
    	summer_job_description: None,
    	summer_job_link: None,
    	summer_job_deadline: None,
    	contacts: None,
    	contact_email: None,
    	employees_world: None,
    	employees_sweden: None,
    	website: None,
    	talk_to_us_about: None,
    	logo: None,
    	map_image: None,
    	booth_number: None,
    	tags: Some(vec![3]),
    };

    // What's tested
    
    //check output validity
    let update_query_result = services::company::update(db.clone(), first_company_update).await;
    assert!(update_query_result.is_ok(), "Update should not return an error");
    assert_eq!(update_query_result.unwrap(), initial_first_company.id, "Update should return the id of the updated row");
    
    // Check updates of company table
    let updated_companies = services::company::get_all(db.clone()).await.unwrap();
    let updated_first_company = updated_companies.first().unwrap();
    
    assert_eq!(
        updated_first_company, 
        &services::company::CompanyDB {
            id: initial_first_company.id,
            last_updated: DateTime::parse_from_str("2023-06-25 21:00:00+02", "%F %T%#z").unwrap().into(),
            active: true,
            charmtalk: false,
            name: "FrennFjord Consulting".to_string(),
            description: "The company creating this catalogue".to_string(),
            unique_selling_point: "They are old CHARM people".to_string(),
            summer_job_description: "No summerjobs".to_string(),
            summer_job_link: "".to_string(),
            summer_job_deadline: DateTime::parse_from_str("2024-01-01 10:00:00+02", "%F %T%#z").unwrap().into(),
            contacts: "Lucas Glimfjord".to_string(),
            contact_email: "redacted".to_string(),
            employees_world: 2,
            employees_sweden: 2,
            website: "frennfjord.se".to_string(),
            talk_to_us_about: "CHARM and this Catalogue".to_string(),
            logo: "logo.png".to_string(),
            map_image: 0,
            booth_number: 15,
            tags: Some(vec![3])
        },
    "The updated sure the company has been properly updated in the database");
    assert_eq!(initial_companies[1..],updated_companies[1..], "Update should not affect other rows");

    // Check updates of companies_tags table
    let remaining_companies_tags_rows = sqlx::query_as!(CompanyTagRel, "SELECT * FROM companies_tags").fetch_all(&db).await?;
    let remaining_tags_for_first_company : Vec<i32> = remaining_companies_tags_rows.iter().filter(|r| r.company_id == initial_first_company.id).map(|r| r.tag_id).collect();
    let remaining_tags_for_other_companies : Vec<CompanyTagRel> = remaining_companies_tags_rows.iter().cloned().filter(|r| r.company_id != initial_first_company.id).collect();

    assert_eq!(remaining_tags_for_first_company.len(), updated_first_company.tags.as_ref().unwrap().len(), "Number of created relations should match number of tags in company");
    assert_eq!(HashSet::<i32>::from_iter(remaining_tags_for_first_company.clone()).len(), remaining_tags_for_first_company.len(), "remaining_tags_for_first_company");
    assert_eq!(initial_companies_tags.iter().cloned().filter(|r| r.company_id != initial_first_company.id).collect::<Vec<CompanyTagRel>>(), remaining_tags_for_other_companies ,"Should not change other companies tags");
    
    Ok(())
}


#[sqlx::test(fixtures("Companies", "Tags", "Companies_tags"))]
async fn delete_on_existing_id_should_remove_correct_row_in_db(db: Pool<Postgres>) -> Result<(), Error> {
    // Setup
    let initial_companies = services::company::get_all(db.clone()).await.unwrap();
    let initial_companies_tags = sqlx::query!("SELECT * FROM companies_tags").fetch_all(&db).await?;
   
    let initial_first_company = initial_companies.first().unwrap();
    let removed_id = initial_first_company.id;

   
    // What's tested
    let remove_query_result = services::company::delete(db.clone(), removed_id.clone()).await;
    assert!(remove_query_result.is_ok());
    assert_eq!(remove_query_result.unwrap(), 1, "Should affect one row");
   
    
    let removed_company = services::company::get_by_id(db.clone(), removed_id.clone()).await;
    assert!(removed_company.is_err(), "Database query should fail for removed id");
    
    //Check that company has been removed
    let remaining_company_rows = sqlx::query!("SELECT id FROM companies").fetch_all(&db).await?;
    assert!(remaining_company_rows.iter().all(|r| r.id != removed_id.clone()), "Should not return removed id when querying remaining rows");
    assert_eq!(remaining_company_rows.len()+1, initial_companies.len(), "Remaining rows +1 should be equal to initial number of rows" );
    
    //Check that company <-> tag relations have been removed
    let remaining_companies_tags_rows = sqlx::query!("SELECT * FROM companies_tags").fetch_all(&db).await?;
    assert!(remaining_companies_tags_rows.iter().all(|r| r.company_id != removed_id), "Should not return any relations containing removed company");
    assert_eq!(
        remaining_companies_tags_rows.len() + initial_first_company.tags.as_ref().unwrap().len(), 
        initial_companies_tags.len(), 
        "Remaining relations + tags in removed company should equal initial number of relations");

    Ok(())
}
