use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::routes::company::CompanyWeb;
use crate::{errors::MyError, services::is_optional_field_or_default};

use chrono::{DateTime, Utc};

use super::is_valid_required_field;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct CompanyDB {
    pub id: i32,
    pub last_updated: DateTime<Utc>,
    pub active: bool,
    pub charmtalk: bool,
    pub name: String,
    pub description: String,
    pub unique_selling_point: String,
    pub summer_job_description: String, // Allow publishing of generic job listings on the
    pub summer_job_link: String,        // company page
    pub summer_job_deadline: DateTime<Utc>,
    pub contacts: String,
    pub contact_email: String,
    pub employees_world: i32,
    pub employees_sweden: i32,
    pub website: String,
    pub talk_to_us_about: String,
    pub logo: String,
    pub map_image: i32,
    pub booth_number: i32,
    pub tags: Option<Vec<i32>>,
}

pub async fn create(db: &Pool<Postgres>, data: &CompanyWeb) -> Result<i32, actix_web::Error> {
    let last_updated = Utc::now();
    let active = is_valid_required_field(&data.active)?;
    let charmtalk = is_valid_required_field(&data.charmtalk)?;
    let name = is_valid_required_field(&data.name)?;
    let description = is_valid_required_field(&data.description)?;
    let unique_selling_point = is_optional_field_or_default(&data.unique_selling_point, "".to_string())?;
    let summer_job_description =
        is_optional_field_or_default(&data.summer_job_description, "".to_string())?;
    let summer_job_link = is_optional_field_or_default(&data.summer_job_link, "".to_string())?;
    let summer_job_deadline =
        is_optional_field_or_default(&data.summer_job_deadline, Utc::now())?;
    let contacts = is_valid_required_field(&data.contacts)?;
    let contact_email = is_valid_required_field(&data.contact_email)?;
    let employees_world = is_valid_required_field(&data.employees_world)?;
    let employees_sweden = is_valid_required_field(&data.employees_sweden)?;
    let website = is_valid_required_field(&data.website)?;
    let talk_to_us_about = is_valid_required_field(&data.talk_to_us_about)?;
    let logo = is_valid_required_field(&data.logo)?;
    let map_image = is_valid_required_field(&data.map_image)?;
    let booth_number = is_valid_required_field(&data.booth_number)?;
    let tags = is_valid_required_field(&data.tags)?;

    //check that tags exist, fail if not
    struct QueryReturn {
        count: Option<i64>,
    }
    let tag_count = sqlx::query_as!(
        QueryReturn,
        "SELECT count(id) from tags where id = ANY(select * from UNNEST($1::int[]))",
        &tags
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;
    if usize::try_from(tag_count.count.unwrap_or(0)) != Ok(tags.len()) {
        return Err(actix_web::error::ErrorUnprocessableEntity(
            "One or more of the received company tags are not present in database",
        ));
    };

    let create_company_query_result = sqlx::query!("INSERT INTO companies (last_updated, active, charmtalk, name, description, unique_selling_point, summer_job_description, summer_job_link, summer_job_deadline, contacts, contact_email, employees_world, employees_sweden, website, talk_to_us_about, logo, map_image, booth_number) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18) returning id",
last_updated, active, charmtalk, name, description, unique_selling_point, summer_job_description, summer_job_link, summer_job_deadline, contacts, contact_email, employees_world, employees_sweden, website, talk_to_us_about, logo, map_image, booth_number)
        .fetch_one(db).await.map_err(MyError::SQLxError)?;

    let new_company_id = create_company_query_result.id;

    // Create the company (one) to tag (many) relation
    if tags.len() > 0 {
        let _create_tag_relation_query_result = sqlx::query!(
            "INSERT INTO companies_tags (company_id, tag_id) (SELECT * FROM UNNEST(array_fill($1::int, ARRAY[$3::int]), $2::int[]))",
            &new_company_id,
            &tags,
            tags.len() as i32
        )
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;
    }

    Ok(new_company_id)
}

pub async fn update(db: &Pool<Postgres>, data: CompanyWeb) -> Result<i32, actix_web::Error> {
    let id = data.id.expect("Should have id to update");

    // In an optimal world we shouldn't need this query
    // (TODO change the second query to only use the data values that will be updated)
    let company = sqlx::query_as!(
        CompanyDB,
        "SELECT C.*, array_remove(array_agg(T.id), NULL) as tags FROM Companies as C \
        left join companies_tags C_T on C_T.company_id = C.id \
        left join tags T on C_T.tag_id = T.id \
        where C.id = $1 \
        group by C.id \
        ",
        id
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;

    let new_tags = data.tags.unwrap_or(Vec::new());
    let current_tags = company.tags.unwrap_or(Vec::new());

    let last_updated = Some(Utc::now());
    let active = data.active.as_ref();
    let charmtalk = data.charmtalk.as_ref();
    let name = data.name.as_ref();
    let description = data.description.as_ref();
    let unique_selling_point = data.unique_selling_point.as_ref();
    let summer_job_description = data.summer_job_description.as_ref();
    let summer_job_link = data.summer_job_link.as_ref();
    let summer_job_deadline = data.summer_job_deadline.as_ref();
    let contacts = data.contacts.as_ref();
    let contact_email = data.contact_email.as_ref();
    let employees_world = data.employees_world.as_ref();
    let employees_sweden = data.employees_sweden.as_ref();
    let website = data.website.as_ref();
    let talk_to_us_about = data.talk_to_us_about.as_ref();
    let logo = data.logo.as_ref();
    let map_image = data.map_image.as_ref();
    let booth_number = data.booth_number.as_ref();

    //check that updated tags exist, fail if not
    struct QueryReturn {
        count: Option<i64>,
    }
    let tag_count = sqlx::query_as!(
        QueryReturn,
        "SELECT count(id) from tags where id = ANY(select * from UNNEST($1::int[]))",
        &new_tags
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;
    if usize::try_from(tag_count.count.unwrap_or(0)) != Ok(new_tags.len()) {
        return Err(actix_web::error::ErrorUnprocessableEntity(
            "One or more of the received company tags are not present in database",
        ));
    };

    let query_result =
        sqlx::query!("UPDATE companies SET last_updated = $1, active = $2, charmtalk = $3, name = $4, description = $5, unique_selling_point = $6, summer_job_description = $7, summer_job_link = $8, summer_job_deadline = $9, contacts = $10, contact_email = $11, employees_world = $12, employees_sweden = $13, website = $14, talk_to_us_about = $15, logo = $16, map_image = $17, booth_number = $18 where id = $19 returning id",
            if last_updated.is_some() {last_updated.unwrap()} else {company.last_updated},
            if active.is_some() {active.unwrap()} else {&company.active},
            if charmtalk.is_some() {charmtalk.unwrap()} else {&company.charmtalk},
            if name.is_some() {name.unwrap()} else {&company.name},
            if description.is_some() {description.unwrap()} else {&company.description},
            if unique_selling_point.is_some() {unique_selling_point.unwrap()} else {&company.unique_selling_point},
            if summer_job_description.is_some() {summer_job_description.unwrap()} else {&company.summer_job_description},
            if summer_job_link.is_some() {summer_job_link.unwrap()} else {&company.summer_job_link},
            if summer_job_deadline.is_some() {summer_job_deadline.unwrap()} else {&company.summer_job_deadline},
            if contacts.is_some() {contacts.unwrap()} else {&company.contacts},
            if contact_email.is_some() {contact_email.unwrap()} else {&company.contact_email},
            if employees_world.is_some() {employees_world.unwrap()} else {&company.employees_world},
            if employees_sweden.is_some() {employees_sweden.unwrap()} else {&company.employees_sweden},
            if website.is_some() {website.unwrap()} else {&company.website},
            if talk_to_us_about.is_some() {talk_to_us_about.unwrap()} else {&company.talk_to_us_about},
            if logo.is_some() {logo.unwrap()} else {&company.logo},
            if map_image.is_some() {map_image.unwrap()} else {&company.map_image},
            if booth_number.is_some() {booth_number.unwrap()} else {&company.booth_number},
            data.id)
        .fetch_one(db).await.map_err(MyError::SQLxError)?;

    // Update the company (one) to tag (many) relation
    let new_tags_set: HashSet<i32> = HashSet::from_iter(new_tags);
    let current_tags_set: HashSet<i32> = HashSet::from_iter(current_tags);

    //Remove tags
    let tags_to_remove: Vec<i32> = current_tags_set
        .difference(&new_tags_set)
        .copied()
        .collect();
    if tags_to_remove.len() > 0 {
        sqlx::query!(
            "DELETE FROM companies_tags where company_id = $1 \
            AND tag_id = ANY (SELECT * FROM UNNEST($2::int[])) \
        ",
            data.id,
            &tags_to_remove
        )
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;
    }

    //Add tags
    let tags_to_add: Vec<i32> = new_tags_set
        .difference(&current_tags_set)
        .copied()
        .collect();
    if tags_to_add.len() > 0 {
        sqlx::query!(
            "INSERT INTO companies_tags (company_id, tag_id) 
            (SELECT * FROM UNNEST( 
                array_fill($1::int, ARRAY[$3::int]),
                $2::int[]))
        ",
            data.id,
            &tags_to_add,
            tags_to_add.len() as i32
        )
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;
    }

    Ok(query_result.id)
}

pub async fn delete(db: &Pool<Postgres>, id: i32) -> Result<u64, actix_web::Error> {
    let delete_company_result = sqlx::query!("DELETE FROM companies WHERE id = $1", id)
        .execute(db)
        .await
        .map_err(MyError::SQLxError)?;

    let _delete_companies_tags_result =
        sqlx::query!("DELETE FROM companies_tags WHERE company_id = $1", id)
            .execute(db)
            .await
            .map_err(MyError::SQLxError)?;

    Ok(delete_company_result.rows_affected())
}

pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<CompanyDB>, actix_web::Error> {
    let query_result = sqlx::query_as!(
        CompanyDB,
        "\
        SELECT C.*, array_remove(array_agg(T.id), NULL) as tags FROM Companies as C \
        left join companies_tags C_T on C_T.company_id = C.id \
        left join tags T on C_T.tag_id = T.id \
        group by C.id"
    )
    .fetch_all(db)
    .await
    .map_err(MyError::SQLxError)?;

    let mapped_result = query_result
        .into_iter()
        .map(|company| CompanyDB {
            tags: Some(company.tags.unwrap_or(Vec::new())),
            ..company
        })
        .collect();

    Ok(mapped_result)
}

pub async fn get_by_id(db: &Pool<Postgres>, id: i32) -> Result<CompanyDB, actix_web::Error> {
    let query_result = sqlx::query_as!(
        CompanyDB,
        "\
        SELECT C.*, array_remove(array_agg(T.id), NULL) as tags FROM Companies as C \
        left join companies_tags C_T on C_T.company_id = C.id \
        left join tags T on C_T.tag_id = T.id \
        where C.id = $1 group by C.id",
        id
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(CompanyDB {
        tags: Some(query_result.tags.unwrap_or(Vec::new())),
        ..query_result
    })
}
