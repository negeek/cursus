use jiff::civil::DateTime;
use serde_json::Value;
use toasty::stmt::Json;
use uuid::Uuid;

use crate::models::{User, WorkflowTask};

/// A unit of work the user owns, reachable at an HTTP endpoint they control.
///
/// Cursus never runs this itself. It only ever calls the endpoint and waits for
/// the result to come back.
#[derive(Debug, toasty::Model)]
#[table = "tasks"]
pub struct Task {
    #[key]
    #[auto(uuid(v7))]
    pub id: Uuid,

    #[index]
    pub user_id: Uuid,

    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::Deferred<User>,

    pub name: String,

    pub description: Option<String>,

    /// The endpoint cursus calls when this task's dependencies are satisfied.
    pub endpoint: String,

    /// Always POST in practice. Kept as a column so the shape stays open, with
    /// the default applied by toasty at insert time rather than by every caller.
    #[default("POST".to_string())]
    pub method: String,

    /// Optional JSON schema used to validate whatever the task reports back.
    /// Null means anything is accepted.
    ///
    /// Toasty will not infer a storage type for `Json`, it has to be stated, and
    /// text is the only backing it offers. Note this is not `JSONB`, so Postgres
    /// JSON operators are unavailable. Acceptable here because these are opaque
    /// payloads passed through to the user's own endpoint, never queried into.
    #[column(type = text)]
    pub result_schema: Option<Json<Value>>,

    /// Default request body sent to the endpoint, overridable per workflow step.
    #[column(type = text)]
    pub body: Option<Json<Value>>,

    pub date_created: DateTime,
    pub date_updated: DateTime,

    #[has_many]
    pub workflow_tasks: toasty::Deferred<Vec<WorkflowTask>>,
}
