use api::models::Task;
use api::repositories::task::{CreateTaskParams, TaskRepository};
use toasty::Db;
use uuid::Uuid;

/// A task owned by the given user, with a fixed name.
pub async fn test_task(db: &mut Db, user_id: String) -> Task {
    test_task_dyn(db, user_id, String::from("Test Task")).await
}

/// A task owned by the given user, under a name the caller chooses.
///
/// Tests that need several tasks in one workflow use this, since a fixed name
/// would make them indistinguishable.
pub async fn test_task_dyn(db: &mut Db, user_id: String, name: String) -> Task {
    TaskRepository
        .create(
            db,
            CreateTaskParams {
                user_id: Uuid::parse_str(&user_id).expect("test user id must be a uuid"),
                name,
                description: Some(String::from("This is a test task.")),
                endpoint: String::from("http://example.com/test-endpoint"),
                result_schema: Some(serde_json::json!({"result": {"status": "string"}})),
                body: Some(serde_json::json!({"input": "string"})),
            },
        )
        .await
        .expect("failed to create the test task")
}
