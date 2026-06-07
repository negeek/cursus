use crate::dto::error::ApiError;
use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr;
use std::fmt;

#[derive(Debug)]
pub enum TaskServiceError {
    DatabaseError(DbErr),
    TaskNotFound,
    ServiceError,
    Unauthorized,
}

impl fmt::Display for TaskServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskServiceError::DatabaseError(e) => write!(f, "Database error: {}", e),
            TaskServiceError::TaskNotFound => write!(f, "Task not found"),
            TaskServiceError::ServiceError => write!(f, "Service error"),
            TaskServiceError::Unauthorized => write!(
                f,
                "You cannot perform this action on a task that does not belong to you"
            ),
        }
    }
}

impl From<DbErr> for TaskServiceError {
    fn from(err: DbErr) -> Self {
        TaskServiceError::DatabaseError(err)
    }
}

impl ResponseError for TaskServiceError {
    fn error_response(&self) -> HttpResponse {
        let api_error = ApiError {
            error: self.to_string(),
        };
        match self {
            TaskServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ApiError::internal_server_error())
            }
            TaskServiceError::TaskNotFound => HttpResponse::NotFound().json(api_error),
            TaskServiceError::ServiceError => HttpResponse::InternalServerError().json(api_error),
            TaskServiceError::Unauthorized => HttpResponse::Forbidden().json(api_error),
        }
    }
}
