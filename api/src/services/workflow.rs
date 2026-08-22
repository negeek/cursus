use uuid::Uuid;

use crate::dtos::error::workflow::WorkflowServiceError;
use crate::dtos::workflow::{CreateWorkflowRequest, EditWorkflowRequest, WorkflowDetailResponse};
use crate::models::Workflow;
use crate::repositories::workflow::{
    CreateWorkflowParams, UpdateWorkflowParams, WorkflowRepository,
};
use crate::repositories::workflow_task::WorkflowTaskRepository;
use crate::repositories::workflow_task_edge::WorkflowTaskEdgeRepository;
use crate::util::type_conv::option_string_to_option_datetime;

pub struct WorkflowService {
    workflows: WorkflowRepository,
    workflow_tasks: WorkflowTaskRepository,
    edges: WorkflowTaskEdgeRepository,
}

impl Default for WorkflowService {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowService {
    pub fn new() -> Self {
        Self {
            workflows: WorkflowRepository,
            workflow_tasks: WorkflowTaskRepository,
            edges: WorkflowTaskEdgeRepository,
        }
    }

    /// Loads a workflow and confirms the caller owns it.
    ///
    /// A workflow belonging to someone else answers "not found" rather than
    /// "forbidden", so ids cannot be probed for existence.
    async fn owned_workflow(
        &self,
        db: &mut toasty::Db,
        user_id: Uuid,
        workflow_id: Uuid,
    ) -> Result<Workflow, WorkflowServiceError> {
        let workflow = self
            .workflows
            .find_by_id(db, workflow_id)
            .await?
            .ok_or(WorkflowServiceError::WorkflowNotFound)?;

        if workflow.user_id != user_id {
            return Err(WorkflowServiceError::NotWorkflowOwner);
        }
        Ok(workflow)
    }

    pub async fn create_workflow(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        request: CreateWorkflowRequest,
    ) -> Result<Workflow, WorkflowServiceError> {
        let user_id = parse_user_id(user_id)?;
        let schedule_time = option_string_to_option_datetime(request.schedule_time)
            .map_err(|_| WorkflowServiceError::InvalidScheduleTime)?;

        let workflow = self
            .workflows
            .create(
                db,
                CreateWorkflowParams {
                    user_id,
                    name: request.name,
                    description: request.description,
                    run_type: request.run_type,
                    schedule_time,
                    cron_expression: request.cron_expression,
                    // Working out when a cron workflow next fires belongs to
                    // whatever schedules runs, not to creating the record.
                    cron_next_run: None,
                },
            )
            .await?;
        Ok(workflow)
    }

    pub async fn edit_workflow(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
        request: EditWorkflowRequest,
    ) -> Result<Workflow, WorkflowServiceError> {
        let user_id = parse_user_id(user_id)?;
        let workflow_id = parse_workflow_id(workflow_id)?;

        let mut workflow = self.owned_workflow(db, user_id, workflow_id).await?;

        let schedule_time = option_string_to_option_datetime(request.schedule_time)
            .map_err(|_| WorkflowServiceError::InvalidScheduleTime)?;

        self.workflows
            .update(
                db,
                &mut workflow,
                UpdateWorkflowParams {
                    name: request.name,
                    description: request.description,
                    run_type: request.run_type,
                    schedule_time,
                    cron_expression: request.cron_expression,
                    cron_next_run: None,
                    is_active: request.is_active,
                },
            )
            .await?;

        Ok(workflow)
    }

    pub async fn get_workflow_by_id(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
    ) -> Result<Workflow, WorkflowServiceError> {
        let user_id = parse_user_id(user_id)?;
        let workflow_id = parse_workflow_id(workflow_id)?;
        self.owned_workflow(db, user_id, workflow_id).await
    }

    /// A workflow together with its steps and the edges between them, which is
    /// the whole graph.
    pub async fn workflow_detail(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
    ) -> Result<WorkflowDetailResponse, WorkflowServiceError> {
        let user_id = parse_user_id(user_id)?;
        let workflow_id = parse_workflow_id(workflow_id)?;

        let workflow = self.owned_workflow(db, user_id, workflow_id).await?;
        let tasks = self
            .workflow_tasks
            .find_by_workflow_id(db, workflow_id)
            .await?;
        let edges = self.edges.find_by_workflow_id(db, workflow_id).await?;

        Ok(WorkflowDetailResponse::from_workflow_row(
            workflow, tasks, edges,
        ))
    }

    pub async fn delete_workflow(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
    ) -> Result<(), WorkflowServiceError> {
        let user_id = parse_user_id(user_id)?;
        let workflow_id = parse_workflow_id(workflow_id)?;

        self.owned_workflow(db, user_id, workflow_id).await?;

        // The workflow's steps, edges, and run history go with it, because those
        // relations are declared on the model.
        self.workflows.delete(db, workflow_id).await?;
        Ok(())
    }

    pub async fn list_workflows(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
    ) -> Result<Vec<Workflow>, WorkflowServiceError> {
        let user_id = parse_user_id(user_id)?;
        Ok(self.workflows.find_by_user_id(db, user_id).await?)
    }
}

fn parse_user_id(value: &str) -> Result<Uuid, WorkflowServiceError> {
    Uuid::parse_str(value).map_err(|_| WorkflowServiceError::InvalidUserId(value.to_string()))
}

fn parse_workflow_id(value: &str) -> Result<Uuid, WorkflowServiceError> {
    Uuid::parse_str(value).map_err(|_| WorkflowServiceError::InvalidWorkflowId(value.to_string()))
}
