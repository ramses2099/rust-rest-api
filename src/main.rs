use actix_web::{ App, HttpServer };
use env_logger;
use dotenv::dotenv;

mod models;
mod services;
mod api;

use crate::api::helth_check;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // Server
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| { App::new().service(helth_check) })
        .bind(("127.0.0.1", 3000))?
        .run().await
}
