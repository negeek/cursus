use crate::db::entity::user::{ActiveModel as UserOp, Model as UserRow};
use crate::db::repository::RepositoryTrait;
use crate::db::repository::user::UserRepository;
use crate::dto::error::user::UserServiceError;
use crate::dto::user::SignUpRequest;
use bcrypt::{DEFAULT_COST, hash};
use sea_orm::{DatabaseConnection, Set};
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            repository: UserRepository {},
        }
    }
    pub async fn create_user(
        &self,
        db: &DatabaseConnection,
        user_req: SignUpRequest,
    ) -> Result<UserRow, UserServiceError> {
        let exists = self.repository.find_by_email(db, &user_req.email).await?;
        if exists.is_some() {
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
        let user = self.repository.create(db, user_data).await?;
        Ok(user)
    }
}
