use jiff::civil::DateTime;

use crate::models::{BlacklistedToken, now};

pub struct CreateBlacklistedTokenParams {
    pub jti: String,
    pub token_type: String,
    pub expires_at: DateTime,
}

pub struct BlacklistTokenRepository;

impl BlacklistTokenRepository {
    pub async fn create(
        &self,
        db: &mut toasty::Db,
        params: CreateBlacklistedTokenParams,
    ) -> Result<BlacklistedToken, toasty::Error> {
        let timestamp = now();
        toasty::create!(BlacklistedToken {
            jti: params.jti,
            token_type: params.token_type,
            expires_at: params.expires_at,
            date_created: timestamp,
            date_updated: timestamp,
        })
        .exec(db)
        .await
    }

    /// Whether this token has been revoked. Checked on every authenticated
    /// request, so it fetches at most one row and never the whole match set.
    pub async fn is_blacklisted(
        &self,
        db: &mut toasty::Db,
        jti: &str,
    ) -> Result<bool, toasty::Error> {
        Ok(
            BlacklistedToken::filter(BlacklistedToken::fields().jti().eq(jti))
                .first()
                .exec(db)
                .await?
                .is_some(),
        )
    }

    /// Drops rows whose underlying token has already expired on its own. They
    /// cannot be replayed at that point, so keeping them only grows the table.
    pub async fn delete_expired(
        &self,
        db: &mut toasty::Db,
        cutoff: DateTime,
    ) -> Result<(), toasty::Error> {
        BlacklistedToken::filter(BlacklistedToken::fields().expires_at().lt(cutoff))
            .delete()
            .exec(db)
            .await
    }
}
