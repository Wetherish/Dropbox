use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<RecordId>,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl User {
    pub fn new(username: String, password: String, email: String) -> User {
        User {
            id: None,
            username,
            password,
            email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseUser {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}
