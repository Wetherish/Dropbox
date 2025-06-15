use crate::{
    database::connection::Database,
    model::user::{LoginRequest, User, UserRequest},
};

use actix_web::{get, post, web, Result};

#[post("/new_user/")]
pub async fn create_user(user_request: web::Json<UserRequest>) -> Result<String> {
    let root_id = Database::create_user(User::new(
        user_request.username.clone(),
        user_request.password.clone(),
        user_request.email.clone(),
    ))
    .await;
    dbg!(&user_request);
    Ok(String::from(root_id.unwrap().key().to_string()))
}

#[post("/login/")]
pub async fn get_user(user_request: web::Json<LoginRequest>) -> Result<String> {
    let user = Database::get_user(
        user_request.email.clone(),
        user_request.password.clone(),
    )
    .await;
    dbg!(&user_request);
    match user {
        Some(u) => Ok(String::from(u.id.unwrap().key().to_string())),
        None => Err(actix_web::error::ErrorNotFound("User not found")),
    }
}

#[get("/get_root_dir/{user_id}")]
pub async fn get_user_root_dir(path: web::Path<String>) -> Result<String> {
    let user_id = path.into_inner();
    dbg!(&user_id);
    let root_dir = Database::get_user_root_dir(format!("User:{}", user_id)).await;
    match root_dir {
        Some(dir) => Ok(String::from(dir.key().to_string())),
        None => Err(actix_web::error::ErrorNotFound("Root directory not found")),
    }
}