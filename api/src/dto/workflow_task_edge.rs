use crate::db::entity::workflow_task_edge::Model as WorkflowTaskEdgeRow;
use crate::dto::RequestValidateTrait;
use crate::dto::error::ValidationError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct WorkflowTaskEdgeRequest {
    pub from_task_id: String,
    pub to_task_id: String,
}
impl RequestValidateTrait for WorkflowTaskEdgeRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.from_task_id == self.to_task_id {
            return Err(ValidationError {
                field: String::from("from_task_id/to_task_id"),
                message: String::from("from_task_id and to_task_id cannot be the same"),
            });
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct WorkflowTaskEdgeResponse {
    pub id: String,
    pub from_task_id: String,
    pub to_task_id: String,
}

impl WorkflowTaskEdgeResponse {
    pub fn from_workflow_task_edge_row(edge: WorkflowTaskEdgeRow) -> Self {
        WorkflowTaskEdgeResponse {
            id: edge.id.to_string(),
            from_task_id: edge.from_task_id.to_string(),
            to_task_id: edge.to_task_id.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct DeleteWorkflowTaskEdgeResponse {
    pub message: String,
}
