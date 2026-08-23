use crate::dtos::error::ValidationError;
use crate::models::Task;
use crate::util::type_conv::{json_column_to_value, value_to_json};
use crate::{dtos::RequestValidateTrait, util::validate::validate_url};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TaskResultSchema {
    #[schema(value_type = Object)]
    pub data: serde_json::Value,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TaskBody {
    #[schema(value_type = Object)]
    pub data: serde_json::Value,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateTaskRequest {
    pub name: String,
    pub description: Option<String>,
    pub endpoint: String,
    pub body: Option<TaskBody>,
    pub result_schema: Option<TaskResultSchema>,
}

impl RequestValidateTrait for CreateTaskRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        validate_url(&self.endpoint, Some("endpoint"))?;
        Ok(())
    }
}

#[derive(Deserialize, ToSchema)]
pub struct EditTaskRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub endpoint: Option<String>,
    pub body: Option<TaskBody>,
    pub result_schema: Option<TaskResultSchema>,
}

impl RequestValidateTrait for EditTaskRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref endpoint) = self.endpoint {
            validate_url(endpoint, Some("endpoint"))?;
        }
        Ok(())
    }
}

#[derive(Serialize, ToSchema)]
pub struct TaskResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub endpoint: String,
    pub method: String,
    pub body: Option<TaskBody>,
    pub result_schema: Option<TaskResultSchema>,
    pub date_created: String,
    pub date_updated: String,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        let result_schema = value_to_json(json_column_to_value(task.result_schema)).unwrap_or(None);
        let body = value_to_json(json_column_to_value(task.body)).unwrap_or(None);
        TaskResponse {
            id: task.id.to_string(),
            name: task.name,
            description: task.description,
            endpoint: task.endpoint,
            method: task.method,
            body,
            result_schema,
            date_created: task.date_created.to_string(),
            date_updated: task.date_updated.to_string(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct DeleteTaskResponse {
    pub message: String,
}
