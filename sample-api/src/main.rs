use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
mod rest;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(rest::systems::index))
            .route("/healthz", web::get().to(rest::systems::healthz))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
