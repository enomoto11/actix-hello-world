use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use std::io::Error;
use std::net::Ipv4Addr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let port = get_port().unwrap();

    return HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((localhost, port))?
    .run()
    .await;
}

fn get_port() -> Result<u16, Error> {
    let port = env::var("PORT");
    match port {
        Ok(p) => Ok(p.parse::<u16>().unwrap()),
        Err(_) => Err(Error::new(std::io::ErrorKind::Other, "No port")),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
