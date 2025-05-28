use crate::web::Json;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use surrealdb::RecordId;
use crate::database::connection::Database;
use crate::model::file::{files_to_response_files, File, ResponseFile};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/directoryy/{dir_id}")]
pub async fn open_dir(path: web::Path<String>) -> Result<Json<Vec<ResponseFile>>> {
    let dir_id = path.into_inner();
    let res= Database::get_dir_contents(&dir_id).await;
    Ok(Json(files_to_response_files(res)))
}

#[post("/directory/")]
pub async fn create_folder(file: web::Json<File>) -> Result<String> {
    Database::create_file(file.into_inner()).await;
    Ok("TODO ERROR HANDLING".to_string())
}

#[get("/directory/{user_id}")]
pub async fn get_users_file(path: web::Path<String>) -> Result<Json<Vec<File>>> {
    let result =Database::get_user_files(&path.into_inner()).await;
    Ok(Json(result))
}