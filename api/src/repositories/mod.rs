pub mod task;
pub mod token;
pub mod user;
pub mod user_verify;
pub mod workflow;
pub mod workflow_state;
pub mod workflow_task;
pub mod workflow_task_edge;
pub mod workflow_task_state;

use std::error::Error as StdError;

/// Was this failure a unique constraint violation, rather than some other
/// database problem?
///
/// Callers need this to tell "that email is already taken", which is the user's
/// fault and deserves a 409, from a genuine outage, which is not. Toasty wraps
/// the driver's error rather than classifying it, so recognising the case means
/// reaching down to the Postgres SQLSTATE underneath.
///
/// Fails closed. Anything unrecognised is treated as not a unique violation, so
/// an unexpected error surfaces as an internal failure instead of being
/// mistaken for a user error.
pub fn is_unique_violation(err: &toasty::Error) -> bool {
    if !err.is_driver_operation_failed() {
        return false;
    }
    let Some(source) = StdError::source(err) else {
        return false;
    };
    let Some(pg_err) = source.downcast_ref::<tokio_postgres::Error>() else {
        return false;
    };
    pg_err.code() == Some(&tokio_postgres::error::SqlState::UNIQUE_VIOLATION)
}
