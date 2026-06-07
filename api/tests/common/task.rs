use api::db::entity::task::{ActiveModel as TaskOp, Model as TaskRow};
use api::db::repository::{RepositoryTrait, task::TaskRepository};
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;

pub async fn test_task(db: &DatabaseConnection, user_id: String) -> TaskRow {
    let task_data = TaskOp {
        id: Default::default(),
        user_id: Set(uuid::Uuid::parse_str(&user_id).unwrap()),
        name: Set(String::from("Test Task")),
        description: Set(Some(String::from("This is a test task."))),
        endpoint: Set(String::from("/test/endpoint")),
        result_schema: Set(Some(serde_json::json!({"result": {"status": "string"}}))),
        body: Set(Some(serde_json::json!({"input": "string"}))),
        ..Default::default()
    };
    let task_res = TaskRepository {}.create(db, task_data).await;
    match task_res {
        Ok(t) => return t,
        Err(e) => panic!("Error: {}", e),
    };
}
