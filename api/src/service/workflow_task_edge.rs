use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

use crate::db::entity::workflow_task_edge::{
    ActiveModel as WorkflowTaskEdgeOp, Model as WorkflowTaskEdgeRow,
};
use crate::db::repository::RepositoryTrait;
use crate::db::repository::workflow::WorkflowRepository;
use crate::db::repository::workflow_task::WorkflowTaskRepository;
use crate::db::repository::workflow_task_edge::WorkflowTaskEdgeRepository;
use crate::dto::error::workflow_task_edge::WorkflowTaskEdgeServiceError;
use crate::dto::workflow_task_edge::WorkflowTaskEdgeRequest;

pub struct WorkflowTaskEdgeService {
    workflow_repository: WorkflowRepository,
    workflow_task_repository: WorkflowTaskRepository,
    workflow_task_edge_repository: WorkflowTaskEdgeRepository,
}

impl WorkflowTaskEdgeService {
    pub fn new() -> Self {
        Self {
            workflow_repository: WorkflowRepository {},
            workflow_task_repository: WorkflowTaskRepository {},
            workflow_task_edge_repository: WorkflowTaskEdgeRepository {},
        }
    }

    pub async fn create_workflow_task_edge(
        &self,
        db: &impl ConnectionTrait,
        user_id: String,
        workflow_id: String,
        edge_req: WorkflowTaskEdgeRequest,
    ) -> Result<WorkflowTaskEdgeRow, WorkflowTaskEdgeServiceError> {
        let workflow_id = uuid::Uuid::parse_str(&workflow_id)
            .map_err(|_| WorkflowTaskEdgeServiceError::ServiceError)?;
        let existing_workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowTaskEdgeServiceError::WorkflowNotFound),
        };
        if existing_workflow.user_id.to_string() != user_id {
            return Err(WorkflowTaskEdgeServiceError::WorkflowNotOwned);
        }
        // ensure from step to step does not exist as an edge already
        let from_task_id = uuid::Uuid::parse_str(&edge_req.from_task_id)
            .map_err(|_| WorkflowTaskEdgeServiceError::ServiceError)?;
        let from_task = match self
            .workflow_task_repository
            .find_by_step_id(db, &workflow_id, &from_task_id)
            .await?
        {
            Some(t) => t,
            None => return Err(WorkflowTaskEdgeServiceError::WorkflowTaskNotFound),
        };
        let to_task_id = uuid::Uuid::parse_str(&edge_req.to_task_id)
            .map_err(|_| WorkflowTaskEdgeServiceError::ServiceError)?;
        let to_task = match self
            .workflow_task_repository
            .find_by_step_id(db, &workflow_id, &to_task_id)
            .await?
        {
            Some(t) => t,
            None => return Err(WorkflowTaskEdgeServiceError::WorkflowTaskNotFound),
        };
        if from_task.workflow_id != workflow_id || to_task.workflow_id != workflow_id {
            return Err(WorkflowTaskEdgeServiceError::WorkflowTaskNotFound);
        }
        if from_task.id == to_task.id {
            return Err(WorkflowTaskEdgeServiceError::SelfLoop);
        }
        if let Some(_) = self
            .workflow_task_edge_repository
            .find_by_from_and_to(db, &workflow_id, &from_task.id, &to_task.id)
            .await?
        {
            return Err(WorkflowTaskEdgeServiceError::DuplicateEdge);
        }
        // Create workflow task Edge
        let edge_data = WorkflowTaskEdgeOp {
            id: Default::default(),
            workflow_id: Set(workflow_id),
            from_task_id: Set(from_task.id),
            to_task_id: Set(to_task.id),
            ..Default::default()
        };
        let created_edge = self
            .workflow_task_edge_repository
            .create(db, edge_data)
            .await?;
        Ok(created_edge)
    }

    pub async fn edit_workflow_task_edge(
        &self,
        db: &impl ConnectionTrait,
        user_id: String,
        workflow_id: String,
        edge_id: String,
        edge_req: WorkflowTaskEdgeRequest,
    ) -> Result<WorkflowTaskEdgeRow, WorkflowTaskEdgeServiceError> {
        let workflow_id = uuid::Uuid::parse_str(&workflow_id)
            .map_err(|_| WorkflowTaskEdgeServiceError::ServiceError)?;
        let edge_id = uuid::Uuid::parse_str(&edge_id)
            .map_err(|_| WorkflowTaskEdgeServiceError::ServiceError)?;
        let existing_workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowTaskEdgeServiceError::WorkflowNotFound),
        };
        if existing_workflow.user_id.to_string() != user_id {
            return Err(WorkflowTaskEdgeServiceError::WorkflowNotOwned);
        }
        let existing_edge = match self
            .workflow_task_edge_repository
            .find_by_uuid(db, &edge_id)
            .await?
        {
            Some(e) => e,
            None => return Err(WorkflowTaskEdgeServiceError::EdgeNotFound),
        };
        if existing_edge.workflow_id != workflow_id {
            return Err(WorkflowTaskEdgeServiceError::EdgeNotFound);
        }
        // ensure from step to step does not exist as an edge already
        let from_task_id = uuid::Uuid::parse_str(&edge_req.from_task_id)
            .map_err(|_| WorkflowTaskEdgeServiceError::ServiceError)?;
        let to_task_id = uuid::Uuid::parse_str(&edge_req.to_task_id)
            .map_err(|_| WorkflowTaskEdgeServiceError::ServiceError)?;
        let from_task = match self
            .workflow_task_repository
            .find_by_step_id(db, &workflow_id, &from_task_id)
            .await?
        {
            Some(t) => t,
            None => return Err(WorkflowTaskEdgeServiceError::WorkflowTaskNotFound),
        };
        let to_task = match self
            .workflow_task_repository
            .find_by_step_id(db, &workflow_id, &to_task_id)
            .await?
        {
            Some(t) => t,
            None => return Err(WorkflowTaskEdgeServiceError::WorkflowTaskNotFound),
        };
        if from_task.workflow_id != workflow_id || to_task.workflow_id != workflow_id {
            return Err(WorkflowTaskEdgeServiceError::WorkflowTaskNotFound);
        }
        if from_task.id == to_task.id {
            return Err(WorkflowTaskEdgeServiceError::SelfLoop);
        }
        if let Some(_) = self
            .workflow_task_edge_repository
            .find_by_from_and_to(db, &workflow_id, &from_task.id, &to_task.id)
            .await?
        {
            return Err(WorkflowTaskEdgeServiceError::DuplicateEdge);
        }
        // Update workflow task Edge
        let mut existing_edge_op: WorkflowTaskEdgeOp = existing_edge.into();
        existing_edge_op.from_task_id = Set(from_task.id);
        existing_edge_op.to_task_id = Set(to_task.id);
        let updated_edge = self
            .workflow_task_edge_repository
            .update(db, existing_edge_op)
            .await?;
        Ok(updated_edge)
    }

    pub async fn delete_workflow_task_edge(
        &self,
        db: &impl ConnectionTrait,
        user_id: String,
        workflow_id: String,
        edge_id: String,
    ) -> Result<(), WorkflowTaskEdgeServiceError> {
        let workflow_id = uuid::Uuid::parse_str(&workflow_id)
            .map_err(|_| WorkflowTaskEdgeServiceError::ServiceError)?;
        let edge_id = uuid::Uuid::parse_str(&edge_id)
            .map_err(|_| WorkflowTaskEdgeServiceError::ServiceError)?;
        let existing_workflow = match self
            .workflow_repository
            .find_by_uuid(db, &workflow_id)
            .await?
        {
            Some(w) => w,
            None => return Err(WorkflowTaskEdgeServiceError::WorkflowNotFound),
        };
        if existing_workflow.user_id.to_string() != user_id {
            return Err(WorkflowTaskEdgeServiceError::WorkflowNotOwned);
        }
        let existing_edge = match self
            .workflow_task_edge_repository
            .find_by_uuid(db, &edge_id)
            .await?
        {
            Some(e) => e,
            None => return Err(WorkflowTaskEdgeServiceError::EdgeNotFound),
        };
        if existing_edge.workflow_id != workflow_id {
            return Err(WorkflowTaskEdgeServiceError::EdgeNotFound);
        }
        let existing_edge_op: WorkflowTaskEdgeOp = existing_edge.into();
        self.workflow_task_edge_repository
            .delete(db, existing_edge_op)
            .await?;
        Ok(())
    }
}
