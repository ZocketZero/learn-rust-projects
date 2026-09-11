use crate::controllers::ProductController;
use actix_web::{Scope, web};

pub fn product_route() -> Scope {
    web::scope("/product")
        .route("/", web::get().to(ProductController::index))
        .route("/", web::post().to(ProductController::create_product))
}
