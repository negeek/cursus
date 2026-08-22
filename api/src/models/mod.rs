pub mod task;
pub mod token;
pub mod user;
pub mod user_verify;
pub mod workflow;
pub mod workflow_state;
pub mod workflow_task;
pub mod workflow_task_edge;
pub mod workflow_task_state;

pub use task::Task;
pub use token::BlacklistedToken;
pub use user::User;
pub use user_verify::UserVerify;
pub use workflow::Workflow;
pub use workflow_state::WorkflowState;
pub use workflow_task::WorkflowTask;
pub use workflow_task_edge::WorkflowTaskEdge;
pub use workflow_task_state::WorkflowTaskState;

/// Every model in the schema, in one place, so the server, the migrator, and the
/// test harness all register exactly the same set. Adding a model here is the
/// only step needed to bring it into all three.
#[macro_export]
macro_rules! all_models {
    () => {
        toasty::models!(
            $crate::models::User,
            $crate::models::UserVerify,
            $crate::models::BlacklistedToken,
            $crate::models::Task,
            $crate::models::Workflow,
            $crate::models::WorkflowTask,
            $crate::models::WorkflowTaskEdge,
            $crate::models::WorkflowState,
            $crate::models::WorkflowTaskState
        )
    };
}

/// Current UTC wall clock time, in the shape every timestamp column uses.
///
/// Toasty has no equivalent of sea-orm's `before_save` hook, so timestamps are
/// set explicitly on every create and update rather than being filled in
/// automatically. This exists so all of them agree on what "now" means.
pub fn now() -> jiff::civil::DateTime {
    jiff::Timestamp::now()
        .to_zoned(jiff::tz::TimeZone::UTC)
        .datetime()
}
