use jiff::civil::DateTime;
use uuid::Uuid;

use crate::models::{Workflow, WorkflowTask};

/// A dependency between two steps: `to_task` may not start until `from_task`
/// finishes. These rows are the DAG.
///
/// Both `from_task` and `to_task` point at the same table, which needs care in
/// two places. Here, the distinct field names are enough on their own, so
/// neither carries a disambiguating attribute. On the other side, where
/// `WorkflowTask` declares the matching `has_many` pair, the field name is not
/// enough and each one has to name the relation it reverses with `pair`.
///
/// Those reverse relations are what make deleting a step take its edges with
/// it, in both directions. Nothing here has to be cleaned up by hand.
// Toasty generates an accessor named after each field, so `from_task` becomes a
// `from_*` method taking self. Clippy reads that as breaking the constructor
// naming convention, but the name is the domain's, saying which end of the edge
// it is, and renaming the field to satisfy the lint would lose that meaning.
#[allow(clippy::wrong_self_convention)]
#[derive(Clone, Debug, toasty::Model)]
#[table = "workflow_task_edges"]
#[unique(workflow_id, from_task_id, to_task_id)]
pub struct WorkflowTaskEdge {
    #[key]
    #[auto(uuid(v7))]
    pub id: Uuid,

    #[index]
    pub workflow_id: Uuid,

    #[belongs_to(key = workflow_id, references = id)]
    pub workflow: toasty::Deferred<Workflow>,

    /// Indexed deliberately: walking the DAG forward means repeatedly asking for
    /// the edges leaving a given step, and toasty indexes nothing on its own
    /// beyond the declared uniques.
    #[index]
    pub from_task_id: Uuid,

    #[belongs_to(key = from_task_id, references = id)]
    pub from_task: toasty::Deferred<WorkflowTask>,

    /// Indexed for the same reason, in the other direction: deciding whether a
    /// step is ready means asking what still points at it.
    #[index]
    pub to_task_id: Uuid,

    #[belongs_to(key = to_task_id, references = id)]
    pub to_task: toasty::Deferred<WorkflowTask>,

    pub date_created: DateTime,
    pub date_updated: DateTime,
}
