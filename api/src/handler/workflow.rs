use crate::dto::workflow::{
    CreateWorkflowRequest, DeleteWorkflowResponse, EditWorkflowRequest, WorkflowResponse,
};
use crate::middleware::user::AuthUser;
use crate::{dto::RequestValidateTrait, service::workflow::WorkflowService};
use actix_web::{Responder, Result, delete, get, http::StatusCode, patch, post, web};
use sea_orm::DatabaseConnection;

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
