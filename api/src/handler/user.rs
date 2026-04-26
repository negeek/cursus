use crate::dto::user::{
    RequestValidateTrait, SignInRequest, SignInResponse, SignUpRequest, SignUpResponse,
    VerifyEmailRequest, VerifyEmailResponse,
};
use crate::service::user::UserService;
use actix_web::{Responder, Result, post, web};
use sea_orm::DatabaseConnection;

#[post("/signup")]
async fn sign_up(
    db: web::Data<DatabaseConnection>,
    body: web::Json<SignUpRequest>,
) -> Result<impl Responder> {
    let _ = &body.validate()?;
    let user_service = UserService::new();
    let _ = user_service.sign_up(&db, body.into_inner()).await?;
    Ok(web::Json(SignUpResponse { success: true }))
}

#[post("/signin")]
async fn signin(
    db: web::Data<DatabaseConnection>,
    body: web::Json<SignInRequest>,
) -> Result<impl Responder> {
    let user_service = UserService::new();
    let user = user_service.sign_in(&db, body.into_inner()).await?;
    Ok(web::Json(SignInResponse {
        id: user.id.to_string(),
        success: true,
    }))
}

#[post("/verify_email")]
async fn verify_email(
    db: web::Data<DatabaseConnection>,
    body: web::Json<VerifyEmailRequest>,
) -> Result<impl Responder> {
    let user_service = UserService::new();
    let vdata = user_service
        .verify_signin_code(&db, body.into_inner())
        .await?;
    Ok(web::Json(VerifyEmailResponse {
        id: vdata.user.id.to_string(),
        username: vdata.user.username,
        email: vdata.user.email,
        access_token: vdata.token.access,
        refresh_token: vdata.token.refresh,
    }))
}
