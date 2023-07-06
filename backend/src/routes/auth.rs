use actix_web::{
    get, post, put,
    web::{self, Json},
    HttpResponse, Responder, Result,
};

use jwt_simple::prelude::{Ed25519KeyPair, NoCustomClaims};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::services::{
    self,
    auth::{AuthedUser, UserDB},
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(get_token)
            .service(register_handler)
            .service(update_password_handler),
    );
}

#[derive(Deserialize, Serialize)]
pub struct UserLoginWeb {
    pub password: String,
}

// NOTE: Currently the same as UserLoginWe but with more sofisticated accounts a user
// will be more than a password and most likely more than what's needed for login
#[derive(Debug, Deserialize, Serialize)]
pub struct UserWeb {
    pub password: String,
}

#[post("/")]
async fn get_token(
    db: web::Data<PgPool>,
    keypair: web::Data<Ed25519KeyPair>,
    data: Json<UserLoginWeb>,
) -> Result<impl Responder> {
    let possible_user = services::auth::get_user(db.as_ref()).await?;

    let is_valid_pass = services::auth::validate_password(&data.password, &possible_user.password);

    if is_valid_pass.is_ok() && is_valid_pass.unwrap() {
        let token = services::auth::generate_token(keypair.as_ref().clone());

        if token.is_ok() {
            return Ok(HttpResponse::Ok().json(token.unwrap()));
        }
    }
    Ok(HttpResponse::Unauthorized().into())
}

#[put("/")]
async fn update_password_handler(
    _db: web::Data<PgPool>,
    _user: AuthedUser,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[get("/")]
async fn register_handler(db: web::Data<PgPool>, data: Json<UserWeb>) -> Result<impl Responder> {
    println!("register: {:?}", data);
    let user_id = services::auth::create_user(&db, data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user_id))
    // }
}
