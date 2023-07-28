extern crate argon2;

mod routes;
mod services;

use actix_web::web::Data;
use actix_web::HttpServer;
use actix_web::{middleware::Logger, App};
use actix_web_httpauth::extractors::bearer;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::path::{Path, PathBuf};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    //Base upload path
    let base_path: PathBuf = PathBuf::from("/upload");

    // DB setup
    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to initialize Database pool");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed");

    //Auth cryptography setup
    let key_pair = jwt_simple::prelude::Ed25519KeyPair::generate();

    //Bearer return setup
    let bearer_config = bearer::Config::default();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            //.wrap(HttpAuthentication::bearer(bearer_auth_validator))
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(base_path.clone()))
            .app_data(bearer_config.clone())
            .app_data(key_pair.clone()) // HACK: Key_pair copied and put both as data and "not"
            .app_data(Data::new(key_pair.clone())) // Since it needs to be used both as extractor
            .configure(routes::setup) // and in definition of another extractor
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Should probably move to separate file
mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use derive_more::{Display, From};

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        SQLxError(sqlx::Error),
    }

    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::SQLxError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

#[cfg(test)]
mod tests;
