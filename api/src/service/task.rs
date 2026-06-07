use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

use crate::db::entity::task::{ActiveModel as TaskOp, Model as TaskRow};
use crate::db::repository::RepositoryTrait;
use crate::db::repository::task::TaskRepository;
use crate::dto::error::task::TaskServiceError;
use crate::dto::task::{CreateTaskRequest, EditTaskRequest};
use crate::util::type_conv::json_to_value;

pub struct TaskService {
    task_repository: TaskRepository,
}

impl TaskService {
    pub fn new() -> Self {
        Self {
            task_repository: TaskRepository {},
        }
    }

    pub async fn create_task(
        &self,
        db: &impl ConnectionTrait,
        user_id: String,
        task_req: CreateTaskRequest,
    ) -> Result<TaskRow, TaskServiceError> {
        let user_id =
            uuid::Uuid::parse_str(&user_id).map_err(|_| TaskServiceError::ServiceError)?;
        let result_schema =
            json_to_value(task_req.result_schema).map_err(|_| TaskServiceError::ServiceError)?;
        let task_body = json_to_value(task_req.body).map_err(|_| TaskServiceError::ServiceError)?;
        let task_data = TaskOp {
            id: Default::default(),
            user_id: Set(user_id),
            name: Set(task_req.name),
            description: Set(task_req.description),
            endpoint: Set(task_req.endpoint),
            result_schema: Set(result_schema),
            body: Set(task_body),
            ..Default::default()
        };
        let created_task = self.task_repository.create(db, task_data).await?;
        Ok(created_task)
    }

    pub async fn edit_task(
        &self,
        db: &impl ConnectionTrait,
        user_id: &String,
        task_id: String,
        task_req: EditTaskRequest,
    ) -> Result<TaskRow, TaskServiceError> {
        let task_id =
            uuid::Uuid::parse_str(&task_id).map_err(|_| TaskServiceError::ServiceError)?;
        let existing_task = match self.task_repository.find_by_uuid(db, &task_id).await? {
            Some(t) => t,
            None => return Err(TaskServiceError::TaskNotFound),
        };
        if existing_task.user_id.to_string() != *user_id {
            return Err(TaskServiceError::Unauthorized);
        }
        let result_schema =
            json_to_value(task_req.result_schema).map_err(|_| TaskServiceError::ServiceError)?;
        let task_body = json_to_value(task_req.body).map_err(|_| TaskServiceError::ServiceError)?;
        let mut existing_task_op: TaskOp = existing_task.into();
        if task_req.name.is_some() {
            existing_task_op.name = Set(task_req.name.unwrap());
        }
        if task_req.description.is_some() {
            existing_task_op.description = Set(task_req.description);
        }
        if task_req.endpoint.is_some() {
            existing_task_op.endpoint = Set(task_req.endpoint.unwrap());
        }
        if task_body.is_some() {
            existing_task_op.body = Set(task_body);
        }
        if result_schema.is_some() {
            existing_task_op.result_schema = Set(result_schema);
        }
        let updated_task = existing_task_op.update(db).await?;
        Ok(updated_task)
    }

    pub async fn delete_task(
        &self,
        db: &impl ConnectionTrait,
        user_id: &String,
        task_id: String,
    ) -> Result<(), TaskServiceError> {
        let task_id =
            uuid::Uuid::parse_str(&task_id).map_err(|_| TaskServiceError::ServiceError)?;
        let existing_task = match self.task_repository.find_by_uuid(db, &task_id).await? {
            Some(t) => t,
            None => return Err(TaskServiceError::TaskNotFound),
        };
        if existing_task.user_id.to_string() != *user_id {
            return Err(TaskServiceError::Unauthorized);
        }
        let task_op: TaskOp = existing_task.into();
        self.task_repository.delete(db, task_op).await?;
        Ok(())
    }

    pub async fn get_user_tasks(
        &self,
        db: &impl ConnectionTrait,
        user_id: &String,
    ) -> Result<Vec<TaskRow>, TaskServiceError> {
        let user_id =
            uuid::Uuid::parse_str(&user_id).map_err(|_| TaskServiceError::ServiceError)?;
        let tasks = self.task_repository.find_by_user_id(db, &user_id).await?;
        Ok(tasks)
    }

    pub async fn get_task_by_id(
        &self,
        db: &impl ConnectionTrait,
        user_id: &String,
        task_id: String,
    ) -> Result<TaskRow, TaskServiceError> {
        let task_id =
            uuid::Uuid::parse_str(&task_id).map_err(|_| TaskServiceError::ServiceError)?;
        let task = match self.task_repository.find_by_uuid(db, &task_id).await? {
            Some(t) => t,
            None => return Err(TaskServiceError::TaskNotFound),
        };
        if task.user_id.to_string() != *user_id {
            return Err(TaskServiceError::Unauthorized);
        }
        Ok(task)
    }
}
