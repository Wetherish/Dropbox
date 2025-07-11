use std::str::FromStr;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub id: Option<RecordId>,
    pub is_file: bool,
    pub is_starred: bool,
    pub name: String,
    pub owner: RecordId,
    pub parent: Option<RecordId>,
    pub size: i64,
    pub thumbnail_url: Option<String>,
    pub file_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResponseFile {
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
pub struct FileUploadRequest {
    pub name: String,
    pub owner: String,
    pub parent: String,
    pub size: i64,
    pub file_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewFolderRequest {
    pub parent_id: String,
    pub owner_id: String,
    pub name: String,
}

pub fn file_to_response_file(file: File) -> ResponseFile {
    ResponseFile {
        id: file.id.unwrap().to_string(),
        is_file: file.is_file,
        is_starred: file.is_starred,
        name: file.name,
        owner: file.owner.to_string(),
        parent: file.parent.map(|p| p.to_string()),
        size: file.size,
        thumbnail_url: file.thumbnail_url,
        file_type: file.file_type,
    }
}

pub fn files_to_response_files(files: Vec<File>) -> Vec<ResponseFile> {
    files.into_iter().map(file_to_response_file).collect()
}

pub fn file_upload_request_to_file(request: FileUploadRequest) -> File {
    File {
        id: None,
        is_file: true,
        name: request.name,
        file_type: request.file_type,
        parent: Some(RecordId::from_str(&request.parent).unwrap()),
        owner: RecordId::from_str(&request.owner).unwrap(),
        size: request.size,
        is_starred: false,
        thumbnail_url: None,
    }
}
