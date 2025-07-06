use crate::controller::configure_routes;
use crate::model::storage_client::BucketClient;
use actix_cors::Cors;
use actix_web::middleware::Logger;
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
    connect_db("ws://localhost:8000", "root", "root", "Dropbox", "FilesDB").await;
    let stoarge = BucketClient::new().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(stoarge.clone()))
            .wrap(create_cors_middleware())
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("TODO: panic message");
    Ok(())
}

fn create_cors_middleware() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}

async fn connect_db(path: &str, username: &str, password: &str, ns: &str, database: &str) {
    DB.connect(path).await.unwrap();
    DB.signin(Root { username, password })
        .await
        .expect("Wrong Password");
    DB.use_ns(ns).use_db(database).await.unwrap();
}
