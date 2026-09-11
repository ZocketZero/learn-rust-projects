use crate::routes::{product::product_route, user::user_route};
use actix_web::{HttpResponse, Responder, Scope, get, web};
pub fn route() -> Scope {
    web::scope("")
        .service(hello)
        .service(test_web)
        .service(user_route())
        .service(product_route())
}

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world"
}

#[get("/test")]
async fn test_web() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
