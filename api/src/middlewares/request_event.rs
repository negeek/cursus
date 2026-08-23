//! One request in, one structured log line out.
//!
//! The middleware fills in the fixed fields. Any layer can add to the rest
//! through [`add_context`] and [`set_user_id`], which are no-ops outside a
//! request so the same code works from the runner and from CLI commands.
//!
//! ```ignore
//! use crate::middlewares::request_event::{add_context, context_key, set_user_id};
//!
//! set_user_id(&user.id);
//! add_context(context_key::CONTEXT, serde_json::json!({ "workflow_id": id, "steps": 4 }));
//! ```
//!
//! Pass ids and counts, not payloads. Everything here is written to a log line.

use std::future::{Ready, ready};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{Error, HttpMessage};
use serde_json::Value;

const REQUEST_ID_HEADER: &str = "request-id";

/// The agreed keys for [`add_context`].
///
/// The named ones are concerns any request can have, whatever it was asking
/// for, so a log query can filter on them across every route. Domain detail does
/// not get its own key, it goes under [`CONTEXT`] where the code doing the work
/// names its own fields.
///
/// [`CONTEXT`]: context_key::CONTEXT
pub mod context_key {
    /// A failure on our side, with its cause.
    pub const ERROR: &str = "error";
    /// How the caller authenticated, beyond the id on the line itself.
    pub const AUTH: &str = "auth";
    /// Database work done for the request.
    pub const DB: &str = "db";
    /// Whatever the code doing the work knows: `{ "workflow_id": ..., "steps": 4 }`.
    /// Merges, so several layers can each add their own fields.
    pub const CONTEXT: &str = "context";
}

// Task local, not thread local: requests interleave on a worker thread and would
// otherwise write into each other's record.
tokio::task_local! {
    static REQUEST_EVENT: Arc<Mutex<RequestEventFields>>;
}

/// What the log line is built from.
struct RequestEventFields {
    request_id: String,
    method: String,
    route: String,
    path: String,
    query_shape: String,
    user_agent: String,
    remote_addr: String,
    user_id: Option<String>,
    context: serde_json::Map<String, Value>,
}

/// Recovers a poisoned lock instead of panicking. Logging must not be what takes
/// a request down.
fn lock(fields: &Mutex<RequestEventFields>) -> MutexGuard<'_, RequestEventFields> {
    fields
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Runs `f` against the current request's record, or does nothing if there is no
/// request in scope.
fn with_event(f: impl FnOnce(&mut RequestEventFields)) {
    let _ = REQUEST_EVENT.try_with(|fields| f(&mut lock(fields)));
}

/// Records who the caller is.
///
/// The id only. Email and username are personal data, and the id looks either up
/// when there is a reason to.
pub fn set_user_id(id: &str) {
    with_event(|fields| fields.user_id = Some(id.to_owned()));
}

/// Adds context under `key`, combining with whatever is already there.
///
/// Use a constant from [`context_key`] where one fits.
pub fn add_context(key: &str, value: Value) {
    with_event(|fields| match fields.context.get_mut(key) {
        Some(existing) => combine(existing, value),
        None => {
            fields.context.insert(key.to_owned(), value);
        }
    });
}

/// Folds a second value into a key that is already present.
///
/// Objects merge, so several layers can each contribute fields to one subject.
/// Anything else collects into an array. Nothing is dropped.
fn combine(slot: &mut Value, incoming: Value) {
    match (slot, incoming) {
        (Value::Object(current), Value::Object(new)) => current.extend(new),
        (Value::Array(items), incoming) => items.push(incoming),
        (slot, incoming) => {
            let previous = slot.take();
            *slot = Value::Array(vec![previous, incoming]);
        }
    }
}

/// The id the current request will be logged under.
///
/// For code with no access to the request. Handlers read [`RequestId`] out of the
/// request extensions.
pub fn request_id() -> Option<String> {
    let mut id = None;
    with_event(|fields| id = Some(fields.request_id.clone()));
    id
}

/// The request id, readable by a handler through the request extensions.
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

impl RequestEventFields {
    fn extract(req: &ServiceRequest) -> Self {
        let header = |name: &str| {
            req.headers()
                .get(name)
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_owned()
        };

        // Reuse an inbound id so a request through a proxy keeps one id end to
        // end.
        let request_id = req
            .headers()
            .get(REQUEST_ID_HEADER)
            .and_then(|value| value.to_str().ok())
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| uuid::Uuid::now_v7().simple().to_string());

