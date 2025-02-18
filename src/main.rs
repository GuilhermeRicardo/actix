use actix_web::{web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};
use std::env;
use log::LevelFilter;
use env_logger::Env;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Inicializa o logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
    .filter_module("mongodb", LevelFilter::Warn)
    .init();

    // Conectando ao MongoDB
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL não está definida");
    let mongo_db = env::var("MONGO_DB").expect("MONGO_DB não está definida");
    let client_options = ClientOptions::parse(&mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(mongo_db.clone()))
            .configure(handlers::routes::config)
    })
    .workers(8) // Número de workers
    .bind("127.0.0.0:8081")?
    .run()
    .await
}