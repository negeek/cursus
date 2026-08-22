//! Where domain errors become HTTP responses.
//!
//! The service error enums in `crate::dtos::error` deliberately know nothing
//! about HTTP. They describe what went wrong in the language of the domain, so
//! the same error means the same thing whether it came out of a request, the
//! runner, or a background job. This module is the one place that decides what
//! each of them looks like on the wire.
//!
//! Keeping the mapping here rather than on the enums means a service can be
//! called from somewhere that has no notion of a status code without dragging
//! actix along with it.
//!
//! One rule runs through all of it: a database failure never reaches the caller
//! in detail. It is logged with its cause and answered with a flat internal
//! error, because the text of a database error can leak schema and query shape
//! to whoever is asking.

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::dtos::error::ApiError;
use crate::dtos::error::task::TaskServiceError;
use crate::dtos::error::user::UserServiceError;
use crate::dtos::error::workflow::WorkflowServiceError;
use crate::dtos::error::workflow_task::WorkflowTaskServiceError;
use crate::dtos::error::workflow_task_edge::WorkflowTaskEdgeServiceError;

/// Builds the response for an error that is the caller's to fix, so the message
/// is safe and useful to show them.
fn client_error(status: StatusCode, message: String) -> HttpResponse {
    HttpResponse::build(status).json(ApiError { error: message })
}

/// Builds the response for a failure on our side. The detail is logged, never
/// returned.
fn internal_error(error: &dyn std::fmt::Display) -> HttpResponse {
    tracing::error!(cause = %error, "request failed with an internal error");
    HttpResponse::InternalServerError().json(ApiError::internal_server_error())
}

impl ResponseError for TaskServiceError {
    fn error_response(&self) -> HttpResponse {
        use TaskServiceError::*;
        match self {
            TaskNotFound => client_error(StatusCode::NOT_FOUND, self.to_string()),
            NotTaskOwner => client_error(StatusCode::FORBIDDEN, self.to_string()),
            InvalidTaskId(_) | InvalidUserId(_) | InvalidJsonField { .. } => {
                client_error(StatusCode::BAD_REQUEST, self.to_string())
            }
            Database(e) => internal_error(e),
        }
    }
}

impl ResponseError for UserServiceError {
    fn error_response(&self) -> HttpResponse {
        use UserServiceError::*;
        match self {
            EmailAlreadyExists => client_error(StatusCode::CONFLICT, self.to_string()),

            // Everything an unauthenticated caller can trigger by guessing
            // answers with 401 and the same shape, so nothing here helps
            // distinguish a real account from a fake one.
            InvalidCredentials | InvalidVerificationCode | VerificationCodeExpired
            | MalformedToken | RejectedToken => {
                client_error(StatusCode::UNAUTHORIZED, self.to_string())
            }

            EmailNotVerified => client_error(StatusCode::FORBIDDEN, self.to_string()),
            UserNotFound => client_error(StatusCode::NOT_FOUND, self.to_string()),
            InvalidUserId(_) => client_error(StatusCode::BAD_REQUEST, self.to_string()),

            // Nothing the caller did wrong, and nothing they can act on.
            HashingFailed | TokenIssuanceFailed | MissingConfiguration(_) => internal_error(self),
            Database(e) => internal_error(e),
        }
    }
}

impl ResponseError for WorkflowServiceError {
    fn error_response(&self) -> HttpResponse {
        use WorkflowServiceError::*;
        match self {
            WorkflowNotFound | TaskNotFound => client_error(StatusCode::NOT_FOUND, self.to_string()),
            NotWorkflowOwner => client_error(StatusCode::FORBIDDEN, self.to_string()),
            InvalidWorkflowId(_)
            | InvalidUserId(_)
            | UnsupportedRunType(_)
            | InvalidCronExpression(_)
            | MissingScheduleField { .. }
            | InvalidScheduleTime => client_error(StatusCode::BAD_REQUEST, self.to_string()),
            Database(e) => internal_error(e),
        }
    }
}

impl ResponseError for WorkflowTaskServiceError {
    fn error_response(&self) -> HttpResponse {
        use WorkflowTaskServiceError::*;
        match self {
            WorkflowTaskNotFound | WorkflowNotFound | TaskNotFound => {
                client_error(StatusCode::NOT_FOUND, self.to_string())
            }
            NotWorkflowOwner | NotTaskOwner => client_error(StatusCode::FORBIDDEN, self.to_string()),
            DuplicateStepName(_) => client_error(StatusCode::CONFLICT, self.to_string()),
            InvalidId(_) | InvalidJsonField { .. } | NegativeRetryConfiguration => {
                client_error(StatusCode::BAD_REQUEST, self.to_string())
            }
            Database(e) => internal_error(e),
        }
    }
}

impl ResponseError for WorkflowTaskEdgeServiceError {
    fn error_response(&self) -> HttpResponse {
        use WorkflowTaskEdgeServiceError::*;
        match self {
            EdgeNotFound | WorkflowNotFound | WorkflowTaskNotFound => {
                client_error(StatusCode::NOT_FOUND, self.to_string())
            }
            NotWorkflowOwner => client_error(StatusCode::FORBIDDEN, self.to_string()),
            DuplicateEdge => client_error(StatusCode::CONFLICT, self.to_string()),

            // A malformed graph is something the caller described wrongly, and
            // the message says exactly which rule was broken so they can fix it.
            StepOutsideWorkflow | SelfLoop | CycleDetected | InvalidId(_) => {
                client_error(StatusCode::BAD_REQUEST, self.to_string())
            }
            Database(e) => internal_error(e),
        }
    }
}
