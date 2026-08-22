use jiff::civil::DateTime;
use uuid::Uuid;

use crate::models::{User, WorkflowState, WorkflowTask, WorkflowTaskEdge};

/// A DAG of tasks, plus how it gets triggered.
#[derive(Debug, toasty::Model)]
#[table = "workflows"]
pub struct Workflow {
    #[key]
    #[auto(uuid(v7))]
    pub id: Uuid,

    #[index]
    pub user_id: Uuid,

    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::Deferred<User>,

    pub name: String,

    pub description: Option<String>,

    /// One of "manual", "scheduled", or "cron". The two fields below only apply
    /// to the latter two respectively.
    pub run_type: String,

    /// For "scheduled", the single moment this should run.
    pub schedule_time: Option<DateTime>,

    /// For "cron", the expression and the next moment it is due.
    pub cron_expression: Option<String>,
    pub cron_next_run: Option<DateTime>,

    pub is_active: bool,

    pub date_created: DateTime,
    pub date_updated: DateTime,

    /// Deleting a workflow takes its steps, its edges, and its run history with
    /// it. Toasty needs these declared to do that, since it enforces the
    /// relationship itself rather than through database foreign keys.
    #[has_many]
    pub workflow_tasks: toasty::Deferred<Vec<WorkflowTask>>,

    #[has_many]
    pub workflow_task_edges: toasty::Deferred<Vec<WorkflowTaskEdge>>,

    #[has_many]
    pub workflow_states: toasty::Deferred<Vec<WorkflowState>>,
}
