use actix_web::{App, HttpServer};
mod controllers;
mod models;
mod repositories;
mod services;
mod schema;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::user_routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
