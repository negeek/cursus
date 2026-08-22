pub mod common;

use actix_web::{http::header, test};
use serde_json::json;

#[actix_web::test]
async fn test_signup_success() {
    let mut db = common::test_db().await;
    let app = common::setup_app(db.clone()).await;

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
    let mut db = common::test_db().await;
    let app = common::setup_app(db.clone()).await;

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
    let mut db = common::test_db().await;
    let app = common::setup_app(db.clone()).await;

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
    let mut db = common::test_db().await;
    let app = common::setup_app(db.clone()).await;

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
    let mut db = common::test_db().await;
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
    let mut db = common::test_db().await;
    let app = common::setup_app(db.clone()).await;

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
    let mut db = common::test_db().await;
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
    let mut db = common::test_db().await;
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
    let repo = api::repositories::user_verify::UserVerifyRepository;
    let verify_row = repo
        .find_latest_by_user_id(&mut db, parsed_id)
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
    let mut db = common::test_db().await;
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

#[actix_web::test]
async fn test_logout_success() {
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
            .uri(common::path::Paths::LOGOUT)
            .set_json(&json!({
               "refresh_token": test_tokens.refresh
            }))
            .insert_header(auth_header)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["success"].is_boolean())
}

#[actix_web::test]
async fn test_logout_invalid_token() {
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
            .uri(common::path::Paths::LOGOUT)
            .set_json(&json!({
               "refresh_token": "test_tokens.refresh"
            }))
            .insert_header(auth_header)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_logout_invalid_for_user() {
    let mut db = common::test_db().await;
    let test_user = common::user::test_user(
        &mut db,
        Some("username".to_string()),
        Some("email@gmail.com".to_string()),
    )
    .await;
    let test_tokens = common::user::test_tokens(&mut db, None).await;
    let test_tokens2 = common::user::test_tokens(&mut db, Some(test_user)).await;
    let app = common::setup_app(db.clone()).await;

    let auth_header = (
        header::AUTHORIZATION,
        format!("Bearer {}", test_tokens.access),
    );
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::LOGOUT)
            .set_json(&json!({
               "refresh_token": test_tokens2.refresh
            }))
            .insert_header(auth_header)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_refresh_access_success() {
    let mut db = common::test_db().await;
    let test_tokens = common::user::test_tokens(&mut db, None).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::REFRESH_ACCESS)
            .set_json(&json!({
               "refresh_token": test_tokens.refresh
            }))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["access_token"].is_string());
}

#[actix_web::test]
async fn test_refresh_access_with_access_token_401() {
    let mut db = common::test_db().await;
    let test_tokens = common::user::test_tokens(&mut db, None).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::REFRESH_ACCESS)
            .set_json(&json!({
               "refresh_token": test_tokens.access
            }))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_refresh_access_blacklisted_401() {
    let mut db = common::test_db().await;
    let test_tokens = common::user::test_tokens(&mut db, None).await;
    let _ = common::user::test_blacklisted(&mut db, test_tokens.refresh.clone()).await;
    let app = common::setup_app(db.clone()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(common::path::Paths::REFRESH_ACCESS)
            .set_json(&json!({
               "refresh_token": test_tokens.refresh
            }))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 401);
}
