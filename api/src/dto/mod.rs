pub mod error;
pub mod user;

use crate::dto::error::ValidationError;

pub trait RequestValidateTrait {
    fn validate(&self) -> Result<(), ValidationError>;
}
