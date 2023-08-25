use actix_web::{
    get, post, put,
    web::{self, Json},
    HttpResponse, Responder, Result,
};

use jwt_simple::prelude::Ed25519KeyPair;
use sqlx::PgPool;

use crate::{
    config::Config,
    models::user::{UserLoginWeb, UserWeb},
    services::{
        self,
        auth::{create_user, AuthedUser},
    },
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth").service(get_token), // .service(register_handler)
                                                // .service(update_password_handler),
    );
}

#[post("/")]
async fn get_token(
    db: web::Data<PgPool>,
    keypair: web::Data<Ed25519KeyPair>,
    config: web::Data<Config>,
    data: Json<UserLoginWeb>,
) -> Result<impl Responder> {
    let salt: &[u8] = config.password_salt.as_bytes();

    let possible_user = services::auth::get_user(db.as_ref()).await;

    // HACK: This should be fixed when a proper authorization system is set up. Currently creates a
    // password "password" if the db returns an error (most likely, no password has previously been set)
    let user = match possible_user {
        Ok(user) => user,
        Err(..) => {
            create_user(
                &db,
                UserWeb {
                    password: "password".to_string(),
                },
                salt,
            )
            .await?;
            services::auth::get_user(db.as_ref()).await?
        }
    };

    let is_valid_pass = services::auth::validate_password(&data.password, &user.password, salt);

    if is_valid_pass.is_ok() && is_valid_pass.unwrap() {
        let token = services::auth::generate_token(keypair.as_ref().clone());

        if token.is_ok() {
            return Ok(HttpResponse::Ok().json(token.unwrap()));
        }
    }
    Ok(HttpResponse::Unauthorized().into())
}

// #[put("/")]
// async fn update_password_handler(
//     _db: web::Data<PgPool>,
//     _user: AuthedUser,
// ) -> Result<impl Responder> {
//     Ok(HttpResponse::Ok())
// }
//
// #[get("/")]
// async fn register_handler(_user: AuthedUser, db: web::Data<PgPool>, config: web::Data<Config>, data: Json<UserWeb>) -> Result<impl Responder> {
//
//     let user_id = services::auth::create_user(&db, data.into_inner(), salt).await?;
//
//     Ok(HttpResponse::Ok().json(user_id))
// }
