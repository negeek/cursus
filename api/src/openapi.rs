use crate::dtos::{
    error::ApiError,
    task::{
        CreateTaskRequest, DeleteTaskResponse, EditTaskRequest, TaskBody, TaskResponse,
        TaskResultSchema,
    },
    user::{
        LogoutRequest, LogoutResponse, RefreshAccessResponse, RefressAccessRequest, SignInRequest,
        SignInResponse, SignUpRequest, SignUpResponse, VerifyEmailRequest, VerifyEmailResponse,
    },
    workflow::{
        CreateWorkflowRequest, DeleteWorkflowResponse, EditWorkflowRequest, WorkflowDetailResponse,
        WorkflowResponse,
    },
    workflow_task::{
        CreateWorkflowTaskRequest, DeleteWorkflowTaskResponse, EditWorkflowTaskRequest,
        WorkflowTaskResponse,
    },
    workflow_task_edge::{
        DeleteWorkflowTaskEdgeResponse, WorkflowTaskEdgeRequest, WorkflowTaskEdgeResponse,
    },
};
use crate::handlers::{task, user, workflow, workflow_task, workflow_task_edge};
use utoipa::{
    OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Cursus API",
        version = "0.1.0",
        description = "DAG-based workflow orchestration service"
    ),
    paths(
        user::sign_up,
        user::signin,
        user::verify_email,
        user::logout,
        user::refresh_access_token,
        task::get_tasks,
        task::create_task,
        task::get_task,
        task::edit_task,
        task::delete_task,
        workflow::get_workflows,
        workflow::create_workflow,
        workflow::get_workflow,
        workflow::get_workflow_detail,
        workflow::edit_workflow,
        workflow::delete_workflow,
        workflow_task::create_workflow_task,
        workflow_task::get_workflow_task,
        workflow_task::edit_workflow_task,
        workflow_task::delete_workflow_task,
        workflow_task_edge::create_workflow_task_edge,
        workflow_task_edge::edit_workflow_task_edge,
        workflow_task_edge::delete_workflow_task_edge,
    ),
    components(schemas(
        ApiError,
        SignUpRequest, SignUpResponse,
        SignInRequest, SignInResponse,
        VerifyEmailRequest, VerifyEmailResponse,
        LogoutRequest, LogoutResponse,
        RefressAccessRequest, RefreshAccessResponse,
        TaskBody, TaskResultSchema,
        CreateTaskRequest, EditTaskRequest, TaskResponse, DeleteTaskResponse,
        CreateWorkflowRequest, EditWorkflowRequest,
        WorkflowResponse, WorkflowDetailResponse, DeleteWorkflowResponse,
        CreateWorkflowTaskRequest, EditWorkflowTaskRequest,
        WorkflowTaskResponse, DeleteWorkflowTaskResponse,
        WorkflowTaskEdgeRequest, WorkflowTaskEdgeResponse, DeleteWorkflowTaskEdgeResponse,
    )),
    modifiers(&BearerAuth),
    tags(
        (name = "Auth", description = "Sign up, sign in, verify, logout, refresh"),
        (name = "Tasks", description = "Reusable HTTP task definitions"),
        (name = "Workflows", description = "DAG workflow definitions"),
        (name = "Workflow Tasks", description = "Nodes in the workflow DAG"),
        (name = "Workflow Edges", description = "Directed edges between workflow tasks"),
    )
)]
pub struct ApiDoc;

struct BearerAuth;

impl utoipa::Modify for BearerAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
