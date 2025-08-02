use crate::{
    database::connection::Database,
    model::user::{LoginRequest, ResponseUser, User, UserRequest},
};

use actix_web::{
    Result, get, post,
    web::{self, Json},
};

#[post("/new_user/")]
pub async fn create_user(user_request: web::Json<UserRequest>) -> Result<String> {
    let root_id = Database::create_user(User::new(
        user_request.username.clone(),
        user_request.password.clone(),
        user_request.email.clone(),
    ))
    .await;
    Ok(root_id.unwrap().key().to_string())
}

#[post("/login/")]
pub async fn get_user(user_request: web::Json<LoginRequest>) -> Result<Json<ResponseUser>> {
    let user = Database::get_user(user_request.email.clone(), user_request.password.clone()).await;
    match user {
        Some(u) => {
            let id = u.clone().id.unwrap().key().to_string();
            let root_id = Database::get_user_root_dir(format!("User:{}", id.clone()))
                .await
                .unwrap()
                .key()
                .to_string();
            let a = Json(ResponseUser {
                id: id,
                root_id: root_id,
            });
            Ok(a)
        }

        None => Err(actix_web::error::ErrorNotFound("User not found")),
    }
}

#[get("/get_root_dir/{user_id}")]
pub async fn get_user_root_dir(path: web::Path<String>) -> Result<String> {
    let user_id = path.into_inner();
    let root_dir = Database::get_user_root_dir(format!("User:{}", user_id)).await;
    match root_dir {
        Some(dir) => Ok(dir.key().to_string()),
        None => Err(actix_web::error::ErrorNotFound("Root directory not found")),
    }
}
