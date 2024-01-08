use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = server_config::new();

    return HttpServer::new(|| {
        App::new()
            .service(controller::hello)
            .service(controller::echo)
            .route("/hey", web::get().to(controller::manual_hello))
    })
    .bind((config.host, config.port))?
    .run()
    .await;
}

mod server_config {
    use std::env;
    use std::io::Error;
    use std::net::Ipv4Addr;

    pub struct ServerConfig {
        pub host: Ipv4Addr,
        pub port: u16,
    }

    pub fn new() -> ServerConfig {
        // FIXME: 上位環境ごとに設定を変える
        let localhost: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
        let port = get_port().unwrap();
        return ServerConfig {
            host: localhost,
            port,
        };
    }

    fn get_port() -> Result<u16, Error> {
        let port = env::var("PORT");
        match port {
            Ok(p) => Ok(p.parse::<u16>().unwrap()),
            Err(_) => Err(Error::new(std::io::ErrorKind::Other, "No port")),
        }
    }
}

mod controller {
    use actix_web::{get, post, HttpResponse, Responder};
    #[get("/")]
    pub async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[post("/echo")]
    pub async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    pub async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }
}
