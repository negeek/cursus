use crate::dto::error::ApiError;
use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr;
use std::fmt;

#[derive(Debug)]
pub enum WorkflowTaskEdgeServiceError {
    DatabaseError(DbErr),
    EdgeNotFound,
    InvalidStep,
    DuplicateEdge,
    SelfLoop,
    WorkflowNotFound,
    WorkflowNotOwned,
    ServiceError,
    WorkflowTaskNotFound,
}

impl fmt::Display for WorkflowTaskEdgeServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowTaskEdgeServiceError::DatabaseError(e) => write!(f, "Database error: {}", e),
            WorkflowTaskEdgeServiceError::EdgeNotFound => write!(f, "Edge not found"),
            WorkflowTaskEdgeServiceError::InvalidStep => {
                write!(f, "One or both steps do not exist in this workflow")
            }
            WorkflowTaskEdgeServiceError::DuplicateEdge => {
                write!(f, "Edge already exists between these steps")
            }
            WorkflowTaskEdgeServiceError::SelfLoop => {
                write!(f, "A task cannot have an edge to itself")
            }
            WorkflowTaskEdgeServiceError::WorkflowNotFound => {
                write!(f, "Workflow not found")
            }
            WorkflowTaskEdgeServiceError::WorkflowNotOwned => {
                write!(f, "You do not own this workflow")
            }
            WorkflowTaskEdgeServiceError::ServiceError => {
                write!(f, "An unexpected error occurred. Please try again later.")
            }
            WorkflowTaskEdgeServiceError::WorkflowTaskNotFound => {
                write!(f, "Workflow task not found")
            }
        }
    }
}

impl From<DbErr> for WorkflowTaskEdgeServiceError {
    fn from(err: DbErr) -> Self {
        WorkflowTaskEdgeServiceError::DatabaseError(err)
    }
}

impl ResponseError for WorkflowTaskEdgeServiceError {
    fn error_response(&self) -> HttpResponse {
        let api_error = ApiError {
            error: self.to_string(),
        };
        match self {
            WorkflowTaskEdgeServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ApiError::internal_server_error())
            }
            WorkflowTaskEdgeServiceError::EdgeNotFound => HttpResponse::NotFound().json(api_error),
            WorkflowTaskEdgeServiceError::InvalidStep => HttpResponse::BadRequest().json(api_error),
            WorkflowTaskEdgeServiceError::DuplicateEdge => HttpResponse::Conflict().json(api_error),
            WorkflowTaskEdgeServiceError::SelfLoop => HttpResponse::BadRequest().json(api_error),
            WorkflowTaskEdgeServiceError::WorkflowNotFound => {
                HttpResponse::NotFound().json(api_error)
            }
            WorkflowTaskEdgeServiceError::WorkflowNotOwned => {
                HttpResponse::Forbidden().json(api_error)
            }
            WorkflowTaskEdgeServiceError::ServiceError => {
                HttpResponse::InternalServerError().json(api_error)
            }
            WorkflowTaskEdgeServiceError::WorkflowTaskNotFound => {
                HttpResponse::NotFound().json(api_error)
            }
        }
    }
}
