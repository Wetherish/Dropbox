use crate::database::connection::Database;
use crate::model::file::{
    File, NewFolderRequest, ResponseFile, file_to_response_file, files_to_response_files,
};
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
    dbg!(&dir_id);
    let res: Vec<File> = Database::get_dir_contents(&dir_id).await;
    dbg!(res.len());
    Ok(Json(files_to_response_files(res)))
}

#[get("/get_dir/{dir_id}")]
pub async fn get(path: web::Path<String>) -> Result<Json<ResponseFile>> {
    let dir_id = path.into_inner();
    dbg!(&dir_id);
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
