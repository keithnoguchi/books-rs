//! [hello] server
//!
//! [hellp]: https://actix.rs/docs/getting-started/
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world, again!")
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
