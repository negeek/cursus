pub mod common;

use actix_web::{http::header, test};
use serde_json::json;

const NONEXISTENT_ID: &str = "00000000-0000-0000-0000-000000000000";

/// Tests are sequential for now
/// TODO: Refactor tests to be independent and run in parallel

#[actix_web::test]
async fn test_create_workflow_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;

    let auth_header = (
        header::AUTHORIZATION,
        format!("Bearer {}", test_tokens.access),
    );
    let test_workflow =
        common::workflow::test_workflow(&db, test_user.id.to_string().clone()).await;
    let test_task = common::task::test_task(&db, test_user.id.to_string().clone()).await;
    let app = common::setup_app(db).await;

    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(
                &common::path::Paths::CREATE_WORKFLOW_TASK
                    .replace("{workflow_id}", test_workflow.id.to_string().as_str()),
            )
            .set_json(&json!({
                "task_id": test_task.id.to_string(),
                "step_name": "Test_Step",
                "task_body": {"data": {"input": "test input"}},
                "task_result_schema": {"data": {"result": {"status": "string", "output": "string"}}},
                "run_position": 1,
                "retry_count": null,
                "retry_delay_secs": null
            }))
            .insert_header(auth_header)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 201);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("id").is_some());
}

#[actix_web::test]
async fn test_get_workflow_task_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&db, test_user.id.to_string()).await;
    let task = common::task::test_task(&db, test_user.id.to_string()).await;
    let wt = common::workflow_task::test_workflow_task(
        &db,
        workflow.id.to_string(),
        task.id.to_string(),
        1,
    )
    .await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW_TASK
                    .replace("{workflow_id}", &workflow.id.to_string())
                    .replace("{workflow_task_id}", &wt.id.to_string()),
            )
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", test_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body.get("id").unwrap().as_str().unwrap(), wt.id.to_string());
}

#[actix_web::test]
async fn test_edit_workflow_task_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&db, test_user.id.to_string()).await;
    let task = common::task::test_task(&db, test_user.id.to_string()).await;
    let wt = common::workflow_task::test_workflow_task(
        &db,
        workflow.id.to_string(),
        task.id.to_string(),
        1,
    )
    .await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::patch()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW_TASK
                    .replace("{workflow_id}", &workflow.id.to_string())
                    .replace("{workflow_task_id}", &wt.id.to_string()),
            )
            .set_json(&json!({"step_name": "Updated_Step"}))
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", test_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body.get("step_name").unwrap().as_str().unwrap(),
        "Updated_Step"
    );
}

#[actix_web::test]
async fn test_delete_workflow_task_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&db, test_user.id.to_string()).await;
    let task = common::task::test_task(&db, test_user.id.to_string()).await;
    let wt = common::workflow_task::test_workflow_task(
        &db,
        workflow.id.to_string(),
        task.id.to_string(),
        1,
    )
    .await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::delete()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW_TASK
                    .replace("{workflow_id}", &workflow.id.to_string())
                    .replace("{workflow_task_id}", &wt.id.to_string()),
            )
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", test_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body.get("message").unwrap().as_str().unwrap(),
        "Workflow task deleted successfully"
    );
}

// --- Failure tests ---

#[actix_web::test]
async fn test_workflow_task_endpoints_unauthenticated() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db).await;
    let list_create_url =
        common::path::Paths::CREATE_WORKFLOW_TASK.replace("{workflow_id}", NONEXISTENT_ID);
    let item_url = common::path::Paths::GET_EDIT_DELETE_WORKFLOW_TASK
        .replace("{workflow_id}", NONEXISTENT_ID)
        .replace("{workflow_task_id}", NONEXISTENT_ID);

    let unauth_status = |r: Result<_, actix_web::Error>| {
        r.map(|resp: actix_web::dev::ServiceResponse| resp.status())
            .unwrap_or_else(|e| e.as_response_error().status_code())
    };

    for status in [
        unauth_status(
            test::try_call_service(
                &app,
                test::TestRequest::post()
                    .uri(&list_create_url)
                    .set_json(&json!({}))
                    .to_request(),
            )
            .await,
        ),
        unauth_status(
            test::try_call_service(&app, test::TestRequest::get().uri(&item_url).to_request())
                .await,
        ),
        unauth_status(
            test::try_call_service(
                &app,
                test::TestRequest::patch()
                    .uri(&item_url)
                    .set_json(&json!({}))
                    .to_request(),
            )
            .await,
        ),
        unauth_status(
            test::try_call_service(
                &app,
                test::TestRequest::delete().uri(&item_url).to_request(),
            )
            .await,
        ),
    ] {
        assert_eq!(status, 401);
    }
}

