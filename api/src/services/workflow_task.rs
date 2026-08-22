use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

use crate::models::workflow_task::{ActiveModel as WorkflowTaskOp, Model as WorkflowTaskRow};
use crate::repositories::RepositoryTrait;
use crate::repositories::task::TaskRepository;
use crate::repositories::workflow::WorkflowRepository;
use crate::repositories::workflow_task::WorkflowTaskRepository;
use crate::dtos::error::workflow_task::WorkflowTaskServiceError;
use crate::dtos::workflow_task::{CreateWorkflowTaskRequest, EditWorkflowTaskRequest};
use crate::util::type_conv::{json_to_value, option_u32_to_option_i32};

pub struct WorkflowTaskService {
    workflow_repository: WorkflowRepository,
    workflow_task_repository: WorkflowTaskRepository,
    task_repository: TaskRepository,
}

impl WorkflowTaskService {
    pub fn new() -> Self {
        Self {
            workflow_repository: WorkflowRepository {},
            workflow_task_repository: WorkflowTaskRepository {},
            task_repository: TaskRepository {},
        }
    }

    pub async fn create_workflow_task(
        &self,
        db: &impl ConnectionTrait,
        user_id: String,
        workflow_id: String,
        workflow_task_req: CreateWorkflowTaskRequest,
    ) -> Result<WorkflowTaskRow, WorkflowTaskServiceError> {
        let workflow_id = uuid::Uuid::parse_str(&workflow_id)
            .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
        let task_id = uuid::Uuid::parse_str(&workflow_task_req.task_id)
            .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
        // Check if workflow exists and belongs to user
        let existing_workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowTaskServiceError::WorkflowNotFound),
        };
        if existing_workflow.user_id.to_string() != user_id {
            return Err(WorkflowTaskServiceError::WorkflowNotOwned);
        }
        // Check if task exists and belongs to user
        let existing_task = match self.task_repository.find_by_uuid(db, &task_id).await? {
            Some(t) => t,
            None => return Err(WorkflowTaskServiceError::TaskNotFound),
        };
        if existing_task.user_id.to_string() != user_id {
            return Err(WorkflowTaskServiceError::TaskNotOwned);
        }
        // Check if step name is unique within the workflow
        match self
            .workflow_task_repository
            .find_by_step_name(db, &workflow_id, &workflow_task_req.step_name)
            .await?
        {
            Some(_) => return Err(WorkflowTaskServiceError::DuplicateStepName),
            None => (),
        };
        // Create workflow task
        let retry_count = option_u32_to_option_i32(workflow_task_req.retry_count);
        let retry_delay_secs = option_u32_to_option_i32(workflow_task_req.retry_delay_secs);
        let result_schema = json_to_value(workflow_task_req.task_result_schema)
            .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
        let task_body = json_to_value(workflow_task_req.task_body)
            .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
        let workflow_task_data = WorkflowTaskOp {
            id: Default::default(),
            workflow_id: Set(workflow_id),
            task_id: Set(task_id),
            step_name: Set(workflow_task_req.step_name),
            task_body: Set(task_body),
            task_result_schema: Set(result_schema),
            run_position: Set(workflow_task_req.run_position as i32),
            retry_count: Set(retry_count),
            retry_delay_secs: Set(retry_delay_secs),
            ..Default::default()
        };
        let created_workflow_task = self
            .workflow_task_repository
            .create(db, workflow_task_data)
            .await?;
        Ok(created_workflow_task)
    }

    pub async fn edit_workflow_task(
        &self,
        db: &impl ConnectionTrait,
        user_id: String,
        workflow_id: String,
        workflow_task_id: String,
        workflow_task_req: EditWorkflowTaskRequest,
    ) -> Result<WorkflowTaskRow, WorkflowTaskServiceError> {
        // Similar checks as create_workflow_task for ownership and existence
        // Then update the workflow task with new values
        let workflow_id = uuid::Uuid::parse_str(&workflow_id)
            .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
        let workflow_task_id = uuid::Uuid::parse_str(&workflow_task_id)
            .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
        // Check if workflow exists and belongs to user
        let existing_workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowTaskServiceError::WorkflowNotFound),
        };
        if existing_workflow.user_id.to_string() != user_id {
            return Err(WorkflowTaskServiceError::WorkflowNotOwned);
        };
        let workflow_task = match self
            .workflow_task_repository
            .find_by_uuid(db, &workflow_task_id)
            .await?
        {
            Some(wt) => wt,
            None => return Err(WorkflowTaskServiceError::TaskNotFound),
        };
        // Check if step name is unique within the workflow if it's being updated
        if let Some(ref step_name) = workflow_task_req.step_name {
            match self
                .workflow_task_repository
                .find_by_step_name(db, &workflow_id, step_name)
                .await?
            {
                Some(existing_task) if existing_task.id != workflow_task_id => {
                    return Err(WorkflowTaskServiceError::DuplicateStepName);
                }
                _ => (),
            };
        }
        // let's update the workflow task now
        let mut workflow_task_op: WorkflowTaskOp = workflow_task.into();
        if workflow_task_req.step_name.is_some() {
            workflow_task_op.step_name = Set(workflow_task_req.step_name.unwrap());
        }
        if workflow_task_req.task_body.is_some() {
            let task_body = json_to_value(workflow_task_req.task_body)
                .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
            workflow_task_op.task_body = Set(task_body);
        }
        if workflow_task_req.task_result_schema.is_some() {
            let result_schema = json_to_value(workflow_task_req.task_result_schema)
                .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
            workflow_task_op.task_result_schema = Set(result_schema);
        }
        if workflow_task_req.retry_count.is_some() {
            workflow_task_op.retry_count =
                Set(option_u32_to_option_i32(workflow_task_req.retry_count));
        }
        if workflow_task_req.retry_delay_secs.is_some() {
            workflow_task_op.retry_delay_secs =
                Set(option_u32_to_option_i32(workflow_task_req.retry_delay_secs));
        }
        if workflow_task_req.run_position.is_some() {
            workflow_task_op.run_position = Set(workflow_task_req.run_position.unwrap() as i32);
        }
        let updated_workflow_task = workflow_task_op.update(db).await?;
        Ok(updated_workflow_task)
    }

    pub async fn get_workflow_task(
        &self,
        db: &impl ConnectionTrait,
        user_id: String,
        workflow_id: String,
        workflow_task_id: String,
    ) -> Result<WorkflowTaskRow, WorkflowTaskServiceError> {
        // Implementation for retrieving a specific workflow task
        let workflow_id = uuid::Uuid::parse_str(&workflow_id)
            .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
        let workflow_task_id = uuid::Uuid::parse_str(&workflow_task_id)
            .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
        let existing_workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowTaskServiceError::WorkflowNotFound),
        };
        if existing_workflow.user_id.to_string() != user_id {
            return Err(WorkflowTaskServiceError::WorkflowNotOwned);
        };
        let workflow_task = match self
            .workflow_task_repository
            .find_by_uuid(db, &workflow_task_id)
            .await?
        {
            Some(wt) => wt,
            None => return Err(WorkflowTaskServiceError::TaskNotFound),
        };
        Ok(workflow_task)
    }

    pub async fn delete_workflow_task(
        &self,
        db: &impl ConnectionTrait,
        user_id: String,
        workflow_id: String,
        workflow_task_id: String,
    ) -> Result<(), WorkflowTaskServiceError> {
        // Implementation for deleting a specific workflow task
        let workflow_id = uuid::Uuid::parse_str(&workflow_id)
            .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
        let workflow_task_id = uuid::Uuid::parse_str(&workflow_task_id)
            .map_err(|_| WorkflowTaskServiceError::ServiceError)?;
        let existing_workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowTaskServiceError::WorkflowNotFound),
        };
        if existing_workflow.user_id.to_string() != user_id {
            return Err(WorkflowTaskServiceError::WorkflowNotOwned);
        };
        let workflow_task = match self
            .workflow_task_repository
            .find_by_uuid(db, &workflow_task_id)
            .await?
        {
            Some(wt) => wt,
            None => return Err(WorkflowTaskServiceError::TaskNotFound),
        };
        let workflow_task_op: WorkflowTaskOp = workflow_task.into();
        self.workflow_task_repository
            .delete(db, workflow_task_op)
            .await?;
        Ok(())
    }
}
