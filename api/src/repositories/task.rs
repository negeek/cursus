use serde_json::Value;
use toasty::stmt::Json;
use uuid::Uuid;

use crate::models::{Task, now};

pub struct CreateTaskParams {
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub endpoint: String,
    pub result_schema: Option<Value>,
    pub body: Option<Value>,
}

#[derive(Default)]
pub struct UpdateTaskParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub endpoint: Option<String>,
    pub result_schema: Option<Value>,
    pub body: Option<Value>,
}

pub struct TaskRepository;

impl TaskRepository {
    pub async fn create(
        &self,
        db: &mut toasty::Db,
        params: CreateTaskParams,
    ) -> Result<Task, toasty::Error> {
        let timestamp = now();
        toasty::create!(Task {
            user_id: params.user_id,
            name: params.name,
            description: params.description,
            endpoint: params.endpoint,
            result_schema: params.result_schema.map(Json),
            body: params.body.map(Json),
            date_created: timestamp,
            date_updated: timestamp,
        })
        .exec(db)
        .await
    }

    pub async fn find_by_id(
        &self,
        db: &mut toasty::Db,
        id: Uuid,
    ) -> Result<Option<Task>, toasty::Error> {
        Task::filter(Task::fields().id().eq(id))
            .first()
            .exec(db)
            .await
    }

    pub async fn find_by_user_id(
        &self,
        db: &mut toasty::Db,
        user_id: Uuid,
    ) -> Result<Vec<Task>, toasty::Error> {
        Task::filter(Task::fields().user_id().eq(user_id))
            .exec(db)
            .await
    }

    pub async fn update(
        &self,
        db: &mut toasty::Db,
        task: &mut Task,
        params: UpdateTaskParams,
    ) -> Result<(), toasty::Error> {
        let mut stmt = toasty::update!(task {
            date_updated: now()
        });
        if let Some(name) = params.name {
            stmt = stmt.name(name);
        }
        if let Some(description) = params.description {
            stmt = stmt.description(description);
        }
        if let Some(endpoint) = params.endpoint {
            stmt = stmt.endpoint(endpoint);
        }
        if let Some(result_schema) = params.result_schema {
            stmt = stmt.result_schema(Json(result_schema));
        }
        if let Some(body) = params.body {
            stmt = stmt.body(Json(body));
        }
        stmt.exec(db).await
    }

    pub async fn delete(&self, db: &mut toasty::Db, id: Uuid) -> Result<(), toasty::Error> {
        Task::filter(Task::fields().id().eq(id))
            .delete()
            .exec(db)
            .await
    }
}