#[actix_web::test]
async fn test_create_workflow_task_duplicate_step_name() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&db, test_user.id.to_string()).await;
    let task = common::task::test_task(&db, test_user.id.to_string()).await;
    common::workflow_task::test_workflow_task(&db, workflow.id.to_string(), task.id.to_string(), 1)
        .await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(
                &common::path::Paths::CREATE_WORKFLOW_TASK
                    .replace("{workflow_id}", &workflow.id.to_string()),
            )
            .set_json(&json!({
                "task_id": task.id.to_string(),
                "step_name": "Step_1",
                "run_position": 2
            }))
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", test_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 409);
}

#[actix_web::test]
async fn test_create_workflow_task_space_in_step_name() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&db, test_user.id.to_string()).await;
    let task = common::task::test_task(&db, test_user.id.to_string()).await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(
                &common::path::Paths::CREATE_WORKFLOW_TASK
                    .replace("{workflow_id}", &workflow.id.to_string()),
            )
            .set_json(&json!({
                "task_id": task.id.to_string(),
                "step_name": "invalid step name",
                "run_position": 1
            }))
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", test_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_get_workflow_task_wrong_user() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let owner = common::user::test_user(&db, None, None).await;
    let workflow = common::workflow::test_workflow(&db, owner.id.to_string()).await;
    let task = common::task::test_task(&db, owner.id.to_string()).await;
    let wt = common::workflow_task::test_workflow_task(
        &db,
        workflow.id.to_string(),
        task.id.to_string(),
        1,
    )
    .await;
    let other_tokens = common::user::test_tokens(
        &db,
        Some(
            common::user::test_user(
                &db,
                Some("other_user".into()),
                Some("other@test.com".into()),
            )
            .await,
        ),
    )
    .await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW_TASK
                    .replace("{workflow_id}", &workflow.id.to_string())
                    .replace("{workflow_task_id}", &wt.id.to_string()),
            )
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", other_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 403);
}

#[actix_web::test]
async fn test_edit_workflow_task_wrong_user() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let owner = common::user::test_user(&db, None, None).await;
    let workflow = common::workflow::test_workflow(&db, owner.id.to_string()).await;
    let task = common::task::test_task(&db, owner.id.to_string()).await;
    let wt = common::workflow_task::test_workflow_task(
        &db,
        workflow.id.to_string(),
        task.id.to_string(),
        1,
    )
    .await;
    let other_tokens = common::user::test_tokens(
        &db,
        Some(
            common::user::test_user(
                &db,
                Some("other_user".into()),
                Some("other@test.com".into()),
            )
            .await,
        ),
    )
    .await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::patch()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW_TASK
                    .replace("{workflow_id}", &workflow.id.to_string())
                    .replace("{workflow_task_id}", &wt.id.to_string()),
            )
            .set_json(&json!({"step_name": "Hacked_Step"}))
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", other_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 403);
}

#[actix_web::test]
async fn test_delete_workflow_task_wrong_user() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let owner = common::user::test_user(&db, None, None).await;
    let workflow = common::workflow::test_workflow(&db, owner.id.to_string()).await;
    let task = common::task::test_task(&db, owner.id.to_string()).await;
    let wt = common::workflow_task::test_workflow_task(
        &db,
        workflow.id.to_string(),
        task.id.to_string(),
        1,
    )
    .await;
    let other_tokens = common::user::test_tokens(
        &db,
        Some(
            common::user::test_user(
                &db,
                Some("other_user".into()),
                Some("other@test.com".into()),
            )
            .await,
        ),
    )
    .await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::delete()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW_TASK
                    .replace("{workflow_id}", &workflow.id.to_string())
                    .replace("{workflow_task_id}", &wt.id.to_string()),
            )
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", other_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 403);
}
