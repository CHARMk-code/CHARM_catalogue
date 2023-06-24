mod routes;
mod services;

use actix_web::web::Data;
use actix_web::{middleware::Logger, App};
use actix_web::HttpServer;
use std::env;
use sqlx::postgres::PgPoolOptions;

// access logs are printed with the INFO level so ensure it is enabled by default


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));


    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url).await.expect("Failed to initialize Database pool");

    sqlx::migrate!().run(&pool).await.expect("Migrations failed");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(pool.clone()))
            
            .configure(routes::setup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Should probably move to separate file
mod errors {
    use actix_web::{ResponseError, HttpResponse};
    use derive_more::{Display, From};


    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        SQLxError(sqlx::Error)
    }

    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::SQLxError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                },
                _ => HttpResponse::InternalServerError().finish()

            }
        } 
    }
}
 
#[cfg(test)]
mod tests;
