use serde::{Deserialize, Serialize};
use crate::model::RecordID::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    pub id: Option<RecordId>,
    pub username: String,
    pub password: String,
    pub email: String,
}