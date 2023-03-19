use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::{Db, Result};


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Shortcut {
    id: i32,
    name: Option<String>,
    desc: Option<String>,
    link: Option<String>,
    icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(crate = "rocket::serde")]
struct ShortcutUpdateable{
    id: Option<i32>,
    name: Option<String>,
    desc: Option<String>,
    link: Option<String>,
    icon: Option<String>,
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_all_shortcuts, get_one_shortcut, update_shortcut, create_shortcut, delete_shortcut]
}

#[get("/shortcut")]
async fn get_all_shortcuts(mut db: Connection<Db>) -> Result<Json<Vec<Shortcut>>> {
    let shortcuts = sqlx::query_as!(Shortcut, "SELECT * FROM shortcuts")
        .fetch_all(&mut *db)
        .await?;
        
    Ok(Json(shortcuts))
}

#[get("/shortcut/<id>")]
async fn get_one_shortcut(mut db: Connection<Db>, id: i32) -> Result<Json<Shortcut>> {
    let shortcut = sqlx::query_as!(Shortcut, "SELECT * FROM shortcuts where id= ? ", id)
        .fetch_one(&mut *db)
        .await?;
        
    Ok(Json(shortcut))
}

#[put("/shortcut", format = "json", data = "<data>")]
async fn update_shortcut(mut db: Connection<Db>, data: Json<ShortcutUpdateable>) -> Result<Json<i32>> {
    match data.id {
        | Some(id) => { 
            let shortcut = sqlx::query_as!(Shortcut, "SELECT * FROM shortcuts where id= ? ", id)
                .fetch_one(&mut *db)
                .await?;

            let affected_rows = sqlx::query!("UPDATE shortcuts SET `name` = ?, `desc` = ?, `link` = ?, `icon` = ? where `id` = ?",
                data.name.as_ref().or(shortcut.name.as_ref()).unwrap(),
                data.desc.as_ref().or(shortcut.desc.as_ref()).unwrap(),
                data.link.as_ref().or(shortcut.link.as_ref()).unwrap(),
                data.icon.as_ref().or(shortcut.icon.as_ref()).unwrap(),
                data.id)
                .execute(&mut *db).await?.rows_affected();
            return Ok(Json(affected_rows.try_into().unwrap()))
        },
        | None => {
            return create_shortcut(db, data).await
        },
    }

}

#[post("/shortcut", format = "json", data = "<data>")]
async fn create_shortcut(mut db: Connection<Db>, data: Json<ShortcutUpdateable>) -> Result<Json<i32>> {
    match data.name.as_ref().and(data.desc.as_ref()).and(data.link.as_ref()).and(data.icon.as_ref()) { 
        | None => { 
            // TODO Respond with error 
            return Ok(Json(-1)) 
        },
        | Some(_) =>  {
            let query_result = sqlx::query!("INSERT INTO shortcuts (`name`, `desc`, `link`, `icon`) VALUES (?, ?, ?, ?)",
                data.name.as_ref().unwrap(),
                data.desc.as_ref().unwrap(),
                data.link.as_ref().unwrap(),
                data.icon.as_ref().unwrap())
                .execute(&mut *db).await?;

            // todo return newly created shortcuts ID instead
            return Ok(Json(query_result.last_insert_id().try_into().unwrap()))
        
        },
    }
}

#[delete("/shortcut/<id>")]
async fn delete_shortcut(mut db: Connection<Db>, id: i32) -> Result<Json<i32>> {
    let affected_rows = sqlx::query!("DELETE FROM shortcuts WHERE `id` = ?", id).execute(&mut *db).await?.rows_affected();

    Ok(Json(affected_rows.try_into().unwrap()))


}

