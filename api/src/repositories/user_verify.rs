use jiff::civil::DateTime;
use uuid::Uuid;

use crate::models::{UserVerify, now};

pub struct CreateUserVerifyParams {
    pub user_id: Uuid,
    pub code: String,
    pub expires_at: DateTime,
}

pub struct UserVerifyRepository;

impl UserVerifyRepository {
    pub async fn create(
        &self,
        db: &mut toasty::Db,
        params: CreateUserVerifyParams,
    ) -> Result<UserVerify, toasty::Error> {
        let timestamp = now();
        toasty::create!(UserVerify {
            user_id: params.user_id,
            code: params.code,
            expires_at: params.expires_at,
            date_created: timestamp,
            date_updated: timestamp,
        })
        .exec(db)
        .await
    }

    pub async fn find_by_code(
        &self,
        db: &mut toasty::Db,
        code: &str,
    ) -> Result<Option<UserVerify>, toasty::Error> {
        UserVerify::filter(UserVerify::fields().code().eq(code))
            .first()
            .exec(db)
            .await
    }

    /// Clears every outstanding code for a user. Called once a code is redeemed,
    /// so an older unused one cannot be presented afterwards.
    pub async fn delete_for_user(
        &self,
        db: &mut toasty::Db,
        user_id: Uuid,
    ) -> Result<(), toasty::Error> {
        UserVerify::filter(UserVerify::fields().user_id().eq(user_id))
            .delete()
            .exec(db)
            .await
    }
}
