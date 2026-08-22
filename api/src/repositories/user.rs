use uuid::Uuid;

use crate::models::{User, now};

pub struct CreateUserParams {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

/// Only the fields a user is allowed to change after signup. Everything left
/// `None` is untouched.
#[derive(Default)]
pub struct UpdateUserParams {
    pub username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub password_hash: Option<String>,
}

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        &self,
        db: &mut toasty::Db,
        params: CreateUserParams,
    ) -> Result<User, toasty::Error> {
        let timestamp = now();
        toasty::create!(User {
            username: params.username,
            email: params.email,
            email_verified: false,
            password_hash: params.password_hash,
            date_created: timestamp,
            date_updated: timestamp,
            date_joined: timestamp,
        })
        .exec(db)
        .await
    }

    pub async fn find_by_id(
        &self,
        db: &mut toasty::Db,
        id: Uuid,
    ) -> Result<Option<User>, toasty::Error> {
        User::filter(User::fields().id().eq(id))
            .first()
            .exec(db)
            .await
    }

    pub async fn find_by_email(
        &self,
        db: &mut toasty::Db,
        email: &str,
    ) -> Result<Option<User>, toasty::Error> {
        User::filter(User::fields().email().eq(email))
            .first()
            .exec(db)
            .await
    }

    pub async fn update(
        &self,
        db: &mut toasty::Db,
        user: &mut User,
        params: UpdateUserParams,
    ) -> Result<(), toasty::Error> {
        let mut stmt = toasty::update!(user {
            date_updated: now()
        });
        if let Some(username) = params.username {
            stmt = stmt.username(username);
        }
        if let Some(email) = params.email {
            stmt = stmt.email(email);
        }
        if let Some(email_verified) = params.email_verified {
            stmt = stmt.email_verified(email_verified);
        }
        if let Some(password_hash) = params.password_hash {
            stmt = stmt.password_hash(password_hash);
        }
        stmt.exec(db).await
    }

    /// Deleting a user takes their tasks, workflows, and pending verifications
    /// with it, because those relations are declared on the model.
    pub async fn delete(&self, db: &mut toasty::Db, id: Uuid) -> Result<(), toasty::Error> {
        User::filter(User::fields().id().eq(id))
            .delete()
            .exec(db)
            .await
    }
}
