pub mod file_controller;
pub mod user_controller;
use crate::controller::file_controller::{
    create_folder, get, get_file_upload_url, get_file_url, get_users_file, hello, open_dir,
};
use crate::controller::user_controller::{create_user, get_user, get_user_root_dir};
use crate::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(create_folder)
        .service(create_user)
        .service(get_users_file)
        .service(open_dir)
        .service(get)
        .service(get_user)
        .service(get_user_root_dir)
        .service(get_file_url)
        .service(get_file_upload_url);
}
