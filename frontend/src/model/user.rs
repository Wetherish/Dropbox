use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl NewUserRequest {
    pub fn new() -> NewUserRequest {
        return NewUserRequest {
            username: String::from(""),
            password: String::from(""),
            email: String::from(""),
        };
    }
}
