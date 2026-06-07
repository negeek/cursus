use crate::dto::error::ApiError;
use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr;
use std::fmt;

#[derive(Debug)]
pub enum WorkflowServiceError {
    DatabaseError(DbErr),
    WorkflowNotFound,
    Unauthorized,
    ServiceError,
    TaskNotFound,
}

impl fmt::Display for WorkflowServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowServiceError::DatabaseError(e) => write!(f, "Database error: {}", e),
            WorkflowServiceError::WorkflowNotFound => write!(f, "Workflow not found"),
            WorkflowServiceError::Unauthorized => {
                write!(f, "You do not have permission to access this workflow")
            }
            WorkflowServiceError::ServiceError => write!(f, "Service error"),
            WorkflowServiceError::TaskNotFound => write!(f, "Task not found"),
        }
    }
}

impl From<DbErr> for WorkflowServiceError {
    fn from(err: DbErr) -> Self {
        WorkflowServiceError::DatabaseError(err)
    }
}

impl ResponseError for WorkflowServiceError {
    fn error_response(&self) -> HttpResponse {
        let api_error = ApiError {
            error: self.to_string(),
        };
        match self {
            WorkflowServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ApiError::internal_server_error())
            }
            WorkflowServiceError::WorkflowNotFound => HttpResponse::NotFound().json(api_error),
            WorkflowServiceError::Unauthorized => HttpResponse::Forbidden().json(api_error),
            WorkflowServiceError::ServiceError => {
                HttpResponse::InternalServerError().json(api_error)
            }
            WorkflowServiceError::TaskNotFound => HttpResponse::NotFound().json(api_error),
        }
    }
}
