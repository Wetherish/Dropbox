use crate::database::connection::Database;

use crate::model::file::{
    File, FileUploadRequest, NewFolderRequest, ResponseFile, file_to_response_file,
    file_upload_request_to_file, files_to_response_files,
};
use crate::model::storage_client::BucketClient;
use crate::web::Json;
use actix_web::{HttpResponse, Responder, Result, get, post, web};
use std::str::FromStr;
use surrealdb::RecordId;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/open_directory/{dir_id}")]
pub async fn open_dir(path: web::Path<String>) -> Result<Json<Vec<ResponseFile>>> {
    let dir_id = path.into_inner();
    let res: Vec<File> = Database::get_dir_contents(&dir_id).await;
    Ok(Json(files_to_response_files(res)))
}

#[get("/get_dir/{dir_id}")]
pub async fn get(path: web::Path<String>) -> Result<Json<ResponseFile>> {
    let dir_id = path.into_inner();
    let back = Database::get_file(&dir_id).await;
    Ok(Json(file_to_response_file(back.unwrap())))
}

#[post("/directory/")]
pub async fn create_folder(req_file: web::Json<NewFolderRequest>) -> Result<String> {
    let file = req_file.into_inner();
    let new_dir = File {
        id: None,
        owner: RecordId::from_str(&file.owner_id).unwrap(),
        is_file: false,
        is_starred: false,
        name: file.name,
        parent: Some(RecordId::from_str(&file.parent_id).unwrap()),
        file_type: "dir".to_string(),
        size: 0,
        thumbnail_url: Some("asd".to_string()),
    };
    Database::create_file(new_dir).await;
    Ok("TODO ERROR HANDLING".to_string())
}

#[get("/directory/{user_id}")]
pub async fn get_users_file(path: web::Path<String>) -> Result<Json<Vec<File>>> {
    let result = Database::get_user_files(&path.into_inner()).await;
    Ok(Json(result))
}

#[get("/file_url/{id}")]
pub async fn get_file_url(
    bucket: web::Data<BucketClient>,
    id: web::Path<String>,
) -> Result<String> {
    let file_id = id.into_inner();
    let slice = &file_id[6..];
    let url = bucket
        .bucket
        .presign_get(&slice, 3600 / 2, None)
        .await
        .unwrap();
    Ok(url)
}

#[post("/upload_url")]
pub async fn get_file_upload_url(
    bucket: web::Data<BucketClient>,
    request_json: web::Json<FileUploadRequest>,
) -> Result<String> {
    let request = request_json.into_inner();
    let file = file_upload_request_to_file(request);
    let file = Database::create_file(file).await;
    let url = bucket
        .bucket
        .presign_put(
            file.unwrap().id.unwrap().key().to_string(),
            3600 / 2,
            None,
            None,
        )
        .await;
    match url {
        Ok(r_url) => Ok(r_url),
        Err(e) => {
            eprintln!("Error generating presigned URL: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(e))
        }
    }
}

#[post("/delete_file_url")]
pub async fn delete_file(
    file_id: web::Path<String>,
    bucket: web::Data<BucketClient>,
) -> Result<String> {
    let file_id = file_id.into_inner();
    let url = bucket.bucket.presign_delete(file_id, 3600 / 2).await;
    match url {
        Ok(url) => Ok(url),
        Err(e) => {
            eprintln!("Error generating presigned URL: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(e))
        }
    }
}
