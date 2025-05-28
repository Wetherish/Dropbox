use std::sync::LazyLock;
use crate::controller::file_controller::{create_folder, get_users_file, hello, open_dir};
use actix_web::{web, App, HttpServer};
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use actix_cors::Cors;
use actix_web::http::header;

mod model;
mod database;
mod controller;
static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    DB.connect("ws://localhost:8000").await.unwrap();
    DB.signin(Root {
        username: "root",
        password: "root",
    }).await.expect("Wrong Password");
    DB.use_ns("Dropbox").use_db("FilesDB").await.unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
            )
            .service(hello)
            .service(create_folder)
            .service(get_users_file)
            .service(open_dir)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await.expect("TODO: panic message");
    Ok(())


}
