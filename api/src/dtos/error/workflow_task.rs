/// What can go wrong when placing a task into a workflow as a step.
///
/// Domain language only, no status codes. See `crate::handlers::errors`.
#[derive(Debug, thiserror::Error)]
pub enum WorkflowTaskServiceError {
    #[error("workflow task not found")]
    WorkflowTaskNotFound,

    #[error("workflow not found")]
    WorkflowNotFound,

    #[error("task not found")]
    TaskNotFound,

    #[error("this workflow belongs to another user")]
    NotWorkflowOwner,

    #[error("this task belongs to another user")]
    NotTaskOwner,

    /// Step names identify a step within its workflow, so they have to be
    /// unique there.
    #[error("a step named '{0}' already exists in this workflow")]
    DuplicateStepName(String),

    #[error("'{0}' is not a valid id")]
    InvalidId(String),

    #[error("the {field} field is not valid json")]
    InvalidJsonField { field: &'static str },

    #[error("retry_count and retry_delay_secs cannot be negative")]
    NegativeRetryConfiguration,

    #[error("database error: {0}")]
    Database(#[from] toasty::Error),
}
