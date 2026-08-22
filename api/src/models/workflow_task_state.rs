use jiff::civil::DateTime;
use serde_json::Value;
use toasty::stmt::Json;
use uuid::Uuid;

use crate::models::{WorkflowState, WorkflowTask};

/// One step's outcome within one run of a workflow.
#[derive(Debug, toasty::Model)]
#[table = "workflow_task_states"]
pub struct WorkflowTaskState {
    #[key]
    #[auto(uuid(v7))]
    pub id: Uuid,

    #[index]
    pub workflow_task_id: Uuid,

    /// Points at the workflow step, not at the underlying task. The sea-orm
    /// version of this model had it typed against `task` while the column was
    /// named `workflow_task_id`, which was a latent bug. Corrected here.
    #[belongs_to(key = workflow_task_id, references = id)]
    pub workflow_task: toasty::Deferred<WorkflowTask>,

    #[index]
    pub workflow_state_id: Uuid,

    #[belongs_to(key = workflow_state_id, references = id)]
    pub workflow_state: toasty::Deferred<WorkflowState>,

    /// Whatever the task's endpoint reported back on completion.
    #[column(type = text)]
    pub result: Option<Json<Value>>,

    /// One of "pending", "running", "success", or "failed".
    pub status: String,

    pub retry_attempts: i32,

    /// The secret the task's endpoint sends back to report this step finished.
    ///
    /// This is a credential, not an identifier. It authorises marking this one
    /// step complete, so it has to be unguessable, scoped to this row alone, and
    /// accepted only once. Generating it on dispatch and checking it on callback
    /// belong to the runner.
    ///
    /// It must never appear in a log line. When a dispatch and its later
    /// callback need tying together in logs, use `id`, which is safe to record.
    #[unique]
    pub callback_token: String,

    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>,

    pub date_created: DateTime,
    pub date_updated: DateTime,
}
