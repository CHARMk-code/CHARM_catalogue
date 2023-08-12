use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::http::header::Header;
use actix_web::{FromRequest, HttpRequest};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::authorization;
use actix_web_httpauth::headers::www_authenticate::bearer;
use argon2::Config;
use jwt_simple::algorithms::EdDSAKeyPairLike;
use jwt_simple::prelude::{Claims, Duration, Ed25519KeyPair};
use jwt_simple::prelude::{EdDSAPublicKeyLike, JWTClaims, NoCustomClaims};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::{query, query_as, Pool, Postgres};

use crate::errors::MyError;
use crate::models::user::{UserDB, UserWeb};

pub struct AuthedUser(JWTClaims<NoCustomClaims>);

impl FromRequest for AuthedUser {
    type Future = Ready<Result<Self, Self::Error>>;
    type Error = AuthenticationError<bearer::Bearer>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> <Self as FromRequest>::Future {
        ready((|| {
            let bearer = req
                .app_data::<actix_web_httpauth::extractors::bearer::Config>()
                .unwrap()
                .as_ref();

            let auth = authorization::Authorization::<authorization::Bearer>::parse(req)
                .map_err(|_| AuthenticationError::new(bearer.clone()))?
                .into_scheme();
            let token = auth.token().to_string();
            let key_pair = req
                .app_data::<Ed25519KeyPair>()
                .expect("KeyPair from main file")
                .clone();

            let claims = validate_token::<NoCustomClaims>(&token, key_pair)
                .map_err(|_| AuthenticationError::new(bearer.clone()))?;

            Ok(AuthedUser(claims))
        })())
    }
}

pub async fn get_user(db: &Pool<Postgres>) -> Result<UserDB, actix_web::Error> {
    // HACK: This should be fixed when a proper authorization system is set up. Currently creates a
    // password "password" if the db returns an error (most likely, no password has previously been set)
    let user = query_as!(UserDB, "SELECT * FROM users")
        .fetch_one(db)
        .await
        .map_err(MyError::SQLxError)?;

    Ok(user)
}

pub async fn create_user(
    db: &Pool<Postgres>,
    user: UserWeb,
    salt: &[u8],
) -> Result<i32, actix_web::Error> {
    let hash =
        hash_password(&user.password, salt).map_err(actix_web::error::ErrorInternalServerError)?;
    let user = query!(
        "INSERT INTO users (password) values ($1) returning id",
        hash
    )
    .fetch_one(db)
    .await
    .map_err(MyError::SQLxError)?;

    Ok(user.id)
}

pub fn hash_password(password_to_hash: &String, salt: &[u8]) -> Result<String, argon2::Error> {
    let config = Config::default();

    argon2::hash_encoded(password_to_hash.as_bytes(), salt, &config)
}

pub fn validate_password(
    password_to_validate: &String,
    valid_password_hash: &String,
    salt: &[u8],
) -> Result<bool, argon2::Error> {
    let new_hash = hash_password(password_to_validate, salt)?;

    Ok(&new_hash == valid_password_hash)
}

pub fn generate_token(key_pair: Ed25519KeyPair) -> Result<String, jwt_simple::Error> {
    let claims = Claims::create(Duration::from_days(1));

    Ok(key_pair.sign(claims)?)
}

pub fn validate_token<CustomClaims: Serialize + DeserializeOwned>(
    token: &String,
    key_pair: Ed25519KeyPair,
) -> Result<JWTClaims<CustomClaims>, jwt_simple::Error> {
    let public_key = key_pair.public_key();

    public_key.verify_token::<CustomClaims>(token, None)
}
