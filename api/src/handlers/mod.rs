pub mod errors;
pub mod task;
pub mod user;
pub mod workflow;
pub mod workflow_task;
pub mod workflow_task_edge;

use actix_web::{HttpResponse, Responder, get};
use serde_json::json;

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Welcome to CURSUS project!",
        "api_docs": "/docs",
        "openapi_spec": "/api-docs/openapi.json"
    }))
}
