//! Application [state]
//!
//! [state]: https://actix.rs/docs/application/
use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

struct AppState {
    app_name: String,
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}
