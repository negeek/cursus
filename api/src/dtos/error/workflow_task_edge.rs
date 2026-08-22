/// What can go wrong when wiring one workflow step to another.
///
/// These edges are the DAG, so most of what can go wrong here is a graph
/// integrity problem rather than a lookup failure. Domain language only, no
/// status codes. See `crate::handlers::errors`.
#[derive(Debug, thiserror::Error)]
pub enum WorkflowTaskEdgeServiceError {
    #[error("edge not found")]
    EdgeNotFound,

    #[error("workflow not found")]
    WorkflowNotFound,

    #[error("workflow task not found")]
    WorkflowTaskNotFound,

    #[error("this workflow belongs to another user")]
    NotWorkflowOwner,

    /// One or both endpoints of the edge are steps of a different workflow.
    /// Allowing this would let a graph span workflows, which nothing supports.
    #[error("both steps must belong to this workflow")]
    StepOutsideWorkflow,

    #[error("an edge already connects these two steps")]
    DuplicateEdge,

    #[error("a step cannot depend on itself")]
    SelfLoop,

    /// Adding this edge would close a cycle, and a cyclic graph has no valid
    /// execution order.
    #[error("this edge would create a cycle in the workflow")]
    CycleDetected,

    #[error("'{0}' is not a valid id")]
    InvalidId(String),

    #[error("database error: {0}")]
    Database(#[from] toasty::Error),
}
