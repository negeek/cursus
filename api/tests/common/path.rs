pub struct Paths;

impl Paths {
    pub const SIGNUP: &'static str = "/api/v1/account/signup";
    pub const SIGNIN: &'static str = "/api/v1/account/signin";
    pub const VERIFY_EMAIL: &'static str = "/api/v1/account/verify_email";
    pub const LOGOUT: &'static str = "/api/v1/account/logout";
}
