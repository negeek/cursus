use crate::dto::workflow_task_edge::{
    DeleteWorkflowTaskEdgeResponse, WorkflowTaskEdgeRequest, WorkflowTaskEdgeResponse,
};
use crate::middleware::user::AuthUser;
use crate::{dto::RequestValidateTrait, service::workflow_task_edge::WorkflowTaskEdgeService};
use actix_web::{Responder, Result, delete, http::StatusCode, patch, post, web};
use sea_orm::DatabaseConnection;

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
