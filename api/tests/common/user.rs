use api::models::{BlacklistedToken, User};
use api::repositories::token::{BlacklistTokenRepository, CreateBlacklistedTokenParams};
use api::repositories::user::{CreateUserParams, UserRepository};
use api::util::token::{Token, TokenCodec};
use toasty::Db;

/// A verified user, ready to act.
///
/// Username and email default to fixed values. Tests that need two users in the
/// same database have to pass distinct ones, since email is unique.
pub async fn test_user(db: &mut Db, username: Option<String>, email: Option<String>) -> User {
    let user = UserRepository
        .create(
            db,
            CreateUserParams {
                username: username.unwrap_or_else(|| "test_user".to_string()),
                email: email.unwrap_or_else(|| "testuser@gmail.com".to_string()),
                password_hash: "password_hash".to_string(),
            },
        )
        .await
        .expect("failed to create the test user");

    // Signup leaves an account unverified, but most tests want one that can
    // already act, so this flips the flag rather than making every caller do it.
    let mut user = user;
    UserRepository
        .update(
            db,
            &mut user,
            api::repositories::user::UpdateUserParams {
                email_verified: Some(true),
                ..Default::default()
            },
        )
        .await
        .expect("failed to mark the test user verified");

    user
}

/// Access and refresh tokens for a user, creating one if none is given.
pub async fn test_tokens(db: &mut Db, user: Option<User>) -> Token {
    let user = match user {
        Some(user) => user,
        None => test_user(db, None, None).await,
    };

    TokenCodec::new(user.id.to_string())
        .generate()
        .expect("failed to generate test tokens")
}

/// Revokes a token, so tests can check a blacklisted one is refused.
pub async fn test_blacklisted(db: &mut Db, token: String) -> BlacklistedToken {
    let claims = TokenCodec::validate(&token).expect("cannot blacklist an invalid token");

    BlacklistTokenRepository
        .create(
            db,
            CreateBlacklistedTokenParams {
                jti: claims.jti,
                token_type: claims.ttype,
                expires_at: expiry_from_claim(claims.exp),
            },
        )
        .await
        .expect("failed to blacklist the token")
}

fn expiry_from_claim(seconds: usize) -> jiff::civil::DateTime {
    jiff::Timestamp::from_second(seconds as i64)
        .map(|ts| ts.to_zoned(jiff::tz::TimeZone::UTC).datetime())
        .unwrap_or_else(|_| api::models::now())
}
