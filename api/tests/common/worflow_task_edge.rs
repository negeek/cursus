use api::models::WorkflowTaskEdge;
use api::repositories::workflow_task_edge::{
    CreateWorkflowTaskEdgeParams, WorkflowTaskEdgeRepository,
};
use toasty::Db;
use uuid::Uuid;

/// A dependency from one step to another within a workflow.
pub async fn test_workflow_task_edge(
    db: &mut Db,
    workflow_id: String,
    from_task_id: String,
    to_task_id: String,
) -> WorkflowTaskEdge {
    WorkflowTaskEdgeRepository
        .create(
            db,
            CreateWorkflowTaskEdgeParams {
                workflow_id: Uuid::parse_str(&workflow_id).expect("workflow id must be a uuid"),
                from_task_id: Uuid::parse_str(&from_task_id).expect("from id must be a uuid"),
                to_task_id: Uuid::parse_str(&to_task_id).expect("to id must be a uuid"),
            },
        )
        .await
        .expect("failed to create the test workflow task edge")
}
