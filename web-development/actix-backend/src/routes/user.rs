use actix_web::{Scope, web};

use crate::controllers::UserController;

pub fn user_route() -> Scope {
    web::scope("/user").route("/", web::get().to(UserController::get_user))
}
