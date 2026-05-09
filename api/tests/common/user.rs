use api::db::entity::token::{ActiveModel as BlacklistTokenOp, Model as BlacklistTokenRow};
use api::db::entity::user::{ActiveModel as UserOp, Model as UserRow};
use api::db::repository::{RepositoryTrait, token::BlacklistTokenRepository, user::UserRepository};
use api::util::token::{Token, TokenCodec};
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;

/// Useful test user that is verified
pub async fn test_user(
    db: &DatabaseConnection,
    mut username: Option<String>,
    mut email: Option<String>,
) -> UserRow {
    if username.is_none() {
        username = Some(String::from("test_user"));
    }
    if email.is_none() {
        email = Some(String::from("testuser@gmail.com"))
    }

    let user_data = UserOp {
        id: Default::default(),
        username: Set(username.unwrap()),
        email: Set(email.unwrap()),
        email_verified: Set(true),
        password_hash: Set(String::from("password_hash")),
        ..Default::default()
    };
    let user_res = UserRepository {}.create(db, user_data).await;
    match user_res {
        Ok(u) => return u,
        Err(e) => panic!("Error: {}", e),
    };
}

/// Provides tokens for test user or user provided
pub async fn test_tokens(db: &DatabaseConnection, mut user: Option<UserRow>) -> Token {
    if user.is_none() {
        let test_user = test_user(db, None, None).await;
        user = Some(test_user)
    }
    let token_codec = TokenCodec::new(user.unwrap().id.to_string());
    let token_res = token_codec.generate();
    match token_res {
        Ok(t) => return t,
        Err(e) => panic!("Error: {}", e),
    };
}

/// Blacklists a token. Extracts jti and type from claims.
/// Panics if token is invalid or DB insert fails.
pub async fn test_blacklisted(db: &DatabaseConnection, token: String) -> BlacklistTokenRow {
    let claims = match TokenCodec::validate(&token) {
        Ok(c) => c,
        Err(e) => panic!("Invalid token: {}", e),
    };
    let token_data = BlacklistTokenOp {
        id: Default::default(),
        jti: Set(claims.jti),
        token_type: Set(claims.ttype),
        expires_at: Set(
            chrono::DateTime::from_timestamp_secs(claims.exp as i64).unwrap_or(chrono::Utc::now())
        ),
        ..Default::default()
    };
    match (BlacklistTokenRepository {}).create(db, token_data).await {
        Ok(row) => row,
        Err(e) => panic!("Failed to blacklist token: {}", e),
    }
}
