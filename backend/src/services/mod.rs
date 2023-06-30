pub mod shortcut;
pub mod tag;
pub mod map;
pub mod layout;
pub mod prepage;
pub mod feedback;
pub mod company;

pub fn is_valid_required_field<T>(val: &Option<T>) -> Result<&T, actix_web::Error> {
    match val.as_ref() {
        None => Err(actix_web::error::ErrorUnprocessableEntity("Missing required field")),
        Some(v) => Ok(v)
    }
}

