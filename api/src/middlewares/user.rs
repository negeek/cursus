use crate::{
    repositories::{token::BlacklistTokenRepository, user::UserRepository},
    state::AppState,
    util::token::{TokenCodec, TokenType},
};
use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};

use core::fmt;
use std::future::{Ready, ready};
use std::pin::Pin;

use crate::dtos::error::ApiError;
use crate::middlewares::request_event::{add_context, context_key, set_user_id};
use actix_web::{HttpResponse, ResponseError};
use std::rc::Rc;
#[derive(Clone)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub access_token_jti: String,
    pub access_token_exp: usize,
}

#[derive(Debug)]
pub enum AuthError {
    Unauthorised,
    InvalidToken,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::Unauthorised => write!(f, "Unauthorised to access this resource"),
            AuthError::InvalidToken => write!(f, "Invalid auth"),
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> HttpResponse {
        let api_error = ApiError {
            error: self.to_string(),
        };
        match self {
            AuthError::Unauthorised => HttpResponse::Unauthorized().json(api_error),
            AuthError::InvalidToken => HttpResponse::Unauthorized().json(api_error),
        }
    }
}

/// Records why a request was turned away, then returns the error.
///
/// Every 401 looks the same to the caller on purpose. The log line is where the
/// difference between a missing header and a revoked token has to show up.
fn reject(reason: &str, error: AuthError) -> Error {
    add_context(
        context_key::AUTH,
        serde_json::json!({ "outcome": "rejected", "reason": reason }),
    );
    error.into()
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthUserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthUserMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthUserMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthUserMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization").cloned();
        let service = self.service.clone();

        Box::pin(async move {
            // check header exists
            let header = match auth_header {
                Some(h) => h,
                None => return Err(reject("no authorization header", AuthError::Unauthorised)),
            };

            // parse token
            let token_str = header
                .to_str()
                .map_err(|_| reject("unreadable authorization header", AuthError::InvalidToken))?;
            let parts: Vec<&str> = token_str.split(' ').collect();
            if parts.len() != 2 || parts[0] != "Bearer" {
                return Err(reject("not a bearer token", AuthError::InvalidToken));
            }

            // validate JWT
            let claims = TokenCodec::validate(parts[1])
                .map_err(|_| reject("token failed validation", AuthError::InvalidToken))?;

            if claims.ttype != TokenType::Access.to_string() {
                return Err(reject("not an access token", AuthError::InvalidToken));
            }

            // The signature is verified by this point, so the subject is
            // trustworthy. Recorded before the revocation and lookup checks so a
            // rejected request still says whose token was presented.
            set_user_id(&claims.sub);

            // Toasty needs an owned, mutable handle, so take a clone of the
            // pooled one rather than borrowing out of the shared state.
            let mut db = req
                .app_data::<web::Data<AppState>>()
                .ok_or_else(|| reject("application state missing", AuthError::Unauthorised))?
                .db
                .clone();

            // A token that has been revoked is no longer usable even though it
            // is still within its lifetime and still verifies.
            if BlacklistTokenRepository
                .is_blacklisted(&mut db, &claims.jti)
                .await
                .map_err(|_| reject("blacklist lookup failed", AuthError::Unauthorised))?
            {
                return Err(reject("token was revoked", AuthError::InvalidToken));
            }

            let id = uuid::Uuid::parse_str(&claims.sub)
                .map_err(|_| reject("subject is not a valid id", AuthError::InvalidToken))?;
            let user = UserRepository
                .find_by_id(&mut db, id)
                .await
                .map_err(|_| reject("user lookup failed", AuthError::Unauthorised))?
                .ok_or_else(|| reject("no user for this token", AuthError::Unauthorised))?;

            // attach AuthUser to request extensions
            req.extensions_mut().insert(AuthUser {
                id: claims.sub,
                email: user.email,
                username: user.username,
                access_token_jti: claims.jti,
                access_token_exp: claims.exp,
            });

            // pass to next service
            service.call(req).await
        })
    }
}
