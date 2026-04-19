use crate::dto::user::{RequestValidateTrait, SignUpRequest, SignUpResponse};
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
    let _ = user_service.create_user(&db, body.into_inner()).await?;
    Ok(web::Json(SignUpResponse { success: true }))
}
