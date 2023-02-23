use actix_web::middleware::Logger;
use env_logger;
use actix_web::{get, web, App, HttpServer, Responder};


#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| 
            App::new()
            .wrap(Logger::default())
            .service(index)
            .service(hello)
        )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
