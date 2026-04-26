pub mod common;

use actix_web::test;
use serde_json::json;

/// Tests are sequential for now
/// TODO: Refactor tests to be independent and run in parallel

#[actix_web::test]
async fn test_signup_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db).await;

    let req = test::TestRequest::post()
        .uri(common::path::Paths::SIGNUP)
        .set_json(&json!({
            "username": "testuser",
            "email": "testuser@example.com",
            "password": "testpassword"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_signup_email_400() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db).await;

    let req = test::TestRequest::post()
        .uri(common::path::Paths::SIGNUP)
        .set_json(&json!({
            "username": "testuser",
            "email": "testuserexample.com",
            "password": "testpassword"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_signup_name_400() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db).await;

    let req = test::TestRequest::post()
        .uri(common::path::Paths::SIGNUP)
        .set_json(&json!({
            "username": "t",
            "email": "testuser@example.com",
            "password": "testpassword"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_signup_password_400() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db).await;

    let req = test::TestRequest::post()
        .uri(common::path::Paths::SIGNUP)
        .set_json(&json!({
            "username": "testuser",
            "email": "testuser@example.com",
            "password": "t"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_signup_user_exist_409() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db).await;

    let _ = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::SIGNUP)
            .set_json(&json!({
                "username": "testuser",
                "email": "testuser@example.com",
                "password": "testpassword"
            }))
            .to_request(),
    )
    .await;

    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::SIGNUP)
            .set_json(&json!({
                "username": "testuser",
                "email": "testuser@example.com",
                "password": "testpassword"
            }))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 409);
}

#[actix_web::test]
async fn test_signin_invalid_creds() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db).await;

    let req = test::TestRequest::post()
        .uri(common::path::Paths::SIGNIN)
        .set_json(&json!({
            "email": "testuser@example.com",
            "password": "testpassword"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_signin_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db).await;

    let _ = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::SIGNUP)
            .set_json(&json!({
                "username": "testuser",
                "email": "testuser@example.com",
                "password": "testpassword"
            }))
            .to_request(),
    )
    .await;

    let req = test::TestRequest::post()
        .uri(common::path::Paths::SIGNIN)
        .set_json(&json!({
            "email": "testuser@example.com",
            "password": "testpassword"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert!(body["id"].is_string());
}

#[actix_web::test]
async fn test_verify_email_success() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db.clone()).await;

    let _ = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::SIGNUP)
            .set_json(&json!({
                "username": "testuser",
                "email": "testuser@example.com",
                "password": "testpassword"
            }))
            .to_request(),
    )
    .await;

    let signin_resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::SIGNIN)
            .set_json(&json!({
                "email": "testuser@example.com",
                "password": "testpassword"
            }))
            .to_request(),
    )
    .await;
    let signin_body: serde_json::Value = test::read_body_json(signin_resp).await;
    let user_id = signin_body["id"].as_str().unwrap().to_string();

    let parsed_id = uuid::Uuid::parse_str(&user_id).unwrap();
    let repo = api::db::repository::user_verify::UserVerifyRepository {};
    let verify_row = repo
        .find_latest_by_user_id(&db, &parsed_id)
        .await
        .unwrap()
        .unwrap();
    let code = verify_row.code;

    let req = test::TestRequest::post()
        .uri(common::path::Paths::VERIFY_EMAIL)
        .set_json(&json!({
            "id": user_id,
            "code": code
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["access_token"].is_string());
    assert!(body["refresh_token"].is_string());
    assert_eq!(body["email"], "testuser@example.com");
}

#[actix_web::test]
async fn test_verify_email_wrong_code() {
    let db = common::test_db().await;
    common::truncate_db(&db).await;
    let app = common::setup_app(db).await;

    let _ = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::SIGNUP)
            .set_json(&json!({
                "username": "testuser",
                "email": "testuser@example.com",
                "password": "testpassword"
            }))
            .to_request(),
    )
    .await;

    let signin_resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::SIGNIN)
            .set_json(&json!({
                "email": "testuser@example.com",
                "password": "testpassword"
            }))
            .to_request(),
    )
    .await;
    let signin_body: serde_json::Value = test::read_body_json(signin_resp).await;
    let user_id = signin_body["id"].as_str().unwrap().to_string();

    let req = test::TestRequest::post()
        .uri(common::path::Paths::VERIFY_EMAIL)
        .set_json(&json!({
            "id": user_id,
            "code": "WRONG1"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}
