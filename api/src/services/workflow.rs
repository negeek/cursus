use sea_orm::{ConnectionTrait, Set};

use crate::models::workflow::{ActiveModel as WorkflowOp, Model as WorkflowRow};
use crate::repositories::RepositoryTrait;
use crate::repositories::workflow::WorkflowRepository;
use crate::dtos::error::workflow::WorkflowServiceError;
use crate::dtos::workflow::{CreateWorkflowRequest, EditWorkflowRequest, WorkflowDetailResponse};
use crate::util::type_conv::option_string_to_option_datetime;

pub struct WorkflowService {
    workflow_repository: WorkflowRepository,
}

impl WorkflowService {
    pub fn new() -> Self {
        Self {
            workflow_repository: WorkflowRepository {},
        }
    }

    pub async fn create_workflow(
        &self,
        db: &impl ConnectionTrait,
        user_id: String,
        workflow_req: CreateWorkflowRequest,
    ) -> Result<WorkflowRow, WorkflowServiceError> {
        let user_id =
            uuid::Uuid::parse_str(&user_id).map_err(|_| WorkflowServiceError::ServiceError)?;
        let schedule_time = option_string_to_option_datetime(workflow_req.schedule_time)
            .map_err(|_| WorkflowServiceError::ServiceError)?;
        let workflow_data = WorkflowOp {
            id: Default::default(),
            user_id: Set(user_id),
            name: Set(workflow_req.name),
            description: Set(workflow_req.description),
            run_type: Set(workflow_req.run_type),
            schedule_time: Set(schedule_time),
            cron_expression: Set(workflow_req.cron_expression),
            ..Default::default()
        };
        let created_workflow = self.workflow_repository.create(db, workflow_data).await?;
        Ok(created_workflow)
    }

    pub async fn edit_workflow(
        &self,
        db: &impl ConnectionTrait,
        user_id: &String,
        workflow_id: String,
        workflow_req: EditWorkflowRequest,
    ) -> Result<WorkflowRow, WorkflowServiceError> {
        let workflow_id =
            uuid::Uuid::parse_str(&workflow_id).map_err(|_| WorkflowServiceError::ServiceError)?;
        let existing_workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowServiceError::WorkflowNotFound),
        };
        if existing_workflow.user_id.to_string() != *user_id {
            return Err(WorkflowServiceError::Unauthorized);
        }
        let mut existing_workflow_op: WorkflowOp = existing_workflow.into();
        if workflow_req.name.is_some() {
            existing_workflow_op.name = Set(workflow_req.name.unwrap());
        }
        if workflow_req.description.is_some() {
            existing_workflow_op.description = Set(workflow_req.description);
        }
        if workflow_req.run_type.is_some() {
            existing_workflow_op.run_type = Set(workflow_req.run_type.unwrap());
        }
        if workflow_req.schedule_time.is_some() {
            existing_workflow_op.schedule_time =
                Set(option_string_to_option_datetime(workflow_req.schedule_time)
                    .map_err(|_| WorkflowServiceError::ServiceError)?);
        }
        if workflow_req.cron_expression.is_some() {
            existing_workflow_op.cron_expression = Set(workflow_req.cron_expression);
        }
        if workflow_req.is_active.is_some() {
            existing_workflow_op.is_active = Set(workflow_req.is_active.unwrap());
        }
        let updated_workflow = self
            .workflow_repository
            .update(db, existing_workflow_op)
            .await?;
        Ok(updated_workflow)
    }

    pub async fn get_workflow_by_id(
        &self,
        db: &impl ConnectionTrait,
        user_id: &String,
        workflow_id: String,
    ) -> Result<WorkflowRow, WorkflowServiceError> {
        let workflow_id =
            uuid::Uuid::parse_str(&workflow_id).map_err(|_| WorkflowServiceError::ServiceError)?;
        let workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowServiceError::WorkflowNotFound),
        };
        if workflow.user_id.to_string() != *user_id {
            return Err(WorkflowServiceError::Unauthorized);
        }
        Ok(workflow)
    }

    /// Full detail of workflow with tasks and edges for a given workflow id
    pub async fn workflow_detail(
        &self,
        db: &impl ConnectionTrait,
        user_id: &String,
        workflow_id: String,
    ) -> Result<WorkflowDetailResponse, WorkflowServiceError> {
        let workflow_id =
            uuid::Uuid::parse_str(&workflow_id).map_err(|_| WorkflowServiceError::ServiceError)?;
        let workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowServiceError::WorkflowNotFound),
        };
        if workflow.user_id.to_string() != *user_id {
            return Err(WorkflowServiceError::Unauthorized);
        }
        let workflow_tasks_edges = self
            .workflow_repository
            .load_tasks_and_edges(db, workflow)
            .await?;
        Ok(WorkflowDetailResponse::from_workflow_row(
            workflow_tasks_edges.0,
            workflow_tasks_edges.1,
            workflow_tasks_edges.2,
        ))
    }

    pub async fn delete_workflow(
        &self,
        db: &impl ConnectionTrait,
        user_id: &String,
        workflow_id: String,
    ) -> Result<(), WorkflowServiceError> {
        let workflow_id =
            uuid::Uuid::parse_str(&workflow_id).map_err(|_| WorkflowServiceError::ServiceError)?;
        let existing_workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowServiceError::WorkflowNotFound),
        };
        if existing_workflow.user_id.to_string() != *user_id {
            return Err(WorkflowServiceError::Unauthorized);
        }
        let existing_workflow_op: WorkflowOp = existing_workflow.into();
        self.workflow_repository
            .delete(db, existing_workflow_op)
            .await?;
        Ok(())
    }

    pub async fn list_workflows(
        &self,
        db: &impl ConnectionTrait,
        user_id: &String,
    ) -> Result<Vec<WorkflowRow>, WorkflowServiceError> {
        let user_id =
            uuid::Uuid::parse_str(&user_id).map_err(|_| WorkflowServiceError::ServiceError)?;
        let workflows = self
            .workflow_repository
            .find_by_user_id(db, &user_id)
            .await?;
        Ok(workflows)
    }
}
