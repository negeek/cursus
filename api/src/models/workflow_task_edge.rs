use jiff::civil::DateTime;
use uuid::Uuid;

use crate::models::{Workflow, WorkflowTask};

/// A dependency between two steps: `to_task` may not start until `from_task`
/// finishes. These rows are the DAG.
///
/// Two things about this model are worth knowing before changing it.
///
/// First, both `from_task` and `to_task` point at the same table. sea-orm needed
/// an explicit `relation_enum` to tell them apart. Toasty does not, the distinct
/// field names are enough on their own.
///
/// Second, and less comfortably, `WorkflowTask` cannot declare `has_many` back
/// to this model. It would need two of them, outgoing and incoming, and toasty
/// offers no way to disambiguate a pair of `has_many` pointing at the same
/// target. The consequence is that deleting a `WorkflowTask` will NOT clean up
/// the edges referencing it, so the delete path has to remove them explicitly.
/// See docs/restructure-plan.md section 0, finding 6.
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
