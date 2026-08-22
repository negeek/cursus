use api::models::Workflow;
use api::repositories::workflow::{CreateWorkflowParams, WorkflowRepository};
use toasty::Db;
use uuid::Uuid;

/// A manually triggered workflow owned by the given user.
pub async fn test_workflow(db: &mut Db, user_id: String) -> Workflow {
    WorkflowRepository
        .create(
            db,
            CreateWorkflowParams {
                user_id: Uuid::parse_str(&user_id).expect("test user id must be a uuid"),
                name: String::from("Test Workflow"),
                description: Some(String::from("This is a test workflow.")),
                run_type: String::from("manual"),
                schedule_time: None,
                cron_expression: None,
                cron_next_run: None,
            },
        )
        .await
        .expect("failed to create the test workflow")
}
