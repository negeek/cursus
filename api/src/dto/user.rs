use crate::dto::error::ValidationError;
use serde::{Deserialize, Serialize};

pub trait RequestValidateTrait {
    fn validate(&self) -> Result<(), ValidationError>;
}

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl RequestValidateTrait for SignUpRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        // Basic validation for email format. Not RFC compliant but serves as a simple check.
        if self.email.split('@').count() != 2 || self.email.split('.').count() < 2 {
            return Err(ValidationError {
                field: String::from("email"),
                message: String::from("Invalid email format"),
            });
        }

        if self.username.chars().count() < 3 {
            return Err(ValidationError {
                field: String::from("username"),
                message: String::from("Username must be at least 3 characters long"),
            });
        }

        if self.password.chars().count() < 8 {
            return Err(ValidationError {
                field: String::from("password"),
                message: String::from("Password must be at least 8 characters long"),
            });
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct SignUpResponse {
    pub success: bool,
}

#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignInResponse {
    pub id: String,
    pub success: bool,
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub id: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct VerifyEmailResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub success: bool,
}
