use crate::controller::file_controller::{create_folder, get_users_file, hello, open_dir};
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use std::sync::LazyLock;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;

mod controller;
mod database;
mod model;
static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    DB.connect("ws://localhost:8000").await.unwrap();
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Wrong Password");
    DB.use_ns("Dropbox").use_db("FilesDB").await.unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(hello)
            .service(create_folder)
            .service(get_users_file)
            .service(open_dir)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("TODO: panic message");
    Ok(())
}
