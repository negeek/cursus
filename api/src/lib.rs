pub mod db;
pub mod dto;
pub mod handler;
pub mod service;

use actix_web::{HttpResponse, Responder, get, web};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to CURSUS project!")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root).service(
        web::scope("/api/v1").service(web::scope("/account").service(handler::user::sign_up)), // .service(web::scope("/tasks").service(...))
                                                                                               // .service(web::scope("/workflows").service(...))
    );
}
