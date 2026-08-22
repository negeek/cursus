use jiff::civil::DateTime;
use uuid::Uuid;

use crate::models::{Workflow, WorkflowTaskState};

/// One run of a workflow. A workflow has many of these over its life, one per
/// trigger.
#[derive(Debug, toasty::Model)]
#[table = "workflow_states"]
pub struct WorkflowState {
    #[key]
    #[auto(uuid(v7))]
    pub id: Uuid,

    #[index]
    pub workflow_id: Uuid,

    #[belongs_to(key = workflow_id, references = id)]
    pub workflow: toasty::Deferred<Workflow>,

    /// One of "pending", "running", "success", or "failed".
    pub status: String,

    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>,

    pub date_created: DateTime,
    pub date_updated: DateTime,

    #[has_many]
    pub workflow_task_states: toasty::Deferred<Vec<WorkflowTaskState>>,
}
