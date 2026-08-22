/// What can go wrong in the task service.
///
/// This describes the problem in the language of the domain and nothing else.
/// It carries no status codes and knows nothing about HTTP, so the same error
/// reads the same whether it surfaced through a request handler, the runner, or
/// a background job. Turning one of these into a response is the job of
/// `crate::handlers::errors`.
#[derive(Debug, thiserror::Error)]
pub enum TaskServiceError {
    #[error("task not found")]
    TaskNotFound,

    /// The caller is authenticated, but this task belongs to someone else.
    #[error("this task belongs to another user")]
    NotTaskOwner,

    #[error("'{0}' is not a valid task id")]
    InvalidTaskId(String),

    #[error("'{0}' is not a valid user id")]
    InvalidUserId(String),

    #[error("the {field} field is not valid json")]
    InvalidJsonField { field: &'static str },

    #[error("database error: {0}")]
    Database(#[from] toasty::Error),
}
