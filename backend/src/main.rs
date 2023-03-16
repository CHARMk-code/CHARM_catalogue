#[macro_use]
extern crate rocket;

use rocket_db_pools::sqlx;
use rocket_db_pools::Database;
use rocket_db_pools::Connection;
use rocket::futures::TryStreamExt;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Database)]
#[database("main")]
struct Db(sqlx::MySqlPool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Shortcut {
    id: i32,
    name: Option<String>,
    desc: Option<String>,
    link: Option<String>,
    icon: Option<String>,
}

#[get("/")]
async fn get_shortcuts(mut db: Connection<Db>) -> Result<Json<Vec<Shortcut>>> {
    let shortcuts = sqlx::query_as!(Shortcut, "SELECT * FROM shortcuts")
        .fetch_all(&mut *db)
        .await?;
        
    Ok(Json(shortcuts))
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/api", routes![get_shortcuts])
        .attach(Db::init())
        .launch()
        .await;
}
