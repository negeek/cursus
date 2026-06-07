use crate::dto::workflow_task::{
    CreateWorkflowTaskRequest, DeleteWorkflowTaskResponse, EditWorkflowTaskRequest,
    WorkflowTaskResponse,
};
use crate::middleware::user::AuthUser;
use crate::{dto::RequestValidateTrait, service::workflow_task::WorkflowTaskService};
use actix_web::{Responder, Result, delete, get, http::StatusCode, patch, post, web};
use sea_orm::DatabaseConnection;

#[post("/workflows/{workflow_id}/tasks")]
async fn create_workflow_task(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String,)>,
    body: web::Json<CreateWorkflowTaskRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let workflow_id = path.into_inner().0;
    let _ = &body.validate()?;
    let service = WorkflowTaskService::new();
    let task = service
        .create_workflow_task(&**db, user.id.clone(), workflow_id, body.into_inner())
        .await?;
    Ok((
        web::Json(WorkflowTaskResponse::from_workflow_task_row(task)),
        StatusCode::CREATED,
    ))
}

#[get("/workflows/{workflow_id}/tasks/{workflow_task_id}")]
async fn get_workflow_task(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String, String)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let (workflow_id, workflow_task_id) = path.into_inner();
    let service = WorkflowTaskService::new();
    let task = service
        .get_workflow_task(&**db, user.id.clone(), workflow_id, workflow_task_id)
        .await?;
    Ok(web::Json(WorkflowTaskResponse::from_workflow_task_row(
        task,
    )))
}

#[patch("/workflows/{workflow_id}/tasks/{workflow_task_id}")]
async fn edit_workflow_task(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String, String)>,
    body: web::Json<EditWorkflowTaskRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let (workflow_id, workflow_task_id) = path.into_inner();
    let _ = &body.validate()?;
    let service = WorkflowTaskService::new();
    let task = service
        .edit_workflow_task(
            &**db,
            user.id.clone(),
            workflow_id,
            workflow_task_id,
            body.into_inner(),
        )
        .await?;
    Ok(web::Json(WorkflowTaskResponse::from_workflow_task_row(
        task,
    )))
}

#[delete("/workflows/{workflow_id}/tasks/{workflow_task_id}")]
async fn delete_workflow_task(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String, String)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let (workflow_id, workflow_task_id) = path.into_inner();
    let service = WorkflowTaskService::new();
    service
        .delete_workflow_task(&**db, user.id.clone(), workflow_id, workflow_task_id)
        .await?;
    Ok(web::Json(DeleteWorkflowTaskResponse {
        message: "Workflow task deleted successfully".to_string(),
    }))
}
