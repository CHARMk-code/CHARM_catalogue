#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_db_pools;
mod shortcuts;


// TODO: Migrate to postgres db
//
#[launch]
fn rocket() -> _ {
    rocket::build().attach(shortcuts::stage())
}
