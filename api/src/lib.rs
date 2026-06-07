pub mod db;
pub mod dto;
pub mod handler;
pub mod middleware;
pub mod service;
pub mod util;

use actix_web::{HttpResponse, Responder, get, web};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to CURSUS project!")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root).service(
        web::scope("/api/v1")
            .service(
                web::scope("/account")
                    .service(handler::user::sign_up)
                    .service(handler::user::signin)
                    .service(handler::user::verify_email)
                    .service(
                        web::resource("/logout")
                            .wrap(middleware::user::Auth)
                            .route(web::post().to(handler::user::logout)),
                    )
                    .service(handler::user::refresh_access_token),
            )
            .service(
                web::scope("")
                    .wrap(middleware::user::Auth)
                    .service(handler::task::get_tasks)
                    .service(handler::task::create_task)
                    .service(handler::task::get_task)
                    .service(handler::task::edit_task)
                    .service(handler::task::delete_task)
                    .service(handler::workflow::get_workflows)
                    .service(handler::workflow::create_workflow)
                    .service(handler::workflow::get_workflow_detail)
                    .service(handler::workflow::get_workflow)
                    .service(handler::workflow::edit_workflow)
                    .service(handler::workflow::delete_workflow)
                    .service(handler::workflow_task::create_workflow_task)
                    .service(handler::workflow_task::get_workflow_task)
                    .service(handler::workflow_task::edit_workflow_task)
                    .service(handler::workflow_task::delete_workflow_task)
                    .service(handler::workflow_task_edge::create_workflow_task_edge)
                    .service(handler::workflow_task_edge::edit_workflow_task_edge)
                    .service(handler::workflow_task_edge::delete_workflow_task_edge),
            ),
    );
}
