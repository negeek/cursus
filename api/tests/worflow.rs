pub mod common;

use actix_web::{http::header, test};
use serde_json::json;

/// Tests are sequential for now
/// TODO: Refactor tests to be independent and run in parallel

#[actix_web::test]
async fn test_create_workflow_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_tokens = common::user::test_tokens(&db, None).await;
    let app = common::setup_app(db).await;

    let auth_header = (
        header::AUTHORIZATION,
        format!("Bearer {}", test_tokens.access),
    );
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::CREATE_LIST_WORKFLOW)
            .set_json(&json!({
                "name": "Test Workflow",
                "description": "This is a test workflow.",
                "run_type": "manual",
                "schedule_time": null,
                "cron_expression": null
            }))
            .insert_header(auth_header)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("id").is_some());
}

