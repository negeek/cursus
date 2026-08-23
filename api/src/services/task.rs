use uuid::Uuid;

use crate::dtos::error::task::TaskServiceError;
use crate::dtos::task::{CreateTaskRequest, EditTaskRequest};
use crate::models::Task;
use crate::repositories::task::{CreateTaskParams, TaskRepository, UpdateTaskParams};
use crate::util::type_conv::{json_to_value, parse_uuid};

pub struct TaskService {
    tasks: TaskRepository,
}

impl Default for TaskService {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskService {
    pub fn new() -> Self {
        Self {
            tasks: TaskRepository,
        }
    }

    /// Loads a task and confirms the caller owns it.
    ///
    /// Every single task operation needs this pair of checks, and getting the
    /// order wrong leaks information: answering "forbidden" for a task that does
    /// not exist tells the caller that someone else's task has that id. Both
    /// misses answer "not found" instead.
    async fn owned_task(
        &self,
        db: &mut toasty::Db,
        user_id: Uuid,
        task_id: Uuid,
    ) -> Result<Task, TaskServiceError> {
        let task = self
            .tasks
            .find_by_id(db, task_id)
            .await?
            .ok_or(TaskServiceError::TaskNotFound)?;

        if task.user_id != user_id {
            return Err(TaskServiceError::NotTaskOwner);
        }
        Ok(task)
    }

    pub async fn create_task(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        request: CreateTaskRequest,
    ) -> Result<Task, TaskServiceError> {
        let user_id = parse_uuid(user_id)
            .map_err(|_| TaskServiceError::InvalidUserId(user_id.to_string()))?;

        let result_schema = json_to_value(request.result_schema).map_err(|_| {
            TaskServiceError::InvalidJsonField {
                field: "result_schema",
            }
        })?;
        let body = json_to_value(request.body)
            .map_err(|_| TaskServiceError::InvalidJsonField { field: "body" })?;

        let task = self
            .tasks
            .create(
                db,
                CreateTaskParams {
                    user_id,
                    name: request.name,
                    description: request.description,
                    endpoint: request.endpoint,
                    result_schema,
                    body,
                },
            )
            .await?;
        Ok(task)
    }

    pub async fn edit_task(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        task_id: &str,
        request: EditTaskRequest,
    ) -> Result<Task, TaskServiceError> {
        let user_id = parse_uuid(user_id)
            .map_err(|_| TaskServiceError::InvalidUserId(user_id.to_string()))?;
        let task_id = parse_uuid(task_id)
            .map_err(|_| TaskServiceError::InvalidTaskId(task_id.to_string()))?;

        let mut task = self.owned_task(db, user_id, task_id).await?;

        let result_schema = json_to_value(request.result_schema).map_err(|_| {
            TaskServiceError::InvalidJsonField {
                field: "result_schema",
            }
        })?;
        let body = json_to_value(request.body)
            .map_err(|_| TaskServiceError::InvalidJsonField { field: "body" })?;

        self.tasks
            .update(
                db,
                &mut task,
                UpdateTaskParams {
                    name: request.name,
                    description: request.description,
                    endpoint: request.endpoint,
                    result_schema,
                    body,
                },
            )
            .await?;

        Ok(task)
    }

    pub async fn delete_task(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        task_id: &str,
    ) -> Result<(), TaskServiceError> {
        let user_id = parse_uuid(user_id)
            .map_err(|_| TaskServiceError::InvalidUserId(user_id.to_string()))?;
        let task_id = parse_uuid(task_id)
            .map_err(|_| TaskServiceError::InvalidTaskId(task_id.to_string()))?;

        self.owned_task(db, user_id, task_id).await?;
        self.tasks.delete(db, task_id).await?;
        Ok(())
    }

    pub async fn get_user_tasks(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
    ) -> Result<Vec<Task>, TaskServiceError> {
        let user_id = parse_uuid(user_id)
            .map_err(|_| TaskServiceError::InvalidUserId(user_id.to_string()))?;
        Ok(self.tasks.find_by_user_id(db, user_id).await?)
    }

    pub async fn get_task_by_id(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        task_id: &str,
    ) -> Result<Task, TaskServiceError> {
        let user_id = parse_uuid(user_id)
            .map_err(|_| TaskServiceError::InvalidUserId(user_id.to_string()))?;
        let task_id = parse_uuid(task_id)
            .map_err(|_| TaskServiceError::InvalidTaskId(task_id.to_string()))?;
        self.owned_task(db, user_id, task_id).await
    }
}
