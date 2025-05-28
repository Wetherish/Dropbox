use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<RecordId>,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseUser {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
}
