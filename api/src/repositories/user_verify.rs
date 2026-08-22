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

    /// Looks up a code, scoped to the user presenting it.
    ///
    /// Scoped rather than global so that knowing a code is not enough on its
    /// own. The caller has to know whose code it is too.
    ///
    /// Whether the code has expired is the caller's decision. Finding the row
    /// does not mean it may be redeemed.
    pub async fn find_by_code(
        &self,
        db: &mut toasty::Db,
        code: &str,
        user_id: Uuid,
    ) -> Result<Option<UserVerify>, toasty::Error> {
        UserVerify::filter(
            UserVerify::fields()
                .code()
                .eq(code)
                .and(UserVerify::fields().user_id().eq(user_id)),
        )
        .first()
        .exec(db)
        .await
    }

    /// The most recently issued code for a user.
    ///
    /// Ordered by the key, which increments, so the highest is the newest.
    pub async fn find_latest_by_user_id(
        &self,
        db: &mut toasty::Db,
        user_id: Uuid,
    ) -> Result<Option<UserVerify>, toasty::Error> {
        UserVerify::filter(UserVerify::fields().user_id().eq(user_id))
            .order_by(UserVerify::fields().id().desc())
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
