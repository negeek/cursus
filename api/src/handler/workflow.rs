use crate::dto::error::ApiError;
use crate::dto::workflow::{
    CreateWorkflowRequest, DeleteWorkflowResponse, EditWorkflowRequest, WorkflowDetailResponse,
    WorkflowResponse,
};
use crate::middleware::user::AuthUser;
use crate::{dto::RequestValidateTrait, service::workflow::WorkflowService};
use actix_web::{Responder, Result, delete, get, http::StatusCode, patch, post, web};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    get,
    path = "/api/v1/workflows",
    responses(
        (status = 200, description = "List of workflows", body = Vec<WorkflowResponse>),
        (status = 401, description = "Unauthenticated", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflows"
)]
#[get("/workflows")]
async fn get_workflows(
    db: web::Data<DatabaseConnection>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let workflow_service = WorkflowService::new();
    let workflows = workflow_service.list_workflows(&**db, &user.id).await?;
    let response: Vec<WorkflowResponse> = workflows
        .into_iter()
        .map(WorkflowResponse::from_workflow_row)
        .collect();
    Ok(web::Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/workflows",
    request_body = CreateWorkflowRequest,
    responses(
        (status = 201, description = "Workflow created", body = WorkflowResponse),
        (status = 400, description = "Validation error", body = ApiError),
        (status = 401, description = "Unauthenticated", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflows"
)]
#[post("/workflows")]
async fn create_workflow(
    db: web::Data<DatabaseConnection>,
    body: web::Json<CreateWorkflowRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let _ = &body.validate()?;
    let workflow_service = WorkflowService::new();
    let workflow = workflow_service
        .create_workflow(&**db, user.id.clone(), body.into_inner())
        .await?;
    Ok((
        web::Json(WorkflowResponse::from_workflow_row(workflow)),
        StatusCode::CREATED,
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/workflows/{workflow_id}",
    params(("workflow_id" = String, Path, description = "Workflow UUID")),
    responses(
        (status = 200, description = "Workflow found", body = WorkflowResponse),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow", body = ApiError),
        (status = 404, description = "Workflow not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflows"
)]
#[get("/workflows/{workflow_id}")]
async fn get_workflow(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String,)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let workflow_id = path.into_inner().0;
    let workflow_service = WorkflowService::new();
    let workflow = workflow_service
        .get_workflow_by_id(&**db, &user.id, workflow_id)
        .await?;
    Ok(web::Json(WorkflowResponse::from_workflow_row(workflow)))
}

#[utoipa::path(
    get,
    path = "/api/v1/workflows/{workflow_id}/detail",
    params(("workflow_id" = String, Path, description = "Workflow UUID")),
    responses(
        (status = 200, description = "Workflow with tasks and edges", body = WorkflowDetailResponse),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow", body = ApiError),
        (status = 404, description = "Workflow not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflows"
)]
#[get("/workflows/{workflow_id}/detail")]
async fn get_workflow_detail(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String,)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let workflow_id = path.into_inner().0;
    let workflow_service = WorkflowService::new();
    let detail = workflow_service
        .workflow_detail(&**db, &user.id, workflow_id)
        .await?;
    Ok(web::Json(detail))
}

#[utoipa::path(
    patch,
    path = "/api/v1/workflows/{workflow_id}",
    params(("workflow_id" = String, Path, description = "Workflow UUID")),
    request_body = EditWorkflowRequest,
    responses(
        (status = 200, description = "Workflow updated", body = WorkflowResponse),
        (status = 400, description = "Validation error", body = ApiError),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow", body = ApiError),
        (status = 404, description = "Workflow not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflows"
)]
#[patch("/workflows/{workflow_id}")]
async fn edit_workflow(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String,)>,
    body: web::Json<EditWorkflowRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let workflow_id = path.into_inner().0;
    let _ = &body.validate()?;
    let workflow_service = WorkflowService::new();
    let workflow = workflow_service
        .edit_workflow(&**db, &user.id, workflow_id, body.into_inner())
        .await?;
    Ok(web::Json(WorkflowResponse::from_workflow_row(workflow)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/workflows/{workflow_id}",
    params(("workflow_id" = String, Path, description = "Workflow UUID")),
    responses(
        (status = 200, description = "Workflow deleted", body = DeleteWorkflowResponse),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow", body = ApiError),
        (status = 404, description = "Workflow not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflows"
)]
#[delete("/workflows/{workflow_id}")]
async fn delete_workflow(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String,)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let workflow_id = path.into_inner().0;
    let workflow_service = WorkflowService::new();
    workflow_service
        .delete_workflow(&**db, &user.id, workflow_id)
        .await?;
    Ok(web::Json(DeleteWorkflowResponse {
        message: "Workflow deleted successfully".to_string(),
    }))
}
