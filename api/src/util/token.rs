use jsonwebtoken::{Algorithm, EncodingKey, Header, encode, errors::Error};
use serde::{Deserialize, Serialize};

pub struct TokenCodec {
    alg: Algorithm,
    secret: String,
    access_expiry: usize,
    refresh_expiry: usize,
    issue_at: usize,
    issuer: String,
    audience: String,
    subject: String,
}

pub struct Token {
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    exp: usize,
    iat: usize,
    iss: String,
    sub: String,
    jti: String,
}

impl TokenCodec {
    /// Gives a default codec
    pub fn new(secret: String, subject: String) -> Self {
        return TokenCodec {
            alg: Algorithm::HS256,
            secret: secret, // Get this from env
            access_expiry: (chrono::Utc::now() + chrono::TimeDelta::days(1)).timestamp() as usize,
            refresh_expiry: (chrono::Utc::now() + chrono::TimeDelta::days(24)).timestamp() as usize,
            issue_at: chrono::Utc::now().timestamp() as usize,
            issuer: String::from("cursus_api"),
            audience: String::from("cursus_user"),
            subject: subject,
        };
    }

    /// Generates access and refresh token
    /// ensure to use the new() method before calling it
    pub fn generate(&self) -> Result<Token, Error> {
        let access_claims = Claims {
            aud: self.audience.clone(),
            exp: self.access_expiry,
            iat: self.issue_at,
            iss: self.issuer.clone(),
            sub: self.subject.clone(),
            jti: uuid::Uuid::new_v4().to_string(),
        };
        let refresh_claims = Claims {
            aud: self.audience.clone(),
            exp: self.refresh_expiry,
            iat: self.issue_at,
            iss: self.issuer.clone(),
            sub: self.subject.clone(),
            jti: uuid::Uuid::new_v4().to_string(),
        };
        let access_token = encode(
            &Header::new(self.alg),
            &access_claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        let refresh_token = encode(
            &Header::new(self.alg),
            &refresh_claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        return Ok(Token {
            access: access_token,
            refresh: refresh_token,
        });
    }
}
