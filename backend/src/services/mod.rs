pub mod auth;
pub mod company;
pub mod feedback;
pub mod layout;
pub mod map;
pub mod prepage;
pub mod settings;
pub mod shortcut;
pub mod tag;
pub mod batch;
pub mod file;
pub mod image;

pub fn is_valid_required_field<T: Clone>(val: &Option<T>) -> Result<T, actix_web::Error> {
    match val.as_ref() {
        None => Err(actix_web::error::ErrorUnprocessableEntity(
            "Missing required field",
        )),
        Some(v) => Ok(v.clone()),
    }
}

pub fn is_optional_field_or_default<T: Clone>(val: &Option<T>, default: T) -> Result<T, actix_web::Error> {
match val.as_ref() {
        None => Ok(default),
        Some(v) => Ok(v.clone()),
    }
}
