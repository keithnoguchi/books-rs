//! [actix-web] [basic] server
//!
//! [actix-web]: https://lib.rs/actix-web
//! [basic]: https://github.com/actix/examples/tree/master/basics/
use actix_web::{http::header, web, App, HttpRequest, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new().service(web::resource("/").route(web::get().to(|req: HttpRequest| {
            println!("{:?}", req);
            HttpResponse::Found()
                .header(header::LOCATION, "static/welcome.html")
                .finish()
        })))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
