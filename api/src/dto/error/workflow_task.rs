use crate::dto::error::ApiError;
use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr;
use std::fmt;

#[derive(Debug)]
pub enum WorkflowTaskServiceError {
    DatabaseError(DbErr),
    WorkflowTaskNotFound,
    WorkflowNotFound,
    WorkflowTaskNotOwned,
    DuplicateStepName,
    TaskNotFound,
    TaskNotOwned,
    WorkflowNotOwned,
    ServiceError, // Generic error for unexpected cases
}

impl fmt::Display for WorkflowTaskServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowTaskServiceError::DatabaseError(e) => write!(f, "Database error: {}", e),
            WorkflowTaskServiceError::WorkflowTaskNotFound => {
                write!(f, "Workflow task not found")
            }
            WorkflowTaskServiceError::WorkflowTaskNotOwned => {
                write!(f, "You do not own this workflow task")
            }
            WorkflowTaskServiceError::DuplicateStepName => {
                write!(f, "Step name already exists in this workflow")
            }
            WorkflowTaskServiceError::ServiceError => {
                write!(f, "Service error")
            }
            WorkflowTaskServiceError::WorkflowNotFound => {
                write!(f, "Workflow not found")
            }
            WorkflowTaskServiceError::TaskNotFound => {
                write!(f, "Task not found")
            }
            WorkflowTaskServiceError::TaskNotOwned => {
                write!(f, "You do not own this task")
            }
            WorkflowTaskServiceError::WorkflowNotOwned => {
                write!(f, "You do not own this workflow")
            }
        }
    }
}

impl From<DbErr> for WorkflowTaskServiceError {
    fn from(err: DbErr) -> Self {
        WorkflowTaskServiceError::DatabaseError(err)
    }
}

impl ResponseError for WorkflowTaskServiceError {
    fn error_response(&self) -> HttpResponse {
        let api_error = ApiError {
            error: self.to_string(),
        };
        match self {
            WorkflowTaskServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ApiError::internal_server_error())
            }
            WorkflowTaskServiceError::WorkflowTaskNotFound => {
                HttpResponse::NotFound().json(api_error)
            }
            WorkflowTaskServiceError::WorkflowTaskNotOwned => {
                HttpResponse::Forbidden().json(api_error)
            }
            WorkflowTaskServiceError::DuplicateStepName => HttpResponse::Conflict().json(api_error),
            WorkflowTaskServiceError::ServiceError => {
                HttpResponse::InternalServerError().json(api_error)
            }
            WorkflowTaskServiceError::WorkflowNotFound => HttpResponse::NotFound().json(api_error),
            WorkflowTaskServiceError::TaskNotFound => HttpResponse::NotFound().json(api_error),
            WorkflowTaskServiceError::TaskNotOwned => HttpResponse::Forbidden().json(api_error),
            WorkflowTaskServiceError::WorkflowNotOwned => HttpResponse::Forbidden().json(api_error),
        }
    }
}
