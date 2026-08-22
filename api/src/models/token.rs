use jiff::civil::DateTime;

/// A revoked JWT, held until its natural expiry so it cannot be replayed.
///
/// Keeps the integer key it has always had rather than moving to a uuid. Nothing
/// references these rows, they are only ever looked up by `jti`, so a uuid would
/// buy nothing and cost the wider index.
#[derive(Clone, Debug, toasty::Model)]
#[table = "blacklisted_tokens"]
pub struct BlacklistedToken {
    #[key]
    #[auto(increment)]
    pub id: i64,

    #[unique]
    pub jti: String,

    pub token_type: String,

    /// When the underlying token expires on its own. Rows past this point are
    /// dead weight and can be cleaned up.
    pub expires_at: DateTime,

    pub date_created: DateTime,
    pub date_updated: DateTime,
}
