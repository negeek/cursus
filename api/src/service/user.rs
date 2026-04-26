use crate::db::entity::user::{ActiveModel as UserOp, Model as UserRow};
use crate::db::entity::user_verify::ActiveModel as UserVerifyOp;
use crate::db::repository::RepositoryTrait;
use crate::db::repository::user::UserRepository;
use crate::db::repository::user_verify::UserVerifyRepository;
use crate::dto::error::user::UserServiceError;
use crate::dto::user::{SignInRequest, SignUpRequest, VerifyEmailRequest};
use crate::util::{gen_, token};
use ::std::env;
use bcrypt::{DEFAULT_COST, hash, verify};
use sea_orm::{ConnectionTrait, Set};
pub struct UserService {
    user_repository: UserRepository,
    user_verify_repository: UserVerifyRepository,
}

pub struct VSigninCodeData {
    pub user: UserRow,
    pub token: token::Token,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            user_repository: UserRepository {},
            user_verify_repository: UserVerifyRepository {},
        }
    }
    /// Signs up a new user by validating the request, checking for existing email,
    /// hashing the password, and saving the user to the database.
    /// Returns the created user or an error.
    pub async fn sign_up(
        &self,
        db: &impl ConnectionTrait,
        user_req: SignUpRequest,
    ) -> Result<UserRow, UserServiceError> {
        let existing_user = self
            .user_repository
            .find_by_email(db, &user_req.email)
            .await?;
        if existing_user.is_some() {
            return Err(UserServiceError::EmailAlreadyExists);
        }
        // hash password here before saving to database
        let password_hash_result = hash(&user_req.password, DEFAULT_COST);
        let password_hash = match password_hash_result {
            Ok(hash) => hash,
            Err(e) => {
                println!("Error hashing password: {}", e);
                return Err(UserServiceError::HashingFailed);
            }
        };
        let user_data = UserOp {
            id: Default::default(), // will be set in before_save
            username: Set(user_req.username),
            email: Set(user_req.email),
            email_verified: Set(false),
            password_hash: Set(password_hash),
            ..Default::default()
        };
        let user = self.user_repository.create(db, user_data).await?;
        Ok(user)
    }

    /// Signs in a user by checking if the email exists and verifying the password.
    /// Sends an OTP code to the user's email if credentials are valid.
    /// Returns success or an error.
    pub async fn sign_in(
        &self,
        db: &impl ConnectionTrait,
        signin_req: SignInRequest,
    ) -> Result<UserRow, UserServiceError> {
        // Check if user exists with given email
        let existing_user = self
            .user_repository
            .find_by_email(db, &signin_req.email)
            .await?;
        if existing_user.is_none() {
            return Err(UserServiceError::InvalidCredentials);
        }
        // Verify password
        let is_valid = verify(
            &signin_req.password,
            &existing_user.as_ref().unwrap().password_hash,
        )
        .unwrap_or(false);
        if !is_valid {
            return Err(UserServiceError::InvalidCredentials);
        }
        let code = gen_::generate_random_code(6);
        let user_id = existing_user.as_ref().unwrap().id;
        let verify_data = UserVerifyOp {
            id: Default::default(), // will be set in before_save
            user_id: Set(user_id),
            code: Set(code.clone()),
            expires_at: Set(chrono::Utc::now() + chrono::TimeDelta::minutes(2)),
            ..Default::default()
        };
        let _ = self.user_verify_repository.create(db, verify_data).await?;
        // TODO: Now we need to send an email. Let's log it for now
        println!("User Verification code is {code}");
        Ok(existing_user.unwrap())
    }

    /// Verifies the code provided
    pub async fn verify_signin_code(
        &self,
        db: &impl ConnectionTrait,
        verify_req: VerifyEmailRequest,
    ) -> Result<VSigninCodeData, UserServiceError> {
        let id = uuid::Uuid::parse_str(&verify_req.id)
            .map_err(|_| UserServiceError::InvalidVerificationCode)?;
        // TODO: refactor to load user on code query
        let existing_code = self
            .user_verify_repository
            .find_by_code(db, &verify_req.code, &id)
            .await?;
        if existing_code.is_none() {
            return Err(UserServiceError::InvalidVerificationCode);
        }
        // Now we need to generate access and refresh tokens
        let secret = env::var("SECRET").map_err(|_| UserServiceError::MissingENV)?;
        let token_codec = token::TokenCodec::new(secret, verify_req.id);
        let token = token_codec
            .generate()
            .map_err(|_| UserServiceError::TokenError)?;
        let user = self.user_repository.find_by_uuid(db, &id).await?;
        Ok(VSigninCodeData {
            user: user.unwrap(),
            token: token,
        })
    }
}
