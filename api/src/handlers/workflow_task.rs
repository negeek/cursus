use crate::dtos::error::ApiError;
use crate::dtos::workflow_task::{
    CreateWorkflowTaskRequest, DeleteWorkflowTaskResponse, EditWorkflowTaskRequest,
    WorkflowTaskResponse,
};
use crate::middlewares::user::AuthUser;
use crate::state::AppState;
use crate::{dtos::RequestValidateTrait, services::workflow_task::WorkflowTaskService};
use actix_web::{Responder, Result, delete, get, http::StatusCode, patch, post, web};

#[utoipa::path(
    post,
    path = "/api/v1/workflows/{workflow_id}/tasks",
    params(("workflow_id" = String, Path, description = "Workflow UUID")),
    request_body = CreateWorkflowTaskRequest,
    responses(
        (status = 201, description = "Workflow task created", body = WorkflowTaskResponse),
        (status = 400, description = "Validation error", body = ApiError),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow or task", body = ApiError),
        (status = 404, description = "Workflow or task not found", body = ApiError),
        (status = 409, description = "Step name already exists", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflow Tasks"
)]
#[post("/workflows/{workflow_id}/tasks")]
pub async fn create_workflow_task(
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
    body: web::Json<CreateWorkflowTaskRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let mut db = state.db.clone();
    let workflow_id = path.into_inner().0;
    let _ = &body.validate()?;
    let service = WorkflowTaskService::new();
    let task = service
        .create_workflow_task(&mut db, &user.id, &workflow_id, body.into_inner())
        .await?;
    Ok((
        web::Json(WorkflowTaskResponse::from_workflow_task_row(task)),
        StatusCode::CREATED,
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/workflows/{workflow_id}/tasks/{workflow_task_id}",
    params(
        ("workflow_id" = String, Path, description = "Workflow UUID"),
        ("workflow_task_id" = String, Path, description = "Workflow task UUID"),
    ),
    responses(
        (status = 200, description = "Workflow task found", body = WorkflowTaskResponse),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow", body = ApiError),
        (status = 404, description = "Workflow task not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflow Tasks"
)]
#[get("/workflows/{workflow_id}/tasks/{workflow_task_id}")]
pub async fn get_workflow_task(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let mut db = state.db.clone();
    let (workflow_id, workflow_task_id) = path.into_inner();
    let service = WorkflowTaskService::new();
    let task = service
        .get_workflow_task(&mut db, &user.id, &workflow_id, &workflow_task_id)
        .await?;
    Ok(web::Json(WorkflowTaskResponse::from_workflow_task_row(
        task,
    )))
}

#[utoipa::path(
    patch,
    path = "/api/v1/workflows/{workflow_id}/tasks/{workflow_task_id}",
    params(
        ("workflow_id" = String, Path, description = "Workflow UUID"),
        ("workflow_task_id" = String, Path, description = "Workflow task UUID"),
    ),
    request_body = EditWorkflowTaskRequest,
    responses(
        (status = 200, description = "Workflow task updated", body = WorkflowTaskResponse),
        (status = 400, description = "Validation error", body = ApiError),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow", body = ApiError),
        (status = 404, description = "Workflow task not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflow Tasks"
)]
#[patch("/workflows/{workflow_id}/tasks/{workflow_task_id}")]
pub async fn edit_workflow_task(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    body: web::Json<EditWorkflowTaskRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let mut db = state.db.clone();
    let (workflow_id, workflow_task_id) = path.into_inner();
    let _ = &body.validate()?;
    let service = WorkflowTaskService::new();
    let task = service
        .edit_workflow_task(
            &mut db,
            &user.id,
            &workflow_id,
            &workflow_task_id,
            body.into_inner(),
        )
        .await?;
    Ok(web::Json(WorkflowTaskResponse::from_workflow_task_row(
        task,
    )))
}

#[utoipa::path(
    delete,
    path = "/api/v1/workflows/{workflow_id}/tasks/{workflow_task_id}",
    params(
        ("workflow_id" = String, Path, description = "Workflow UUID"),
        ("workflow_task_id" = String, Path, description = "Workflow task UUID"),
    ),
    responses(
        (status = 200, description = "Workflow task deleted", body = DeleteWorkflowTaskResponse),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow", body = ApiError),
        (status = 404, description = "Workflow task not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflow Tasks"
)]
#[delete("/workflows/{workflow_id}/tasks/{workflow_task_id}")]
pub async fn delete_workflow_task(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let mut db = state.db.clone();
    let (workflow_id, workflow_task_id) = path.into_inner();
    let service = WorkflowTaskService::new();
    service
        .delete_workflow_task(&mut db, &user.id, &workflow_id, &workflow_task_id)
        .await?;
    Ok(web::Json(DeleteWorkflowTaskResponse {
        message: "Workflow task deleted successfully".to_string(),
    }))
}
