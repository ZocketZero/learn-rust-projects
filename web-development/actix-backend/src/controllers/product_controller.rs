use crate::types::{Product, ProductInput, ProductList};
use actix_web::{HttpResponse, Responder, web};
use std::sync::Mutex;

pub struct ProductController {}

static PRODUCT_LIST: Mutex<ProductList> = Mutex::new(Vec::new());

impl ProductController {
    /// Get product list
    pub async fn index() -> impl Responder {
        let product_list = PRODUCT_LIST.lock().unwrap();
        HttpResponse::Ok().json(&*product_list)
    }

    /// Example: Get request's body by web::Json<T>
    pub async fn create_product(product: web::Json<ProductInput>) -> impl Responder {
        let mut product_list = PRODUCT_LIST.lock().unwrap();
        let id = product_list.len() as u32;
        product_list.push(Product::new(id, &product.name, product.price));
        HttpResponse::Ok()
    }
}
