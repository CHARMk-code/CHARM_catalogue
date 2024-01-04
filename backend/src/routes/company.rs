use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use sqlx::PgPool;

use crate::models::company::CompanyWeb;
use crate::services;
use crate::services::auth::AuthedUser;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/company")
            .service(get_all_handler)
            .service(get_by_id_handler)
            .service(update_handler)
            .service(create_handler)
            .service(delete_handler),
    );
}

#[get("/")]
async fn get_all_handler(db: web::Data<PgPool>) -> Result<impl Responder> {
    let companies = services::company::get_all(&db).await?;

    Ok(HttpResponse::Ok().json(companies))
}

#[get("/{id}")]
async fn get_by_id_handler(db: web::Data<PgPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let company = services::company::get_by_id(&db, id).await?;

    Ok(HttpResponse::Ok().json(company))
}

#[put("/")]
async fn update_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<CompanyWeb>,
) -> Result<impl Responder> {
    let input_company = data.into_inner();

    let response = match input_company.id {
        Some(_) => {
            let active = input_company.active.as_ref();
            let charmtalk = input_company.charmtalk.as_ref();
            let name = input_company.name.as_ref();
            let description = input_company.description.as_ref();
            let unique_selling_point = input_company.unique_selling_point.as_ref();
            let contacts = input_company.contacts.as_ref();
            let contact_email = input_company.contact_email.as_ref();
            let employees_world = input_company.employees_world.as_ref();
            let employees_sweden = input_company.employees_sweden.as_ref();
            let website = input_company.website.as_ref();
            let talk_to_us_about = input_company.talk_to_us_about.as_ref();
            let logo = input_company.logo.as_ref();
            let map_image = input_company.map_image.as_ref();
            let booth_number = input_company.booth_number.as_ref();

            if active
                .and(charmtalk)
                .and(name)
                .and(description)
                .and(unique_selling_point)
                .and(contacts)
                .and(contact_email)
                .and(employees_world)
                .and(employees_sweden)
                .and(website)
                .and(talk_to_us_about)
                .and(logo)
                .and(map_image)
                .and(booth_number)
                .is_none()
            {
                HttpResponse::UnprocessableEntity().finish()
            } else {
                let company = services::company::update(&db, input_company).await?;
                HttpResponse::Ok().json(company)
            }
        }
        None => {
            let company = services::company::create(&db, &input_company).await?;
            HttpResponse::Created().json(company)
        }
    };

    Ok(response)
}

#[post("/")] // TODO Deprecatea in favor of put
async fn create_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    data: Json<CompanyWeb>,
) -> Result<impl Responder> {
    let input_company = data.into_inner();
    let affected_rows = services::company::create(&db, &input_company).await?;

    Ok(HttpResponse::Created().json(affected_rows))
}

#[delete("/{id}")]
async fn delete_handler(
    _user: AuthedUser,
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = services::company::delete(&db, id).await?;

    Ok(HttpResponse::Ok().json(affected_rows))
}
