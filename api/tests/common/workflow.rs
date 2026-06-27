use api::db::entity::workflow::{ActiveModel as WorkflowOp, Model as WorkflowRow};
use api::db::repository::{RepositoryTrait, workflow::WorkflowRepository};
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;

pub async fn test_workflow(db: &DatabaseConnection, user_id: String) -> WorkflowRow {
    let workflow_data = WorkflowOp {
        id: Default::default(),
        user_id: Set(uuid::Uuid::parse_str(&user_id).unwrap()),
        name: Set(String::from("Test Workflow")),
        description: Set(Some(String::from("This is a test workflow."))),
        run_type: Set(String::from("manual")),
        schedule_time: Set(None),
        cron_expression: Set(None),
        ..Default::default()
    };
    let workflow_res = WorkflowRepository {}.create(db, workflow_data).await;
    match workflow_res {
        Ok(w) => return w,
        Err(e) => panic!("Error: {}", e),
    };
}
