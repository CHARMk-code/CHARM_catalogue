use rocket::fairing::AdHoc;

pub mod shortcut;


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Routes stage", |rocket| async {
//        rocket.mount("/api/v2", routes![index])
        rocket.mount("/api/v2/", shortcut::routes()) 
    } )
}


