use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
};
use serde::{Deserialize, Serialize};
use std::env;

pub enum TokenType {
    Access,
    Refresh,
}
impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Access => String::from("access"),
            Self::Refresh => String::from("refresh"),
        }
    }
}

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
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub sub: String,
    pub jti: String,
    pub ttype: String,
}

impl TokenCodec {
    /// Gives a default codec
    pub fn new(subject: String) -> Self {
        return TokenCodec {
            alg: Algorithm::HS256,
            secret: TokenCodec::secret(),
            access_expiry: (chrono::Utc::now() + chrono::TimeDelta::days(1)).timestamp() as usize,
            refresh_expiry: (chrono::Utc::now() + chrono::TimeDelta::days(24)).timestamp() as usize,
            issue_at: chrono::Utc::now().timestamp() as usize,
            issuer: TokenCodec::issuer(),
            audience: TokenCodec::audience(),
            subject: subject,
        };
    }

    pub fn secret() -> String {
        env::var("SECRET").expect("Token code secret unavilable")
    }

    pub fn issuer() -> String {
        String::from("cursus_api")
    }

    pub fn audience() -> String {
        String::from("cursus_user")
    }

    pub fn generate_token(&self, ttype: TokenType) -> Result<String, Error> {
        let expiry = match ttype {
            TokenType::Access => self.access_expiry,
            TokenType::Refresh => self.refresh_expiry,
        };
        let claims = Claims {
            aud: self.audience.clone(),
            exp: expiry,
            iat: self.issue_at,
            iss: self.issuer.clone(),
            sub: self.subject.clone(),
            jti: uuid::Uuid::new_v4().to_string(),
            ttype: ttype.to_string(),
        };
        let token: String = encode(
            &Header::new(self.alg),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        return Ok(token);
    }

    /// Generates access and refresh token
    /// ensure to use the new() method before calling it
    pub fn generate(&self) -> Result<Token, Error> {
        let access_token = self.generate_token(TokenType::Access)?;
        let refresh_token = self.generate_token(TokenType::Refresh)?;
        return Ok(Token {
            access: access_token,
            refresh: refresh_token,
        });
    }

    pub fn validate(token: &str) -> Result<Claims, Error> {
        let secret = TokenCodec::secret();
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[TokenCodec::audience()]);
        validation.set_issuer(&[TokenCodec::issuer()]);

        let decode_result: Result<jsonwebtoken::TokenData<Claims>, Error> = decode(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        );
        let claims: Claims = match decode_result {
            Ok(c) => c.claims,
            Err(e) => return Err(e),
        };
        Ok(claims)
    }
}
