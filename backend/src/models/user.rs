use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserLoginWeb {
    pub password: String,
}

// NOTE: Currently the same as UserLoginWeb but with more sofisticated accounts a user
// will be more than a password and most likely more than what's needed for login
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct UserWeb {
    pub password: String,
}

pub struct UserDB {
    pub id: i32,
    pub password: String,
}
