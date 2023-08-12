extern crate argon2;

mod config;
mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::HttpServer;
use actix_web::{middleware::Logger, App};
use actix_web_httpauth::extractors::bearer;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let config = config::read_config("config.toml".into())?;

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
        // CORS setup
        let cors = Cors::default()
            .allowed_origin(&config.cors.allowed_origin)
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config.clone()))
            .app_data(bearer_config.clone())
            // HACK: Key_pair copied and put both as data and "not"
            // Since it needs to be used both as extractor
            // and in definition of another extractor
            .app_data(key_pair.clone())
            .app_data(Data::new(key_pair.clone()))
            .configure(routes::setup)
    })
    .bind(("0.0.0.0", 8080))?
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
                } //_ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

#[cfg(test)]
mod tests;
