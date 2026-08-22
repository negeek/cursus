pub mod error;
pub mod task;
pub mod user;
pub mod workflow;
pub mod workflow_task;
pub mod workflow_task_edge;

use crate::dtos::error::ValidationError;

pub trait RequestValidateTrait {
    fn validate(&self) -> Result<(), ValidationError>;
}
