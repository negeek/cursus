use api::models::WorkflowTask;
use api::repositories::workflow_task::{CreateWorkflowTaskParams, WorkflowTaskRepository};
use toasty::Db;
use uuid::Uuid;

/// A step placing a task into a workflow, named "Step_1".
pub async fn test_workflow_task(
    db: &mut Db,
    workflow_id: String,
    task_id: String,
    position: i32,
) -> WorkflowTask {
    test_workflow_task_dyn(db, workflow_id, task_id, position, String::from("Step_1")).await
}

/// The same, under a name the caller chooses.
///
/// Step names are unique within a workflow, so any test building more than one
/// step has to use this rather than the fixed name above.
pub async fn test_workflow_task_dyn(
    db: &mut Db,
    workflow_id: String,
    task_id: String,
    position: i32,
    step_name: String,
) -> WorkflowTask {
    WorkflowTaskRepository
        .create(
            db,
            CreateWorkflowTaskParams {
                workflow_id: Uuid::parse_str(&workflow_id).expect("workflow id must be a uuid"),
                task_id: Uuid::parse_str(&task_id).expect("task id must be a uuid"),
                step_name,
                task_body: None,
                task_result_schema: None,
                run_position: position,
                retry_count: None,
                retry_delay_secs: None,
            },
        )
        .await
        .expect("failed to create the test workflow task")
}
