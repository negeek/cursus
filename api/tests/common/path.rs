pub struct Paths;

impl Paths {
    pub const SIGNUP: &'static str = "/api/v1/account/signup";
    pub const SIGNIN: &'static str = "/api/v1/account/signin";
    pub const VERIFY_EMAIL: &'static str = "/api/v1/account/verify_email";
    pub const LOGOUT: &'static str = "/api/v1/account/logout";
    pub const REFRESH_ACCESS: &'static str = "/api/v1/account/refresh_access";
    pub const CREATE_LIST_TASK: &'static str = "/api/v1/tasks";
    pub const GET_EDIT_DELETE_TASK: &'static str = "/api/v1/tasks/{task_id}";
    pub const CREATE_LIST_WORKFLOW: &'static str = "/api/v1/workflows";
    pub const GET_EDIT_DELETE_WORKFLOW: &'static str = "/api/v1/workflows/{workflow_id}";
    pub const GET_WORKFLOW_DETAIL: &'static str = "/api/v1/workflows/{workflow_id}/detail";
    pub const CREATE_WORKFLOW_TASK: &'static str = "/api/v1/workflows/{workflow_id}/tasks";
    pub const GET_EDIT_DELETE_WORKFLOW_TASK: &'static str =
        "/api/v1/workflows/{workflow_id}/tasks/{workflow_task_id}";
    pub const CREATE_WORKFLOW_TASK_EDGE: &'static str = "/api/v1/workflows/{workflow_id}/edges";
    pub const EDIT_DELETE_WORKFLOW_TASK_EDGE: &'static str =
        "/api/v1/workflows/{workflow_id}/edges/{edge_id}";
}
