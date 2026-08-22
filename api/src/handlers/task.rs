use crate::dtos::error::ApiError;
use crate::dtos::task::{CreateTaskRequest, DeleteTaskResponse, EditTaskRequest, TaskResponse};
use crate::middlewares::user::AuthUser;
use crate::{dtos::RequestValidateTrait, services::task::TaskService};
use actix_web::{Responder, Result, delete, get, http::StatusCode, patch, post, web};
use crate::state::AppState;

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
pub async fn get_tasks(
    state: web::Data<AppState>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let mut db = state.db.clone();
    let task_service = TaskService::new();
    let tasks = task_service.get_user_tasks(&mut db, &user.id).await?;
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
pub async fn create_task(
    state: web::Data<AppState>,
    body: web::Json<CreateTaskRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let mut db = state.db.clone();
    let _ = &body.validate()?;
    let user_service = TaskService::new();
    let task = user_service
        .create_task(&mut db, user.id.clone(), body.into_inner())
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
pub async fn get_task(
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let mut db = state.db.clone();
    let task_id = path.into_inner().0;
    let task_service = TaskService::new();
    let task = task_service
        .get_task_by_id(&mut db, &user.id, task_id)
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
pub async fn edit_task(
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
    body: web::Json<EditTaskRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let mut db = state.db.clone();
    let task_id = path.into_inner().0;
    let _ = &body.validate()?;
    let user_service = TaskService::new();
    let task = user_service
        .edit_task(&mut db, &user.id, task_id, body.into_inner())
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
pub async fn delete_task(
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let mut db = state.db.clone();
    let task_id = path.into_inner().0;
    let user_service = TaskService::new();
    let _ = user_service.delete_task(&mut db, &user.id, task_id).await?;
    Ok(web::Json(DeleteTaskResponse {
        message: "Task deleted successfully".to_string(),
    }))
}
