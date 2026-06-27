use api::db::entity::workflow_task_edge::{
    ActiveModel as WorkflowTaskEdgeOp, Model as WorkflowTaskEdgeRow,
};
use api::db::repository::{RepositoryTrait, workflow_task_edge::WorkflowTaskEdgeRepository};
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;

pub async fn test_workflow_task_edge(
    db: &DatabaseConnection,
    workflow_id: String,
    from_task_id: String,
    to_task_id: String,
) -> WorkflowTaskEdgeRow {
    let workflow_task_edge_data = WorkflowTaskEdgeOp {
        id: Default::default(),
        workflow_id: Set(uuid::Uuid::parse_str(&workflow_id).unwrap()),
        from_task_id: Set(uuid::Uuid::parse_str(&from_task_id).unwrap()),
        to_task_id: Set(uuid::Uuid::parse_str(&to_task_id).unwrap()),
        ..Default::default()
    };
    let workflow_res = WorkflowTaskEdgeRepository {}
        .create(db, workflow_task_edge_data)
        .await;
    match workflow_res {
        Ok(w) => return w,
        Err(e) => panic!("Error: {}", e),
    };
}
