//! Application guards and virtual hosting
//!
//! [guards]: https://actix.rs/docs/application/
use actix_web::{guard, web, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("Hello www"))),
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("Hello user"))),
            )
            .route("/", web::to(|| HttpResponse::Ok().body("Hello world!")))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
