pub mod task;
pub mod user;
pub mod workflow;
pub mod workflow_task;
pub mod workflow_task_edge;
use core::fmt;

use actix_web::ResponseError;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiError {
    pub error: String,
}

impl ApiError {
    pub fn internal_server_error() -> Self {
        ApiError {
            error: String::from("Something went wrong. Please try again later."),
        }
    }
}

#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl ResponseError for ValidationError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let api_error = ApiError {
            error: format!("{}: {}", self.field, self.message),
        };
        actix_web::HttpResponse::BadRequest().json(api_error)
    }
}
