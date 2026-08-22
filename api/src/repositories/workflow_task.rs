use serde_json::Value;
use toasty::stmt::Json;
use uuid::Uuid;

use crate::models::{WorkflowTask, now};

pub struct CreateWorkflowTaskParams {
    pub workflow_id: Uuid,
    pub task_id: Uuid,
    pub step_name: String,
    pub task_body: Option<Value>,
    pub task_result_schema: Option<Value>,
    pub run_position: i32,
    pub retry_count: Option<i32>,
    pub retry_delay_secs: Option<i32>,
}

#[derive(Default)]
pub struct UpdateWorkflowTaskParams {
    pub step_name: Option<String>,
    pub task_body: Option<Value>,
    pub task_result_schema: Option<Value>,
    pub run_position: Option<i32>,
    pub retry_count: Option<i32>,
    pub retry_delay_secs: Option<i32>,
}

pub struct WorkflowTaskRepository;

impl WorkflowTaskRepository {
    pub async fn create(
        &self,
        db: &mut toasty::Db,
        params: CreateWorkflowTaskParams,
    ) -> Result<WorkflowTask, toasty::Error> {
        let timestamp = now();
        toasty::create!(WorkflowTask {
            workflow_id: params.workflow_id,
            task_id: params.task_id,
            step_name: params.step_name,
            task_body: params.task_body.map(Json),
            task_result_schema: params.task_result_schema.map(Json),
            run_position: params.run_position,
            retry_count: params.retry_count,
            retry_delay_secs: params.retry_delay_secs,
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
    ) -> Result<Option<WorkflowTask>, toasty::Error> {
        WorkflowTask::filter(WorkflowTask::fields().id().eq(id))
            .first()
            .exec(db)
            .await
    }

    /// Every step in a workflow. This is what the runner loads before walking
    /// the graph.
    pub async fn find_by_workflow_id(
        &self,
        db: &mut toasty::Db,
        workflow_id: Uuid,
    ) -> Result<Vec<WorkflowTask>, toasty::Error> {
        WorkflowTask::filter(WorkflowTask::fields().workflow_id().eq(workflow_id))
            .exec(db)
            .await
    }

    pub async fn update(
        &self,
        db: &mut toasty::Db,
        workflow_task: &mut WorkflowTask,
        params: UpdateWorkflowTaskParams,
    ) -> Result<(), toasty::Error> {
        let mut stmt = toasty::update!(workflow_task { date_updated: now() });
        if let Some(step_name) = params.step_name {
            stmt = stmt.step_name(step_name);
        }
        if let Some(task_body) = params.task_body {
            stmt = stmt.task_body(Json(task_body));
        }
        if let Some(task_result_schema) = params.task_result_schema {
            stmt = stmt.task_result_schema(Json(task_result_schema));
        }
        if let Some(run_position) = params.run_position {
            stmt = stmt.run_position(run_position);
        }
        if let Some(retry_count) = params.retry_count {
            stmt = stmt.retry_count(retry_count);
        }
        if let Some(retry_delay_secs) = params.retry_delay_secs {
            stmt = stmt.retry_delay_secs(retry_delay_secs);
        }
        stmt.exec(db).await
    }

    pub async fn delete(&self, db: &mut toasty::Db, id: Uuid) -> Result<(), toasty::Error> {
        WorkflowTask::filter(WorkflowTask::fields().id().eq(id))
            .delete()
            .exec(db)
            .await
    }
}
