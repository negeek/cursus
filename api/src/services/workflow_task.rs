use uuid::Uuid;

use crate::dtos::error::workflow_task::WorkflowTaskServiceError as Error;
use crate::dtos::workflow_task::{CreateWorkflowTaskRequest, EditWorkflowTaskRequest};
use crate::models::WorkflowTask;
use crate::repositories::is_unique_violation;
use crate::repositories::task::TaskRepository;
use crate::repositories::workflow::WorkflowRepository;
use crate::repositories::workflow_task::{
    CreateWorkflowTaskParams, UpdateWorkflowTaskParams, WorkflowTaskRepository,
};
use crate::util::type_conv::{json_to_value, option_u32_to_option_i32};

pub struct WorkflowTaskService {
    workflows: WorkflowRepository,
    workflow_tasks: WorkflowTaskRepository,
    tasks: TaskRepository,
}

impl Default for WorkflowTaskService {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowTaskService {
    pub fn new() -> Self {
        Self {
            workflows: WorkflowRepository,
            workflow_tasks: WorkflowTaskRepository,
            tasks: TaskRepository,
        }
    }

    async fn assert_owns_workflow(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: Uuid,
    ) -> Result<(), Error> {
        let workflow = self
            .workflows
            .find_by_id(db, workflow_id)
            .await?
            .ok_or(Error::WorkflowNotFound)?;

        if workflow.user_id.to_string() != user_id {
            return Err(Error::NotWorkflowOwner);
        }
        Ok(())
    }

    /// Loads a step, but only if it belongs to the workflow named.
    async fn step_in_workflow(
        &self,
        db: &mut toasty::Db,
        workflow_id: Uuid,
        workflow_task_id: Uuid,
    ) -> Result<WorkflowTask, Error> {
        self.workflow_tasks
            .find_in_workflow(db, workflow_id, workflow_task_id)
            .await?
            .ok_or(Error::WorkflowTaskNotFound)
    }

    pub async fn create_workflow_task(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
        request: CreateWorkflowTaskRequest,
    ) -> Result<WorkflowTask, Error> {
        let workflow_id = parse_id(workflow_id)?;
        let task_id = parse_id(&request.task_id)?;

        self.assert_owns_workflow(db, user_id, workflow_id).await?;

        // The task being placed has to belong to the same person, otherwise a
        // workflow could call out to someone else's endpoint.
        let task = self
            .tasks
            .find_by_id(db, task_id)
            .await?
            .ok_or(Error::TaskNotFound)?;
        if task.user_id.to_string() != user_id {
            return Err(Error::NotTaskOwner);
        }

        if self
            .workflow_tasks
            .find_by_step_name(db, workflow_id, &request.step_name)
            .await?
            .is_some()
        {
            return Err(Error::DuplicateStepName(request.step_name));
        }

        let task_result_schema =
            json_to_value(request.task_result_schema).map_err(|_| Error::InvalidJsonField {
                field: "task_result_schema",
            })?;
        let task_body = json_to_value(request.task_body)
            .map_err(|_| Error::InvalidJsonField { field: "task_body" })?;

        let step_name = request.step_name.clone();
        self.workflow_tasks
            .create(
                db,
                CreateWorkflowTaskParams {
                    workflow_id,
                    task_id,
                    step_name: request.step_name,
                    task_body,
                    task_result_schema,
                    run_position: request.run_position as i32,
                    retry_count: option_u32_to_option_i32(request.retry_count),
                    retry_delay_secs: option_u32_to_option_i32(request.retry_delay_secs),
                },
            )
            .await
            .map_err(|e| {
                // The name check above can lose a race with a concurrent create.
                // The unique index decides, and means the same thing.
                if is_unique_violation(&e) {
                    Error::DuplicateStepName(step_name)
                } else {
                    Error::Database(e)
                }
            })
    }

    pub async fn edit_workflow_task(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
        workflow_task_id: &str,
        request: EditWorkflowTaskRequest,
    ) -> Result<WorkflowTask, Error> {
        let workflow_id = parse_id(workflow_id)?;
        let workflow_task_id = parse_id(workflow_task_id)?;

        self.assert_owns_workflow(db, user_id, workflow_id).await?;
        let mut workflow_task = self
            .step_in_workflow(db, workflow_id, workflow_task_id)
            .await?;

        // Renaming onto a name another step already holds is a conflict, but
        // keeping the current name is not.
        if let Some(step_name) = &request.step_name
            && let Some(existing) = self
                .workflow_tasks
                .find_by_step_name(db, workflow_id, step_name)
                .await?
            && existing.id != workflow_task.id
        {
            return Err(Error::DuplicateStepName(step_name.clone()));
        }

        let task_result_schema =
            json_to_value(request.task_result_schema).map_err(|_| Error::InvalidJsonField {
                field: "task_result_schema",
            })?;
        let task_body = json_to_value(request.task_body)
            .map_err(|_| Error::InvalidJsonField { field: "task_body" })?;

        self.workflow_tasks
            .update(
                db,
                &mut workflow_task,
                UpdateWorkflowTaskParams {
                    step_name: request.step_name,
                    task_body,
                    task_result_schema,
                    run_position: request.run_position.map(|p| p as i32),
                    retry_count: option_u32_to_option_i32(request.retry_count),
                    retry_delay_secs: option_u32_to_option_i32(request.retry_delay_secs),
                },
            )
            .await?;

        Ok(workflow_task)
    }

    pub async fn get_workflow_task(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
        workflow_task_id: &str,
    ) -> Result<WorkflowTask, Error> {
        let workflow_id = parse_id(workflow_id)?;
        let workflow_task_id = parse_id(workflow_task_id)?;

        self.assert_owns_workflow(db, user_id, workflow_id).await?;
        self.step_in_workflow(db, workflow_id, workflow_task_id)
            .await
    }

    pub async fn delete_workflow_task(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
        workflow_task_id: &str,
    ) -> Result<(), Error> {
        let workflow_id = parse_id(workflow_id)?;
        let workflow_task_id = parse_id(workflow_task_id)?;

        self.assert_owns_workflow(db, user_id, workflow_id).await?;
        self.step_in_workflow(db, workflow_id, workflow_task_id)
            .await?;

        // The step's edges go with it, in both directions, because WorkflowTask
        // declares the paired has_many relations. That happens for any delete of
        // a step, not just this one, so there is no path that can leave an edge
        // pointing at a step which no longer exists.
        self.workflow_tasks.delete(db, workflow_task_id).await?;
        Ok(())
    }
}

fn parse_id(value: &str) -> Result<Uuid, Error> {
    Uuid::parse_str(value).map_err(|_| Error::InvalidId(value.to_string()))
}
