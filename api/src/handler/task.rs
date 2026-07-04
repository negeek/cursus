use crate::dto::error::ApiError;
use crate::dto::task::{CreateTaskRequest, DeleteTaskResponse, EditTaskRequest, TaskResponse};
use crate::middleware::user::AuthUser;
use crate::{dto::RequestValidateTrait, service::task::TaskService};
use actix_web::{Responder, Result, delete, get, http::StatusCode, patch, post, web};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    get,
    path = "/api/v1/tasks",
    responses(
        (status = 200, description = "List of tasks", body = Vec<TaskResponse>),
        (status = 401, description = "Unauthenticated", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Tasks"
)]
#[get("/tasks")]
async fn get_tasks(
    db: web::Data<DatabaseConnection>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let task_service = TaskService::new();
    let tasks = task_service.get_user_tasks(&**db, &user.id).await?;
    let response_tasks: Vec<TaskResponse> = tasks
        .into_iter()
        .map(|task| TaskResponse::from_task_row(task))
        .collect();
    Ok(web::Json(response_tasks))
}

#[utoipa::path(
    post,
    path = "/api/v1/tasks",
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "Task created", body = TaskResponse),
        (status = 400, description = "Validation error", body = ApiError),
        (status = 401, description = "Unauthenticated", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Tasks"
)]
#[post("/tasks")]
async fn create_task(
    db: web::Data<DatabaseConnection>,
    body: web::Json<CreateTaskRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let _ = &body.validate()?;
    let user_service = TaskService::new();
    let task = user_service
        .create_task(&**db, user.id.clone(), body.into_inner())
        .await?;
    Ok((
        web::Json(TaskResponse::from_task_row(task)),
        StatusCode::CREATED,
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/tasks/{task_id}",
    params(("task_id" = String, Path, description = "Task UUID")),
    responses(
        (status = 200, description = "Task found", body = TaskResponse),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your task", body = ApiError),
        (status = 404, description = "Task not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Tasks"
)]
#[get("/tasks/{task_id}")]
async fn get_task(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String,)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let task_id = path.into_inner().0;
    let task_service = TaskService::new();
    let task = task_service
        .get_task_by_id(&**db, &user.id, task_id)
        .await?;
    Ok(web::Json(TaskResponse::from_task_row(task)))
}

#[utoipa::path(
    patch,
    path = "/api/v1/tasks/{task_id}",
    params(("task_id" = String, Path, description = "Task UUID")),
    request_body = EditTaskRequest,
    responses(
        (status = 200, description = "Task updated", body = TaskResponse),
        (status = 400, description = "Validation error", body = ApiError),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your task", body = ApiError),
        (status = 404, description = "Task not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Tasks"
)]
#[patch("/tasks/{task_id}")]
async fn edit_task(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String,)>,
    body: web::Json<EditTaskRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let task_id = path.into_inner().0;
    let _ = &body.validate()?;
    let user_service = TaskService::new();
    let task = user_service
        .edit_task(&**db, &user.id, task_id, body.into_inner())
        .await?;
    Ok(web::Json(TaskResponse::from_task_row(task)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/tasks/{task_id}",
    params(("task_id" = String, Path, description = "Task UUID")),
    responses(
        (status = 200, description = "Task deleted", body = DeleteTaskResponse),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your task", body = ApiError),
        (status = 404, description = "Task not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Tasks"
)]
#[delete("/tasks/{task_id}")]
async fn delete_task(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String,)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let task_id = path.into_inner().0;
    let user_service = TaskService::new();
    let _ = user_service.delete_task(&**db, &user.id, task_id).await?;
    Ok(web::Json(DeleteTaskResponse {
        message: "Task deleted successfully".to_string(),
    }))
}