        Self {
            request_id,
            method: req.method().to_string(),
            // The route pattern, so the field stays low cardinality. Falls back
            // to the path for requests that matched no route.
            route: req
                .match_pattern()
                .unwrap_or_else(|| req.path().to_string()),
            path: req.path().to_string(),
            query_shape: query_shape(req.query_string()),
            user_agent: header("user-agent"),
            // The socket address, not `x-forwarded-for`, which a caller can set
            // to anything when the service is reachable directly.
            remote_addr: req
                .peer_addr()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_default(),
            user_id: None,
            context: serde_json::Map::new(),
        }
    }
}

/// The query's sorted, deduplicated key names. Values stay out of the logs.
fn query_shape(query: &str) -> String {
    if query.is_empty() {
        return String::new();
    }
    let mut keys: Vec<&str> = query
        .split('&')
        .filter_map(|pair| pair.split('=').next())
        .filter(|key| !key.is_empty())
        .collect();
    keys.sort_unstable();
    keys.dedup();
    keys.join(",")
}

pub struct RequestEvent;

impl<S, B> Transform<S, ServiceRequest> for RequestEvent
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestEventMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestEventMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct RequestEventMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestEventMiddleware<S>
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
        let fields = RequestEventFields::extract(&req);
        let request_id = fields.request_id.clone();

        req.extensions_mut().insert(RequestId(request_id.clone()));

        let service = self.service.clone();
        let event = Arc::new(Mutex::new(fields));

        Box::pin(REQUEST_EVENT.scope(event.clone(), async move {
            let started_at = Instant::now();
            let result = service.call(req).await;
            // Microseconds, rendered as milliseconds. Most requests finish inside a
            // millisecond and whole-millisecond truncation reports them all as zero.
            let duration_ms = started_at.elapsed().as_micros() as f64 / 1000.0;

            // A handler can fail with an error response or with Err. Both are
            // outcomes and both get logged, so lines and requests stay one to one.
            let status = match &result {
                Ok(response) => response.status(),
                Err(error) => error.as_response_error().status_code(),
            };

            emit(&lock(&event), status, duration_ms);

            let mut response = result?;
            if let Ok(value) = HeaderValue::from_str(&request_id) {
                response
                    .headers_mut()
                    .insert(HeaderName::from_static(REQUEST_ID_HEADER), value);
            }
            Ok(response)
        }))
    }
}

/// Writes the line. The arms carry identical fields and differ only in level.
fn emit(fields: &RequestEventFields, status: actix_web::http::StatusCode, duration_ms: f64) {
    let context = if fields.context.is_empty() {
        String::new()
    } else {
        Value::Object(fields.context.clone()).to_string()
    };

    macro_rules! line {
        ($level:ident, $message:literal) => {
            tracing::$level!(
                request_id = %fields.request_id,
                method = %fields.method,
                route = %fields.route,
                path = %fields.path,
                status = status.as_u16(),
                duration_ms,
                user_id = fields.user_id.as_deref().unwrap_or_default(),
                query_shape = %fields.query_shape,
                user_agent = %fields.user_agent,
                remote_addr = %fields.remote_addr,
                context = %context,
                $message
            )
        };
    }

    if status.is_server_error() {
        line!(error, "request failed");
    } else if status.is_client_error() {
        line!(warn, "request rejected");
    } else {
        line!(info, "request completed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn objects_under_one_key_merge() {
        let mut slot = json!({ "cause": "timeout" });
        combine(&mut slot, json!({ "retries": 2 }));
        assert_eq!(slot, json!({ "cause": "timeout", "retries": 2 }));
    }

    #[test]
    fn a_repeated_field_takes_the_later_value() {
        let mut slot = json!({ "status": "running" });
        combine(&mut slot, json!({ "status": "failed" }));
        assert_eq!(slot, json!({ "status": "failed" }));
    }

    #[test]
    fn non_objects_collect_rather_than_overwrite() {
        let mut slot = json!("first");
        combine(&mut slot, json!("second"));
        combine(&mut slot, json!("third"));
        assert_eq!(slot, json!(["first", "second", "third"]));
    }

    #[test]
    fn query_shape_keeps_keys_and_drops_values() {
        assert_eq!(query_shape("b=secret&a=1&b=other"), "a,b");
        assert_eq!(query_shape(""), "");
    }
}
