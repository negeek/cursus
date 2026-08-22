pub mod common;

use actix_web::{http::header, test};
use serde_json::json;

const NONEXISTENT_ID: &str = "00000000-0000-0000-0000-000000000000";

#[actix_web::test]
async fn test_create_workflow_success() {
    let mut db = common::test_db().await;
    let test_tokens = common::user::test_tokens(&mut db, None).await;
    let app = common::setup_app(db.clone()).await;

    let auth_header = (
        header::AUTHORIZATION,
        format!("Bearer {}", test_tokens.access),
    );
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::CREATE_LIST_WORKFLOW)
            .set_json(json!({
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
    assert_eq!(resp.status(), 201);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("id").is_some());
}

#[actix_web::test]
async fn test_list_workflows_success() {
    let mut db = common::test_db().await;
    let test_user = common::user::test_user(&mut db, None, None).await;
    let test_tokens = common::user::test_tokens(&mut db, Some(test_user.clone())).await;
    common::workflow::test_workflow(&mut db, test_user.id.to_string()).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(common::path::Paths::CREATE_LIST_WORKFLOW)
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", test_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 1);
}

#[actix_web::test]
async fn test_get_workflow_success() {
    let mut db = common::test_db().await;
    let test_user = common::user::test_user(&mut db, None, None).await;
    let test_tokens = common::user::test_tokens(&mut db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&mut db, test_user.id.to_string()).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW
                    .replace("{workflow_id}", &workflow.id.to_string()),
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
        body.get("id").unwrap().as_str().unwrap(),
        workflow.id.to_string()
    );
}

#[actix_web::test]
async fn test_edit_workflow_success() {
    let mut db = common::test_db().await;
    let test_user = common::user::test_user(&mut db, None, None).await;
    let test_tokens = common::user::test_tokens(&mut db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&mut db, test_user.id.to_string()).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::patch()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW
                    .replace("{workflow_id}", &workflow.id.to_string()),
            )
            .set_json(json!({"name": "Updated Workflow"}))
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
        body.get("name").unwrap().as_str().unwrap(),
        "Updated Workflow"
    );
}

#[actix_web::test]
async fn test_delete_workflow_success() {
    let mut db = common::test_db().await;
    let test_user = common::user::test_user(&mut db, None, None).await;
    let test_tokens = common::user::test_tokens(&mut db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&mut db, test_user.id.to_string()).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::delete()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW
                    .replace("{workflow_id}", &workflow.id.to_string()),
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
        "Workflow deleted successfully"
    );
}

#[actix_web::test]
async fn test_get_workflow_detail_success() {
    let mut db = common::test_db().await;
    let test_user = common::user::test_user(&mut db, None, None).await;
    let test_tokens = common::user::test_tokens(&mut db, Some(test_user.clone())).await;
    let workflow = common::workflow::test_workflow(&mut db, test_user.id.to_string()).await;
    let task1 = common::task::test_task(&mut db, test_user.id.to_string()).await;
    let task2 =
        common::task::test_task_dyn(&mut db, test_user.id.to_string(), "Task2".into()).await;
    let wt1 = common::workflow_task::test_workflow_task(
        &mut db,
        workflow.id.to_string(),
        task1.id.to_string(),
        1,
    )
    .await;
    let wt2 = common::workflow_task::test_workflow_task_dyn(
        &mut db,
        workflow.id.to_string(),
        task2.id.to_string(),
        2,
        "Step_2".into(),
    )
    .await;
    common::worflow_task_edge::test_workflow_task_edge(
        &mut db,
        workflow.id.to_string(),
        wt1.id.to_string(),
        wt2.id.to_string(),
    )
    .await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(
                &common::path::Paths::GET_WORKFLOW_DETAIL
                    .replace("{workflow_id}", &workflow.id.to_string()),
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
    assert_eq!(body.get("tasks").unwrap().as_array().unwrap().len(), 2);
    assert_eq!(body.get("edges").unwrap().as_array().unwrap().len(), 1);
}

#[actix_web::test]
async fn test_workflow_endpoints_unauthenticated() {
    let db = common::test_db().await;
    let app = common::setup_app(db.clone()).await;
    let wf_url =
        common::path::Paths::GET_EDIT_DELETE_WORKFLOW.replace("{workflow_id}", NONEXISTENT_ID);

    for status in [
        common::rejected_status(
            test::try_call_service(
                &app,
                test::TestRequest::get()
                    .uri(common::path::Paths::CREATE_LIST_WORKFLOW)
                    .to_request(),
            )
            .await,
        ),
        common::rejected_status(
            test::try_call_service(
                &app,
                test::TestRequest::post()
                    .uri(common::path::Paths::CREATE_LIST_WORKFLOW)
                    .set_json(json!({}))
                    .to_request(),
            )
            .await,
        ),
        common::rejected_status(
            test::try_call_service(&app, test::TestRequest::get().uri(&wf_url).to_request()).await,
        ),
        common::rejected_status(
            test::try_call_service(
                &app,
                test::TestRequest::patch()
                    .uri(&wf_url)
                    .set_json(json!({}))
                    .to_request(),
            )
            .await,
        ),
        common::rejected_status(
            test::try_call_service(&app, test::TestRequest::delete().uri(&wf_url).to_request())
                .await,
        ),
    ] {
        assert_eq!(status, 401);
    }
}

#[actix_web::test]
async fn test_get_workflow_not_found() {
    let mut db = common::test_db().await;
    let test_tokens = common::user::test_tokens(&mut db, None).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW
                    .replace("{workflow_id}", NONEXISTENT_ID),
            )
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", test_tokens.access),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_workflow_wrong_user() {
    let mut db = common::test_db().await;
    let owner = common::user::test_user(&mut db, None, None).await;
    let workflow = common::workflow::test_workflow(&mut db, owner.id.to_string()).await;
    let other_user = common::user::test_user(
        &mut db,
        Some("other_user".into()),
        Some("other@test.com".into()),
    )
    .await;
    let other_tokens = common::user::test_tokens(&mut db, Some(other_user)).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW
                    .replace("{workflow_id}", &workflow.id.to_string()),
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
async fn test_edit_workflow_wrong_user() {
    let mut db = common::test_db().await;
    let owner = common::user::test_user(&mut db, None, None).await;
    let workflow = common::workflow::test_workflow(&mut db, owner.id.to_string()).await;
    let other_user = common::user::test_user(
        &mut db,
        Some("other_user".into()),
        Some("other@test.com".into()),
    )
    .await;
    let other_tokens = common::user::test_tokens(&mut db, Some(other_user)).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::patch()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW
                    .replace("{workflow_id}", &workflow.id.to_string()),
            )
            .set_json(json!({"name": "hacked"}))
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
async fn test_delete_workflow_wrong_user() {
    let mut db = common::test_db().await;
    let owner = common::user::test_user(&mut db, None, None).await;
    let workflow = common::workflow::test_workflow(&mut db, owner.id.to_string()).await;
    let other_user = common::user::test_user(
        &mut db,
        Some("other_user".into()),
        Some("other@test.com".into()),
    )
    .await;
    let other_tokens = common::user::test_tokens(&mut db, Some(other_user)).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::delete()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_WORKFLOW
                    .replace("{workflow_id}", &workflow.id.to_string()),
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
