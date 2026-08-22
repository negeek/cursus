use bcrypt::{DEFAULT_COST, hash, verify};
use uuid::Uuid;

use crate::dtos::error::user::UserServiceError;
use crate::dtos::user::{LogoutRequest, SignInRequest, SignUpRequest, VerifyEmailRequest};
use crate::middlewares::user::AuthUser;
use crate::models::{User, now};
use crate::repositories::is_unique_violation;
use crate::repositories::token::{BlacklistTokenRepository, CreateBlacklistedTokenParams};
use crate::repositories::user::{CreateUserParams, UserRepository};
use crate::repositories::user_verify::{CreateUserVerifyParams, UserVerifyRepository};
use crate::util::token::{TokenCodec, TokenType};
use crate::util::{gen_, token};

/// How long a sign in code stays usable.
const VERIFICATION_CODE_TTL_MINUTES: i64 = 2;

pub struct UserService {
    users: UserRepository,
    verifications: UserVerifyRepository,
    blacklisted_tokens: BlacklistTokenRepository,
}

pub struct VSigninCodeData {
    pub user: User,
    pub token: token::Token,
}

impl Default for UserService {
    fn default() -> Self {
        Self::new()
    }
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: UserRepository,
            verifications: UserVerifyRepository,
            blacklisted_tokens: BlacklistTokenRepository,
        }
    }

    pub async fn sign_up(
        &self,
        db: &mut toasty::Db,
        request: SignUpRequest,
    ) -> Result<User, UserServiceError> {
        if self
            .users
            .find_by_email(db, &request.email)
            .await?
            .is_some()
        {
            return Err(UserServiceError::EmailAlreadyExists);
        }

        let password_hash = hash(&request.password, DEFAULT_COST).map_err(|e| {
            tracing::error!(error = %e, "password hashing failed");
            UserServiceError::HashingFailed
        })?;

        self.users
            .create(
                db,
                CreateUserParams {
                    username: request.username,
                    email: request.email,
                    password_hash,
                },
            )
            .await
            .map_err(|e| {
                // The check above can lose a race with a second signup for the
                // same address. The unique index is what actually decides, so a
                // violation here means the same thing as the check failing.
                if is_unique_violation(&e) {
                    UserServiceError::EmailAlreadyExists
                } else {
                    UserServiceError::Database(e)
                }
            })
    }

    /// Checks credentials and issues a short lived code to complete sign in.
    pub async fn sign_in(
        &self,
        db: &mut toasty::Db,
        request: SignInRequest,
    ) -> Result<User, UserServiceError> {
        let user = self.users.find_by_email(db, &request.email).await?;

        // An unknown email and a wrong password answer identically, so this
        // cannot be used to work out which addresses have accounts.
        let Some(user) = user else {
            return Err(UserServiceError::InvalidCredentials);
        };
        if !verify(&request.password, &user.password_hash).unwrap_or(false) {
            return Err(UserServiceError::InvalidCredentials);
        }

        let code = gen_::generate_random_code(6);
        let expires_at = now()
            .checked_add(jiff::Span::new().minutes(VERIFICATION_CODE_TTL_MINUTES))
            .map_err(|_| UserServiceError::TokenIssuanceFailed)?;

        self.verifications
            .create(
                db,
                CreateUserVerifyParams {
                    user_id: user.id,
                    code: code.clone(),
                    expires_at,
                },
            )
            .await?;

        // TODO: deliver by email. Logged until an email service is wired up.
        tracing::info!(user_id = %user.id, code = %code, "verification code generated");
        Ok(user)
    }

    /// Redeems a sign in code and issues tokens.
    pub async fn verify_signin_code(
        &self,
        db: &mut toasty::Db,
        request: VerifyEmailRequest,
    ) -> Result<VSigninCodeData, UserServiceError> {
        let user_id = Uuid::parse_str(&request.id)
            .map_err(|_| UserServiceError::InvalidUserId(request.id.clone()))?;

        let verification = self
            .verifications
            .find_by_code(db, &request.code, user_id)
            .await?
            .ok_or(UserServiceError::InvalidVerificationCode)?;

        // The row carries an expiry, so honour it. Without this check a code
        // stays usable forever, which is the whole reason it has a lifetime.
        if verification.expires_at < now() {
            return Err(UserServiceError::VerificationCodeExpired);
        }

        let user = self
            .users
            .find_by_id(db, user_id)
            .await?
            .ok_or(UserServiceError::UserNotFound)?;

        let token = TokenCodec::new(request.id)
            .generate()
            .map_err(|_| UserServiceError::TokenIssuanceFailed)?;

        // Spend every outstanding code for this user, so a code cannot be
        // redeemed twice and older unused ones stop working too.
        self.verifications.delete_for_user(db, user_id).await?;

        Ok(VSigninCodeData { user, token })
    }

    /// Revokes both the access token that made this request and the refresh
    /// token presented alongside it.
    pub async fn logout(
        &self,
        db: &mut toasty::Db,
        request: LogoutRequest,
        user: AuthUser,
    ) -> Result<(), UserServiceError> {
        let refresh_claims = TokenCodec::validate(&request.refresh_token)
            .map_err(|_| UserServiceError::MalformedToken)?;

        if refresh_claims.sub != user.id || refresh_claims.ttype != TokenType::Refresh.to_string() {
            return Err(UserServiceError::RejectedToken);
        }

        self.blacklisted_tokens
            .create(
                db,
                CreateBlacklistedTokenParams {
                    jti: user.access_token_jti,
                    token_type: TokenType::Access.to_string(),
                    expires_at: timestamp_to_datetime(user.access_token_exp),
                },
            )
            .await?;

        self.blacklisted_tokens
            .create(
                db,
                CreateBlacklistedTokenParams {
                    jti: refresh_claims.jti,
                    token_type: TokenType::Refresh.to_string(),
                    expires_at: timestamp_to_datetime(refresh_claims.exp),
                },
            )
            .await?;

        Ok(())
    }

    /// Issues a fresh access token against a still valid refresh token.
    pub async fn refresh_access_token(
        &self,
        db: &mut toasty::Db,
        refresh_token: &str,
    ) -> Result<String, UserServiceError> {
        let claims =
            TokenCodec::validate(refresh_token).map_err(|_| UserServiceError::MalformedToken)?;

        if claims.ttype != TokenType::Refresh.to_string() {
            return Err(UserServiceError::RejectedToken);
        }

        if self
            .blacklisted_tokens
            .is_blacklisted(db, &claims.jti)
            .await?
        {
            return Err(UserServiceError::RejectedToken);
        }

        TokenCodec::new(claims.sub)
            .generate_token(TokenType::Access)
            .map_err(|_| UserServiceError::TokenIssuanceFailed)
    }
}

/// Converts a JWT expiry claim into the shape the timestamp columns use.
///
/// Falls back to the current time if the claim is out of range, which records
/// the entry as already expired. That is the safe direction to fail: a
/// blacklist row that looks expired gets cleaned up early, whereas silently
/// dropping the row would leave a revoked token working.
fn timestamp_to_datetime(seconds: usize) -> jiff::civil::DateTime {
    jiff::Timestamp::from_second(seconds as i64)
        .map(|ts| ts.to_zoned(jiff::tz::TimeZone::UTC).datetime())
        .unwrap_or_else(|_| now())
}
