use rocket::fairing::AdHoc;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{sqlx, Database, Connection};

#[derive(Database)]
#[database("sqlx")]
struct Db(sqlx::MySqlPool);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Shortcut {
    id: Option<i64>,
    name: String,
    desc: String,
    link: String,
    icon: String,
}


type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/")]
async fn get_shortcuts(mut db: Connection<Db>) -> Result<Json<Vec<Shortcut>>> {
    let shortcuts = sqlx::query!("SELECT * FROM shortcuts")
        .fetch(&mut *db)
        .map_ok(|record| record)
        .try_collect::<Vec<Result<Json<Vec<Shortcut>>>>>()
        .await?;

    Ok(Json(shortcuts))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Shortcut Stage", |rocket| async {
        rocket.attach(Db::init())
            .mount("/shortcuts", routes![get_shortcuts])
    })
}
