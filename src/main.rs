use actix_web::{ get, web, App, HttpServer, Responder };
use env_logger;
use dotenv::dotenv;

// Router
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    // Server
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| { App::new().service(greet) })
        .bind(("127.0.0.1", 3000))?
        .run().await
}
