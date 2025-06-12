use crate::{
    database::connection::Database,
    model::user::{User, UserRequest},
};

use actix_web::{Result, post, web};

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
