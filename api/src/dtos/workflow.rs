use std::str::FromStr;

use crate::models::workflow::Model as WorkflowRow;
use crate::models::workflow_task::Model as WorkflowTaskRow;
use crate::models::workflow_task_edge::Model as WorkflowTaskEdgeRow;
use crate::dtos::RequestValidateTrait;
use crate::dtos::error::ValidationError;
use crate::dtos::workflow_task::WorkflowTaskResponse;
use crate::dtos::workflow_task_edge::WorkflowTaskEdgeResponse;
use cron::Schedule;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub enum WorkflowRunType {
    Manual,
    Scheduled,
    Cron,
}

impl WorkflowRunType {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "manual" => Some(WorkflowRunType::Manual),
            "scheduled" => Some(WorkflowRunType::Scheduled),
            "cron" => Some(WorkflowRunType::Cron),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            WorkflowRunType::Manual => "manual".into(),
            WorkflowRunType::Scheduled => "scheduled".into(),
            WorkflowRunType::Cron => "cron".into(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateWorkflowRequest {
    pub name: String,
    pub description: Option<String>,
    pub run_type: String,
    pub schedule_time: Option<String>,
    pub cron_expression: Option<String>,
}

impl RequestValidateTrait for CreateWorkflowRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if WorkflowRunType::from_string(&self.run_type).is_none() {
            return Err(ValidationError {
                field: String::from("run_type"),
                message: String::from("Invalid run_type, must be 'manual', 'scheduled', or 'cron'"),
            });
        }
        if self.run_type == WorkflowRunType::Scheduled.to_string() && self.schedule_time.is_none() {
            return Err(ValidationError {
                field: String::from("schedule_time"),
                message: String::from("schedule_time is required for scheduled run_type"),
            });
        }
        if self.run_type == WorkflowRunType::Cron.to_string() && self.cron_expression.is_none() {
            return Err(ValidationError {
                field: String::from("cron_expression"),
                message: String::from("cron_expression is required for cron run_type"),
            });
        }
        if self.run_type == WorkflowRunType::Scheduled.to_string() {
            if let Some(ref schedule_time) = self.schedule_time {
                if schedule_time.parse::<jiff::Timestamp>().is_err() {
                    return Err(ValidationError {
                        field: String::from("schedule_time"),
                        message: String::from("Invalid schedule_time format, must be ISO 8601"),
                    });
                }
            }
        }
        if self.run_type == WorkflowRunType::Cron.to_string() {
            if let Some(ref cron_expression) = self.cron_expression {
                if Schedule::from_str(cron_expression).is_err() {
                    return Err(ValidationError {
                        field: String::from("cron_expression"),
                        message: String::from("Invalid cron_expression format"),
                    });
                }
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, ToSchema)]
pub struct EditWorkflowRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub run_type: Option<String>,
    pub schedule_time: Option<String>,
    pub cron_expression: Option<String>,
}

impl RequestValidateTrait for EditWorkflowRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.run_type.is_some() {
            let run_type = self.run_type.as_ref().unwrap();
            if WorkflowRunType::from_string(run_type).is_none() {
                return Err(ValidationError {
                    field: String::from("run_type"),
                    message: String::from(
                        "Invalid run_type, must be 'manual', 'scheduled', or 'cron'",
                    ),
                });
            }
            if run_type == &WorkflowRunType::Scheduled.to_string() && self.schedule_time.is_none() {
                return Err(ValidationError {
                    field: String::from("schedule_time"),
                    message: String::from("schedule_time is required for scheduled run_type"),
                });
            }
            if run_type == &WorkflowRunType::Cron.to_string() && self.cron_expression.is_none() {
                return Err(ValidationError {
                    field: String::from("cron_expression"),
                    message: String::from("cron_expression is required for cron run_type"),
                });
            }
            if run_type == &WorkflowRunType::Scheduled.to_string() {
                if let Some(ref schedule_time) = self.schedule_time {
                    if schedule_time.parse::<jiff::Timestamp>().is_err() {
                        return Err(ValidationError {
                            field: String::from("schedule_time"),
                            message: String::from("Invalid schedule_time format, must be ISO 8601"),
                        });
                    }
                }
            }
            if run_type == &WorkflowRunType::Cron.to_string() {
                if let Some(ref cron_expression) = self.cron_expression {
                    if Schedule::from_str(cron_expression).is_err() {
                        return Err(ValidationError {
                            field: String::from("cron_expression"),
                            message: String::from("Invalid cron_expression format"),
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Serialize, ToSchema)]
pub struct WorkflowDetailResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub run_type: String,
    pub schedule_time: Option<String>,
    pub cron_expression: Option<String>,
    pub is_active: bool,
    pub date_created: String,
    pub date_updated: String,
    pub tasks: Vec<WorkflowTaskResponse>,
    pub edges: Vec<WorkflowTaskEdgeResponse>,
}

impl WorkflowDetailResponse {
    pub fn from_workflow_row(
        workflow_row: WorkflowRow,
        tasks: Vec<WorkflowTaskRow>,
        edges: Vec<WorkflowTaskEdgeRow>,
    ) -> Self {
        let tasks = tasks
            .into_iter()
            .map(WorkflowTaskResponse::from_workflow_task_row)
            .collect();
        let edges = edges
            .into_iter()
            .map(WorkflowTaskEdgeResponse::from_workflow_task_edge_row)
            .collect();
        WorkflowDetailResponse {
            id: workflow_row.id.to_string(),
            name: workflow_row.name,
            description: workflow_row.description,
            run_type: workflow_row.run_type,
            schedule_time: workflow_row.schedule_time.map(|dt| dt.to_rfc3339()),
            cron_expression: workflow_row.cron_expression,
            is_active: workflow_row.is_active,
            date_created: workflow_row.date_created.to_rfc3339(),
            date_updated: workflow_row.date_updated.to_rfc3339(),
            tasks: tasks,
            edges: edges,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct WorkflowResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub run_type: String,
    pub schedule_time: Option<String>,
    pub cron_expression: Option<String>,
    pub is_active: bool,
    pub date_created: String,
    pub date_updated: String,
}

#[derive(Serialize, ToSchema)]
pub struct DeleteWorkflowResponse {
    pub message: String,
}

impl WorkflowResponse {
    pub fn from_workflow_row(workflow_row: WorkflowRow) -> Self {
        WorkflowResponse {
            id: workflow_row.id.to_string(),
            name: workflow_row.name,
            description: workflow_row.description,
            run_type: workflow_row.run_type,
            schedule_time: workflow_row.schedule_time.map(|dt| dt.to_rfc3339()),
            cron_expression: workflow_row.cron_expression,
            is_active: workflow_row.is_active,
            date_created: workflow_row.date_created.to_rfc3339(),
            date_updated: workflow_row.date_updated.to_rfc3339(),
        }
    }
}
