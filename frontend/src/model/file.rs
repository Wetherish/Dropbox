use dioxus::prelude::server_fn::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct File {
    pub id: String,
    pub is_file: bool,
    pub is_starred: bool,
    pub name: String,
    pub owner: String,
    pub parent: Option<String>,
    pub size: i64,
    pub thumbnail_url: Option<String>,
    pub file_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewFolderRequest {
    pub parent_id: String,
    pub owner_id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UploadRequestFile {
    pub id_user: String,
    pub name: String,
    pub owner: String,
    pub parent: String,
    pub size: i64,
    pub file_type: String,
}
