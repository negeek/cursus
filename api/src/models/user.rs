use jiff::civil::DateTime;
use uuid::Uuid;

use crate::models::{Task, UserVerify, Workflow};

#[derive(Debug, toasty::Model)]
#[table = "users"]
pub struct User {
    #[key]
    #[auto(uuid(v7))]
    pub id: Uuid,

    pub username: String,

    #[unique]
    pub email: String,

    pub email_verified: bool,

    pub password_hash: String,

    pub date_created: DateTime,
    pub date_updated: DateTime,
    pub date_joined: DateTime,

    /// Reverse sides of the relations that point here. These are what make
    /// toasty cascade a delete down to the owned rows, since it applies
    /// referential integrity in the query engine rather than through database
    /// foreign keys. Without them, deleting a user would silently orphan
    /// everything it owns.
    #[has_many]
    pub tasks: toasty::Deferred<Vec<Task>>,

    #[has_many]
    pub workflows: toasty::Deferred<Vec<Workflow>>,

    #[has_many]
    pub user_verifies: toasty::Deferred<Vec<UserVerify>>,
}
