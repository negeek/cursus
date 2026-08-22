/// What can go wrong in the user service.
///
/// Domain language only, no status codes. See `crate::handlers::errors` for how
/// these reach the wire.
///
/// One deliberate shape worth keeping: sign in failures collapse into a single
/// `InvalidCredentials` rather than distinguishing an unknown email from a wrong
/// password. Telling those apart would let anyone enumerate which addresses have
/// accounts.
#[derive(Debug, thiserror::Error)]
pub enum UserServiceError {
    #[error("an account with that email already exists")]
    EmailAlreadyExists,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("this account has not verified its email address")]
    EmailNotVerified,

    #[error("the verification code is not valid")]
    InvalidVerificationCode,

    #[error("the verification code has expired")]
    VerificationCodeExpired,

    #[error("user not found")]
    UserNotFound,

    #[error("'{0}' is not a valid user id")]
    InvalidUserId(String),

    /// The token was structurally unreadable: wrong shape, bad signature, or
    /// claims that do not parse.
    #[error("the token could not be read")]
    MalformedToken,

    /// The token read fine but cannot be accepted: expired, revoked, or the
    /// wrong kind for this endpoint.
    #[error("the token is no longer valid")]
    RejectedToken,

    #[error("could not issue a token")]
    TokenIssuanceFailed,

    #[error("password hashing failed")]
    HashingFailed,

    #[error("required configuration is missing: {0}")]
    MissingConfiguration(&'static str),

    #[error("database error: {0}")]
    Database(#[from] toasty::Error),
}
