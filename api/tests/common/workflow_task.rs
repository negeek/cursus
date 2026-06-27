use api::db::entity::workflow_task::{ActiveModel as WorkflowTaskOp, Model as WorkflowTaskRow};
use api::db::repository::{RepositoryTrait, workflow_task::WorkflowTaskRepository};
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;

pub async fn test_workflow_task(
    db: &DatabaseConnection,
    workflow_id: String,
    task_id: String,
    position: i32,
) -> WorkflowTaskRow {
    test_workflow_task_dyn(db, workflow_id, task_id, position, String::from("Step_1")).await
}

pub async fn test_workflow_task_dyn(
    db: &DatabaseConnection,
    workflow_id: String,
    task_id: String,
    position: i32,
    step_name: String,
) -> WorkflowTaskRow {
    let workflow_task_data = WorkflowTaskOp {
        id: Default::default(),
        workflow_id: Set(uuid::Uuid::parse_str(&workflow_id).unwrap()),
        task_id: Set(uuid::Uuid::parse_str(&task_id).unwrap()),
        step_name: Set(step_name),
        run_position: Set(position),
        ..Default::default()
    };
    let workflow_res = WorkflowTaskRepository {}
        .create(db, workflow_task_data)
        .await;
    match workflow_res {
        Ok(w) => return w,
        Err(e) => panic!("Error: {}", e),
    };
}
