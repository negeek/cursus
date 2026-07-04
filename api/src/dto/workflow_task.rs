use crate::db::entity::workflow_task::Model as WorkflowTaskRow;
use crate::dto::RequestValidateTrait;
use crate::dto::error::ValidationError;
use crate::dto::task::{TaskBody, TaskResultSchema};
use crate::util::type_conv::value_to_json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateWorkflowTaskRequest {
    pub task_id: String,
    pub step_name: String,
    pub task_body: Option<TaskBody>,
    pub task_result_schema: Option<TaskResultSchema>,
    pub run_position: u32,
    pub retry_count: Option<u32>,
    pub retry_delay_secs: Option<u32>,
}

impl RequestValidateTrait for CreateWorkflowTaskRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.step_name.contains(" ") {
            return Err(ValidationError {
                field: String::from("step_name"),
                message: String::from(
                    "Step name cannot contain spaces. It should be a single word or use underscores.",
                ),
            });
        }
        if !(self.retry_count.is_some() == self.retry_delay_secs.is_some()) {
            return Err(ValidationError {
                field: String::from("retry_count/retry_delay_secs"),
                message: String::from(
                    "retry_count and retry_delay_secs must be both set or both null",
                ),
            });
        }
        Ok(())
    }
}

#[derive(Deserialize, ToSchema)]
pub struct EditWorkflowTaskRequest {
    pub step_name: Option<String>,
    pub task_body: Option<TaskBody>,
    pub task_result_schema: Option<TaskResultSchema>,
    pub run_position: Option<u32>,
    pub retry_count: Option<u32>,
    pub retry_delay_secs: Option<u32>,
}

impl RequestValidateTrait for EditWorkflowTaskRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if !(self.retry_count.is_some() == self.retry_delay_secs.is_some()) {
            return Err(ValidationError {
                field: String::from("retry_count/retry_delay_secs"),
                message: String::from(
                    "retry_count and retry_delay_secs must be both set or both null",
                ),
            });
        }
        Ok(())
    }
}

#[derive(Serialize, ToSchema)]
pub struct WorkflowTaskResponse {
    pub id: String,
    pub task_id: String,
    pub step_name: String,
    pub task_body: Option<TaskBody>,
    pub task_result_schema: Option<TaskResultSchema>,
    pub run_position: i32,
    pub retry_count: Option<i32>,
    pub retry_delay_secs: Option<i32>,
    pub date_created: String,
    pub date_updated: String,
}

impl WorkflowTaskResponse {
    pub fn from_workflow_task_row(row: WorkflowTaskRow) -> Self {
        WorkflowTaskResponse {
            id: row.id.to_string(),
            task_id: row.task_id.to_string(),
            step_name: row.step_name,
            task_body: value_to_json(row.task_body).unwrap_or(None),
            task_result_schema: value_to_json(row.task_result_schema).unwrap_or(None),
            run_position: row.run_position,
            retry_count: row.retry_count,
            retry_delay_secs: row.retry_delay_secs,
            date_created: row.date_created.to_rfc3339(),
            date_updated: row.date_updated.to_rfc3339(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct DeleteWorkflowTaskResponse {
    pub message: String,
}
