use crate::types::User;
use actix_web::{HttpResponse, Responder};

pub struct UserController {}

impl UserController {
    pub async fn get_user() -> impl Responder {
        let users = vec![User {
            name: "Nawasan".to_string(),
            age: 20,
        }];
        HttpResponse::Ok().json(users)
    }
}
