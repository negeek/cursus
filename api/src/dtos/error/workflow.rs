/// What can go wrong in the workflow service.
///
/// Domain language only, no status codes. See `crate::handlers::errors`.
#[derive(Debug, thiserror::Error)]
pub enum WorkflowServiceError {
    #[error("workflow not found")]
    WorkflowNotFound,

    #[error("task not found")]
    TaskNotFound,

    #[error("this workflow belongs to another user")]
    NotWorkflowOwner,

    #[error("'{0}' is not a valid workflow id")]
    InvalidWorkflowId(String),

    #[error("'{0}' is not a valid user id")]
    InvalidUserId(String),

    /// A run type outside the accepted set of manual, scheduled, and cron.
    #[error("'{0}' is not a supported run type")]
    UnsupportedRunType(String),

    #[error("'{0}' is not a valid cron expression")]
    InvalidCronExpression(String),

    /// A scheduled or cron workflow missing the field its run type requires.
    #[error("a {run_type} workflow requires {field}")]
    MissingScheduleField {
        run_type: &'static str,
        field: &'static str,
    },

    #[error("the scheduled time is not a valid timestamp")]
    InvalidScheduleTime,

    #[error("database error: {0}")]
    Database(#[from] toasty::Error),
}
