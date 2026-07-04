pub mod common;

use actix_web::{http::header, test};
use serde_json::json;

const NONEXISTENT_ID: &str = "00000000-0000-0000-0000-000000000000";

/// Tests are sequential for now
/// TODO: Refactor tests to be independent and run in parallel

#[actix_web::test]
async fn test_create_workflow_task_edge_success() {
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
    let test_task2 = common::task::test_task_dyn(
        &db,
        test_user.id.to_string().clone(),
        String::from("Test Task 2"),
    )
    .await;
    let test_workflow_task1 = common::workflow_task::test_workflow_task(
        &db,
        test_workflow.id.to_string().clone(),
        test_task.id.to_string().clone(),
        1,
    )
    .await;

    let test_workflow_task2 = common::workflow_task::test_workflow_task_dyn(
        &db,
        test_workflow.id.to_string().clone(),
        test_task2.id.to_string().clone(),
        2,
        "Step_2".into(),
    )
    .await;

    let app = common::setup_app(db).await;

    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(
                &common::path::Paths::CREATE_WORKFLOW_TASK_EDGE
                    .replace("{workflow_id}", test_workflow.id.to_string().as_str()),
            )
            .set_json(&json!({
                "from_task_id": test_workflow_task1.id.to_string().as_str(),
                "to_task_id": test_workflow_task2.id.to_string().as_str()
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
async fn test_delete_workflow_task_edge_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&db, test_user.id.to_string()).await;
    let task1 = common::task::test_task(&db, test_user.id.to_string()).await;
    let task2 = common::task::test_task_dyn(&db, test_user.id.to_string(), "Task2".into()).await;
    let wt1 = common::workflow_task::test_workflow_task(
        &db,
        workflow.id.to_string(),
        task1.id.to_string(),
        1,
    )
    .await;
    let wt2 = common::workflow_task::test_workflow_task_dyn(
        &db,
        workflow.id.to_string(),
        task2.id.to_string(),
        2,
        "Step_2".into(),
    )
    .await;
    let edge = common::worflow_task_edge::test_workflow_task_edge(
        &db,
        workflow.id.to_string(),
        wt1.id.to_string(),
        wt2.id.to_string(),
    )
    .await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::delete()
            .uri(
                &common::path::Paths::EDIT_DELETE_WORKFLOW_TASK_EDGE
                    .replace("{workflow_id}", &workflow.id.to_string())
                    .replace("{edge_id}", &edge.id.to_string()),
            )
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", test_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
}

// --- Failure tests ---

#[actix_web::test]
async fn test_workflow_task_edge_endpoints_unauthenticated() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db).await;
    let create_url =
        common::path::Paths::CREATE_WORKFLOW_TASK_EDGE.replace("{workflow_id}", NONEXISTENT_ID);
    let item_url = common::path::Paths::EDIT_DELETE_WORKFLOW_TASK_EDGE
        .replace("{workflow_id}", NONEXISTENT_ID)
        .replace("{edge_id}", NONEXISTENT_ID);

    let unauth_status = |r: Result<_, actix_web::Error>| {
        r.map(|resp: actix_web::dev::ServiceResponse| resp.status())
            .unwrap_or_else(|e| e.as_response_error().status_code())
    };

    for status in [
        unauth_status(
            test::try_call_service(
                &app,
                test::TestRequest::post()
                    .uri(&create_url)
                    .set_json(&json!({}))
                    .to_request(),
            )
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
async fn test_create_workflow_task_edge_duplicate() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&db, test_user.id.to_string()).await;
    let task1 = common::task::test_task(&db, test_user.id.to_string()).await;
    let task2 = common::task::test_task_dyn(&db, test_user.id.to_string(), "Task2".into()).await;
    let wt1 = common::workflow_task::test_workflow_task(
        &db,
        workflow.id.to_string(),
        task1.id.to_string(),
        1,
    )
    .await;
    let wt2 = common::workflow_task::test_workflow_task_dyn(
        &db,
        workflow.id.to_string(),
        task2.id.to_string(),
        2,
        "Step_2".into(),
    )
    .await;
    common::worflow_task_edge::test_workflow_task_edge(
        &db,
        workflow.id.to_string(),
        wt1.id.to_string(),
        wt2.id.to_string(),
    )
    .await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(
                &common::path::Paths::CREATE_WORKFLOW_TASK_EDGE
                    .replace("{workflow_id}", &workflow.id.to_string()),
            )
            .set_json(&json!({
                "from_task_id": wt1.id.to_string(),
                "to_task_id": wt2.id.to_string()
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
async fn test_create_workflow_task_edge_self_loop() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&db, test_user.id.to_string()).await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(
                &common::path::Paths::CREATE_WORKFLOW_TASK_EDGE
                    .replace("{workflow_id}", &workflow.id.to_string()),
            )
            .set_json(&json!({
                "from_task_id": NONEXISTENT_ID,
                "to_task_id": NONEXISTENT_ID
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
async fn test_delete_workflow_task_edge_wrong_user() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let owner = common::user::test_user(&db, None, None).await;
    let workflow = common::workflow::test_workflow(&db, owner.id.to_string()).await;
    let task1 = common::task::test_task(&db, owner.id.to_string()).await;
    let task2 = common::task::test_task_dyn(&db, owner.id.to_string(), "Task2".into()).await;
    let wt1 = common::workflow_task::test_workflow_task(
        &db,
        workflow.id.to_string(),
        task1.id.to_string(),
        1,
    )
    .await;
    let wt2 = common::workflow_task::test_workflow_task_dyn(
        &db,
        workflow.id.to_string(),
        task2.id.to_string(),
        2,
        "Step_2".into(),
    )
    .await;
    let edge = common::worflow_task_edge::test_workflow_task_edge(
        &db,
        workflow.id.to_string(),
        wt1.id.to_string(),
        wt2.id.to_string(),
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
                &common::path::Paths::EDIT_DELETE_WORKFLOW_TASK_EDGE
                    .replace("{workflow_id}", &workflow.id.to_string())
                    .replace("{edge_id}", &edge.id.to_string()),
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
