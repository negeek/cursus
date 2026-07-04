use crate::dto::error::ApiError;
use crate::dto::workflow_task_edge::{
    DeleteWorkflowTaskEdgeResponse, WorkflowTaskEdgeRequest, WorkflowTaskEdgeResponse,
};
use crate::middleware::user::AuthUser;
use crate::{dto::RequestValidateTrait, service::workflow_task_edge::WorkflowTaskEdgeService};
use actix_web::{Responder, Result, delete, http::StatusCode, patch, post, web};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    post,
    path = "/api/v1/workflows/{workflow_id}/edges",
    params(("workflow_id" = String, Path, description = "Workflow UUID")),
    request_body = WorkflowTaskEdgeRequest,
    responses(
        (status = 201, description = "Edge created", body = WorkflowTaskEdgeResponse),
        (status = 400, description = "Self-loop or validation error", body = ApiError),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow", body = ApiError),
        (status = 404, description = "Workflow or task not found", body = ApiError),
        (status = 409, description = "Duplicate edge", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflow Edges"
)]
#[post("/workflows/{workflow_id}/edges")]
async fn create_workflow_task_edge(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String,)>,
    body: web::Json<WorkflowTaskEdgeRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let workflow_id = path.into_inner().0;
    let _ = &body.validate()?;
    let service = WorkflowTaskEdgeService::new();
    let edge = service
        .create_workflow_task_edge(&**db, user.id.clone(), workflow_id, body.into_inner())
        .await?;
    Ok((
        web::Json(WorkflowTaskEdgeResponse::from_workflow_task_edge_row(edge)),
        StatusCode::CREATED,
    ))
}

#[utoipa::path(
    patch,
    path = "/api/v1/workflows/{workflow_id}/edges/{edge_id}",
    params(
        ("workflow_id" = String, Path, description = "Workflow UUID"),
        ("edge_id" = String, Path, description = "Edge UUID"),
    ),
    request_body = WorkflowTaskEdgeRequest,
    responses(
        (status = 200, description = "Edge updated", body = WorkflowTaskEdgeResponse),
        (status = 400, description = "Self-loop or validation error", body = ApiError),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow", body = ApiError),
        (status = 404, description = "Edge not found", body = ApiError),
        (status = 409, description = "Duplicate edge", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflow Edges"
)]
#[patch("/workflows/{workflow_id}/edges/{edge_id}")]
async fn edit_workflow_task_edge(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String, String)>,
    body: web::Json<WorkflowTaskEdgeRequest>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let (workflow_id, edge_id) = path.into_inner();
    let _ = &body.validate()?;
    let service = WorkflowTaskEdgeService::new();
    let edge = service
        .edit_workflow_task_edge(
            &**db,
            user.id.clone(),
            workflow_id,
            edge_id,
            body.into_inner(),
        )
        .await?;
    Ok(web::Json(
        WorkflowTaskEdgeResponse::from_workflow_task_edge_row(edge),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/workflows/{workflow_id}/edges/{edge_id}",
    params(
        ("workflow_id" = String, Path, description = "Workflow UUID"),
        ("edge_id" = String, Path, description = "Edge UUID"),
    ),
    responses(
        (status = 200, description = "Edge deleted", body = DeleteWorkflowTaskEdgeResponse),
        (status = 401, description = "Unauthenticated", body = ApiError),
        (status = 403, description = "Not your workflow", body = ApiError),
        (status = 404, description = "Edge not found", body = ApiError),
    ),
    security(("bearer_auth" = [])),
    tag = "Workflow Edges"
)]
#[delete("/workflows/{workflow_id}/edges/{edge_id}")]
async fn delete_workflow_task_edge(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String, String)>,
    user: web::ReqData<AuthUser>,
) -> Result<impl Responder> {
    let (workflow_id, edge_id) = path.into_inner();
    let service = WorkflowTaskEdgeService::new();
    service
        .delete_workflow_task_edge(&**db, user.id.clone(), workflow_id, edge_id)
        .await?;
    Ok(web::Json(DeleteWorkflowTaskEdgeResponse {
        message: "Edge deleted successfully".to_string(),
    }))
}
