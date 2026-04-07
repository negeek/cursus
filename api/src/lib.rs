pub mod db;

use actix_web::{HttpResponse, Responder, get, web};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to CURSUS project!")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root);
}
