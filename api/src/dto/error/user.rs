use crate::dto::error::ApiError;
use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr;
use std::fmt;

#[derive(Debug)]
pub enum UserServiceError {
    EmailAlreadyExists,
    HashingFailed,
    DatabaseError(DbErr),
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserServiceError::EmailAlreadyExists => write!(f, "Email already exists"),
            UserServiceError::HashingFailed => write!(f, "Password hashing failed"),
            UserServiceError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl From<DbErr> for UserServiceError {
    fn from(err: DbErr) -> Self {
        UserServiceError::DatabaseError(err)
    }
}

impl ResponseError for UserServiceError {
    fn error_response(&self) -> HttpResponse {
        let api_error = ApiError {
            error: self.to_string(),
        };
        match self {
            UserServiceError::EmailAlreadyExists => HttpResponse::Conflict().json(api_error),
            UserServiceError::HashingFailed => {
                HttpResponse::InternalServerError().json(ApiError::internal_server_error())
            }
            UserServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ApiError::internal_server_error())
            }
        }
    }
}
