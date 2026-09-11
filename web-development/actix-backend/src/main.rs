use actix_backend::routes::route;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(route()))
        .bind(("0.0.0.0", 4000))?
        .run()
        .await
}
