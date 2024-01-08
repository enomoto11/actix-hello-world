use actix_web::{web, App, HttpServer};
mod config;
mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Server::new();

    return HttpServer::new(|| {
        App::new()
            .service(controller::hello)
            .service(controller::echo)
            .route("/hey", web::get().to(controller::manual_hello))
    })
    .bind((config.get_host(), config.get_port()))?
    .run()
    .await;
}
