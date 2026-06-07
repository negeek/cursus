pub mod common;

use actix_web::{http::header, test};
use serde_json::json;

/// Tests are sequential for now
/// TODO: Refactor tests to be independent and run in parallel

#[actix_web::test]
async fn test_create_task_success() {
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
            .uri(common::path::Paths::CREATE_LIST_TASK)
            .set_json(&json!({
               "name": "send_email",
               "description": "Send an email to user",
               "endpoint": "https://webhook.com/",
               "result_schema": {
                   "data": {
                       "status": "string"
                    }
               },
               "body": {
                   "data": {
                       "type": "send_email",
                       "to": "danieladebowale192@gmail.com",
                       "subject": "Welcome to cursus",
                       "content": "Hey Daniel, welcome to cursus! We're excited to have you on board."
                   }
                }
            }))
            .insert_header(auth_header)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("id").is_some());
}

#[actix_web::test]
async fn test_edit_task_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let auth_header = (
        header::AUTHORIZATION,
        format!("Bearer {}", test_tokens.access),
    );
    let task = common::task::test_task(&db, test_user.id.to_string()).await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::patch()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_TASK
                    .replace("{task_id}", &task.id.to_string()),
            )
            .set_json(&json!({
               "name": "send_email",
               "description": "Send an email to user",
               "endpoint": "https://webhooks.com/",
               "result_schema": {
                   "data": {
                       "status": "string"
                    }
               },
               "body": {
                   "data": {
                       "type": "send_email",
                       "to": "danieladebowale192@gmail.com",
                       "subject": "Welcome to cursus",
                       "content": "Hey Daniel, welcome to cursus."
                   }
                }
            }))
            .insert_header(auth_header)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("id").is_some());
}

#[actix_web::test]
async fn test_delete_task_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let auth_header = (
        header::AUTHORIZATION,
        format!("Bearer {}", test_tokens.access),
    );
    let task = common::task::test_task(&db, test_user.id.to_string()).await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::delete()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_TASK
                    .replace("{task_id}", &task.id.to_string()),
            )
            .insert_header(auth_header)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body.get("message").unwrap().as_str().unwrap(),
        "Task deleted successfully"
    );
}

#[actix_web::test]
async fn test_get_task_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let auth_header = (
        header::AUTHORIZATION,
        format!("Bearer {}", test_tokens.access),
    );
    let task = common::task::test_task(&db, test_user.id.to_string()).await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(
                &common::path::Paths::GET_EDIT_DELETE_TASK
                    .replace("{task_id}", &task.id.to_string()),
            )
            .insert_header(auth_header)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body.get("id").unwrap().as_str().unwrap(),
        task.id.to_string()
    );
}

#[actix_web::test]
async fn test_list_tasks_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let test_user = common::user::test_user(&db, None, None).await;
    let test_tokens = common::user::test_tokens(&db, Some(test_user.clone())).await;
    let auth_header = (
        header::AUTHORIZATION,
        format!("Bearer {}", test_tokens.access),
    );
    common::task::test_task(&db, test_user.id.to_string()).await;
    let app = common::setup_app(db).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&common::path::Paths::CREATE_LIST_TASK)
            .insert_header(auth_header)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 1);
}
