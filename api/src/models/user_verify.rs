use jiff::civil::DateTime;
use uuid::Uuid;

use crate::models::User;

/// A pending email verification code.
#[derive(Clone, Debug, toasty::Model)]
#[table = "user_verifications"]
pub struct UserVerify {
    #[key]
    #[auto(increment)]
    pub id: i64,

    #[index]
    pub user_id: Uuid,

    /// Named after the parent model in snake case. Toasty pairs `has_many` on
    /// the parent with the child's `belongs_to` by that convention, so a
    /// shortened field name would break the pairing.
    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::Deferred<User>,

    #[unique]
    pub code: String,

    pub expires_at: DateTime,

    pub date_created: DateTime,
    pub date_updated: DateTime,
}
