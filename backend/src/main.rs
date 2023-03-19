mod routes;


#[macro_use]
extern crate rocket;

use rocket_db_pools::sqlx;
use rocket_db_pools::Database;
use rocket::{Rocket, Build};

#[derive(Database)]
#[database("main")]
pub struct Db(sqlx::MySqlPool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(routes::stage())
        .attach(Db::init())
}

