use jiff::civil::DateTime;
use serde_json::Value;
use toasty::stmt::Json;
use uuid::Uuid;

use crate::models::{Task, Workflow, WorkflowTaskState};

/// One step in a workflow: a task, placed in a particular workflow, under a name
/// unique to that workflow.
#[derive(Debug, toasty::Model)]
#[table = "workflow_tasks"]
#[unique(workflow_id, step_name)]
pub struct WorkflowTask {
    #[key]
    #[auto(uuid(v7))]
    pub id: Uuid,

    #[index]
    pub workflow_id: Uuid,

    #[belongs_to(key = workflow_id, references = id)]
    pub workflow: toasty::Deferred<Workflow>,

    #[index]
    pub task_id: Uuid,

    #[belongs_to(key = task_id, references = id)]
    pub task: toasty::Deferred<Task>,

    /// Identifies the step within its workflow. Half of the composite unique
    /// declared on the struct.
    pub step_name: String,

    /// Overrides the task's own body for this workflow. Null falls back to the
    /// task's default.
    #[column(type = text)]
    pub task_body: Option<Json<Value>>,

    /// Overrides the task's own result schema for this workflow. Null falls back
    /// to the task's default.
    #[column(type = text)]
    pub task_result_schema: Option<Json<Value>>,

    pub run_position: i32,

    /// How many times to retry this step, and how long to wait between attempts.
    /// Both are the runner's concern, not the SDK's.
    pub retry_count: Option<i32>,
    pub retry_delay_secs: Option<i32>,

    pub date_created: DateTime,
    pub date_updated: DateTime,

    #[has_many]
    pub workflow_task_states: toasty::Deferred<Vec<WorkflowTaskState>>,
}
