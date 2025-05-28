use dioxus::prelude::server_fn::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub id: Option<String>,
    pub is_file: bool,
    pub is_starred: bool,
    pub name: String,
    pub owner: String,
    pub parent: Option<String>,
    pub size: i64,
    pub thumbnail_url: Option<String>,
    pub file_type: String,
}