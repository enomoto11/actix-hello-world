use actix_web::{web, App, HttpServer};
mod config;
mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = config::Server::new().get_address();

    return HttpServer::new(|| {
        App::new()
            .service(controller::hello)
            .service(controller::echo)
            .route("/hey", web::get().to(controller::manual_hello))
    })
    .bind(server_address)?
    .run()
    .await;
}