#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    
    #[test]
    fn create_update_and_remove_shortcut() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
       
        // Create a shortcut in DB
        //////////////////////////
        let mut expected_shortcut = super::ShortcutUpdateable{
            id: None,
            name: Some("TestName".to_string()),
            desc: Some("Test description for shortcut".to_string()),
            link: Some("https://testurl.com".to_string()),
            icon: Some("mdi-search".to_string()),
        };

        let create_response = client.post(uri!("/api/v2/", super::create_shortcut)).json(&expected_shortcut).dispatch();
        assert_eq!(create_response.status(), Status::Ok);
        
        let created_shortcuts_id: Option<i32> = create_response.into_json();
        assert_ne!(created_shortcuts_id, None);
        //
        // update expected_shortcut to reflect DB
        expected_shortcut.id = created_shortcuts_id;


        // GET newly created shortcut from DB
        /////////////////////////////////////
        let get_one_uri = uri!("/api/v2/", super::get_one_shortcut(id = created_shortcuts_id.unwrap()));
        let get_initial_response = client.get(get_one_uri).dispatch();
        assert_eq!(get_initial_response.status(), Status::Ok);

        let initial_shortcut: Option<super::ShortcutUpdateable> = get_initial_response.into_json();
        assert_eq!(initial_shortcut.unwrap(), expected_shortcut);

        
        // Update shortcut, partially
        /////////////////////////////
        let partial_update_shortcut = super::ShortcutUpdateable {
            id: expected_shortcut.id,
            name: Some("UpdatedTestName".to_string()),
            desc: None,
            link: None,
            icon: None,
        };
        
        // update expected_shortcut to reflect DB
        expected_shortcut.name = partial_update_shortcut.name.clone();


        let update_response = client.put(uri!("/api/v2/", super::update_shortcut)).json(&partial_update_shortcut).dispatch();
        assert_eq!(update_response.status(), Status::Ok);
        assert_eq!(update_response.into_json(), Some(1));

        // Check that the correct object was updated
        let get_one_updated_uri = uri!("/api/v2/", super::get_one_shortcut(id = created_shortcuts_id.unwrap()));
        let get_updated_response = client.get(get_one_updated_uri).dispatch();
        assert_eq!(get_updated_response.status(), Status::Ok);

        let updated_shortcut: Option<super::ShortcutUpdateable> = get_updated_response.into_json();
        assert_eq!(updated_shortcut.unwrap(), expected_shortcut);

        // Update shortcut, all attributes
        //////////////////////////////////
        let partial_update_shortcut = super::ShortcutUpdateable {
            id: expected_shortcut.id,
            name: None,
            desc: Some("UpdatedDesc".to_string()),
            link: Some("https://dtek.link/glimfjord/updated".to_string()),
            icon: Some("mdi-update".to_string()),
        };
        
        // update expected_shortcut to reflect DB
        expected_shortcut.desc = partial_update_shortcut.desc.clone();
        expected_shortcut.link = partial_update_shortcut.link.clone();
        expected_shortcut.icon = partial_update_shortcut.icon.clone();


        let full_update_response = client.put(uri!("/api/v2/", super::update_shortcut)).json(&partial_update_shortcut).dispatch();
        assert_eq!(full_update_response.status(), Status::Ok);
        assert_eq!(full_update_response.into_json(), Some(1));

        // Check that the correct object was updated
        let get_one_fully_updated_uri = uri!("/api/v2/", super::get_one_shortcut(id = created_shortcuts_id.unwrap()));
        let get_updated_response = client.get(get_one_fully_updated_uri).dispatch();
        assert_eq!(get_updated_response.status(), Status::Ok);

        let updated_shortcut: Option<super::ShortcutUpdateable> = get_updated_response.into_json();
        assert_eq!(updated_shortcut.unwrap(), expected_shortcut);

        
        // Delete shortcut
        //////////////////
        let delete_uri = uri!("/api/v2/", super::get_one_shortcut(id = created_shortcuts_id.unwrap()));
        let delete_response = client.delete(delete_uri).dispatch();
        assert_eq!(delete_response.status(), Status::Ok);
        assert_eq!(delete_response.into_json(), Some(1));

        // Make sure object is deleted
        //////////////////////////////
        // Check that the correct object was updated
        let get_one_deleted_uri = uri!("/api/v2/", super::get_one_shortcut(id = created_shortcuts_id.unwrap()));
        let get_deleted_response = client.get(get_one_deleted_uri).dispatch();
        assert_eq!(get_deleted_response.status(), Status::InternalServerError);
    }
}
