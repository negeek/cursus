use crate::dto::task::{CreateTaskRequest, DeleteTaskResponse, EditTaskRequest, TaskResponse};
use crate::middleware::user::AuthUser;
use crate::{dto::RequestValidateTrait, service::task::TaskService};
use actix_web::{Responder, Result, delete, get, http::StatusCode, patch, post, web};
use sea_orm::DatabaseConnection;

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
