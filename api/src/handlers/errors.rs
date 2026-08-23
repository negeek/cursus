//! Where domain errors become HTTP responses.
//!
//! The error enums in `crate::dtos::error` know nothing about HTTP, so a service
//! can be called from the runner or a background job. This module holds the
//! mapping to status codes.
//!
//! A database failure never reaches the caller in detail. Its text can leak
//! schema and query shape, so it goes to the log line and the caller gets a flat
//! internal error.

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::dtos::error::ApiError;
use crate::dtos::error::task::TaskServiceError;
use crate::dtos::error::user::UserServiceError;
use crate::dtos::error::workflow::WorkflowServiceError;
use crate::dtos::error::workflow_task::WorkflowTaskServiceError;
use crate::dtos::error::workflow_task_edge::WorkflowTaskEdgeServiceError;
use crate::middlewares::request_event::{add_context, context_key};

/// Builds the response for an error that is the caller's to fix, so the message
/// is safe and useful to show them.
///
/// The same message goes onto the log line. Without it a 4xx says only that
/// something was refused, not which rule refused it.
fn client_error(status: StatusCode, message: String) -> HttpResponse {
    add_context(
        context_key::ERROR,
        serde_json::json!({ "reason": message.clone() }),
    );
    HttpResponse::build(status).json(ApiError { error: message })
}

/// Builds the response for a failure on our side. The cause goes onto the
/// request's log line, never into the response.
fn internal_error(error: &dyn std::fmt::Display) -> HttpResponse {
    add_context(
        context_key::ERROR,
        serde_json::json!({ "cause": error.to_string() }),
    );
    HttpResponse::InternalServerError().json(ApiError::internal_server_error())
}

impl ResponseError for TaskServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::TaskNotFound => client_error(StatusCode::NOT_FOUND, self.to_string()),
            Self::NotTaskOwner => client_error(StatusCode::FORBIDDEN, self.to_string()),
            Self::InvalidTaskId(_) | Self::InvalidUserId(_) | Self::InvalidJsonField { .. } => {
                client_error(StatusCode::BAD_REQUEST, self.to_string())
            }
            Self::Database(e) => internal_error(e),
        }
    }
}

impl ResponseError for UserServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::EmailAlreadyExists => client_error(StatusCode::CONFLICT, self.to_string()),

            // Everything an unauthenticated caller can trigger by guessing
            // answers with 401 and the same shape, so nothing here helps
            // distinguish a real account from a fake one.
            Self::InvalidCredentials
            | Self::InvalidVerificationCode
            | Self::VerificationCodeExpired
            | Self::MalformedToken
            | Self::RejectedToken => client_error(StatusCode::UNAUTHORIZED, self.to_string()),

            Self::EmailNotVerified => client_error(StatusCode::FORBIDDEN, self.to_string()),
            Self::UserNotFound => client_error(StatusCode::NOT_FOUND, self.to_string()),
            Self::InvalidUserId(_) => client_error(StatusCode::BAD_REQUEST, self.to_string()),

            // Nothing the caller did wrong, and nothing they can act on.
            Self::HashingFailed | Self::TokenIssuanceFailed | Self::MissingConfiguration(_) => {
                internal_error(self)
            }
            Self::Database(e) => internal_error(e),
        }
    }
}

impl ResponseError for WorkflowServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::WorkflowNotFound | Self::TaskNotFound => {
                client_error(StatusCode::NOT_FOUND, self.to_string())
            }
            Self::NotWorkflowOwner => client_error(StatusCode::FORBIDDEN, self.to_string()),
            Self::InvalidWorkflowId(_)
            | Self::InvalidUserId(_)
            | Self::UnsupportedRunType(_)
            | Self::InvalidCronExpression(_)
            | Self::MissingScheduleField { .. }
            | Self::InvalidScheduleTime => client_error(StatusCode::BAD_REQUEST, self.to_string()),
            Self::Database(e) => internal_error(e),
        }
    }
}

impl ResponseError for WorkflowTaskServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::WorkflowTaskNotFound | Self::WorkflowNotFound | Self::TaskNotFound => {
                client_error(StatusCode::NOT_FOUND, self.to_string())
            }
            Self::NotWorkflowOwner | Self::NotTaskOwner => {
                client_error(StatusCode::FORBIDDEN, self.to_string())
            }
            Self::DuplicateStepName(_) => client_error(StatusCode::CONFLICT, self.to_string()),
            Self::InvalidId(_)
            | Self::InvalidJsonField { .. }
            | Self::NegativeRetryConfiguration => {
                client_error(StatusCode::BAD_REQUEST, self.to_string())
            }
            Self::Database(e) => internal_error(e),
        }
    }
}

impl ResponseError for WorkflowTaskEdgeServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::EdgeNotFound | Self::WorkflowNotFound | Self::WorkflowTaskNotFound => {
                client_error(StatusCode::NOT_FOUND, self.to_string())
            }
            Self::NotWorkflowOwner => client_error(StatusCode::FORBIDDEN, self.to_string()),
            Self::DuplicateEdge => client_error(StatusCode::CONFLICT, self.to_string()),

            // A malformed graph is something the caller described wrongly, and
            // the message says exactly which rule was broken so they can fix it.
            Self::StepOutsideWorkflow
            | Self::SelfLoop
            | Self::CycleDetected
            | Self::InvalidId(_) => client_error(StatusCode::BAD_REQUEST, self.to_string()),
            Self::Database(e) => internal_error(e),
        }
    }
}
