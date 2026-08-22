//! One request in, one structured log line out.
//!
//! Rather than scattering log calls along the request path and stitching them
//! together afterwards, every request produces exactly one record carrying what
//! actually matters: what was asked for, what came back, how long it took, and
//! an id to find it by.
//!
//! Deliberately free of any particular observability backend. The request id is
//! attached to a tracing span, so if a Sentry or OpenTelemetry layer is ever
//! added it reads the same span and the same fields without this middleware
//! changing.
//!
//! The `request-id` header is named plainly on purpose. Its scope is one inbound
//! HTTP request and it stops there. It does NOT follow a task dispatch out to a
//! user's endpoint and come back on the callback that arrives minutes later,
//! because those are two separate requests with an arbitrary gap between them.
//! Correlating a dispatch with its callback is `workflow_task_state.id`'s job,
//! and belongs to the runner.

use std::future::{Ready, ready};
use std::pin::Pin;
use std::rc::Rc;
use std::time::Instant;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{Error, HttpMessage};
use tracing::Instrument;

const REQUEST_ID_HEADER: &str = "request-id";

/// The request id assigned to the request currently being handled.
///
/// Put in the request extensions so a handler can read it, most usefully to
/// include in an error response so a user can quote it in a bug report.
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

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
        // Reuse whatever the caller sent, so a request passing through a proxy
        // that already assigned an id keeps one id end to end rather than
        // picking up a second one here.
        let request_id = req
            .headers()
            .get(REQUEST_ID_HEADER)
            .and_then(|value| value.to_str().ok())
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| uuid::Uuid::now_v7().simple().to_string());

        req.extensions_mut().insert(RequestId(request_id.clone()));

        let method = req.method().to_string();
        // The matched route pattern rather than the raw path, so the field stays
        // low cardinality: a thousand requests for different task ids group under
        // one route instead of a thousand distinct values. Falls back to the raw
        // path for requests that matched nothing, such as a 404.
        let route = req
            .match_pattern()
            .unwrap_or_else(|| req.path().to_string());

        let span = tracing::info_span!(
            "http_request",
            request_id = %request_id,
            method = %method,
            route = %route,
        );

        let service = self.service.clone();
        let started_at = Instant::now();

        Box::pin(
            async move {
                let result = service.call(req).await;
                let duration_ms = started_at.elapsed().as_millis();

                // A handler can fail either by returning an error response or by
                // returning Err. Both are real outcomes and both get logged, so
                // the count of log lines matches the count of requests either way.
                let status = match &result {
                    Ok(response) => response.status(),
                    Err(error) => error.as_response_error().status_code(),
                };

                if status.is_server_error() {
                    tracing::error!(
                        status = status.as_u16(),
                        duration_ms = duration_ms as u64,
                        "request failed"
                    );
                } else if status.is_client_error() {
                    tracing::warn!(
                        status = status.as_u16(),
                        duration_ms = duration_ms as u64,
                        "request rejected"
                    );
                } else {
                    tracing::info!(
                        status = status.as_u16(),
                        duration_ms = duration_ms as u64,
                        "request completed"
                    );
                }

                // Echo the id back so a caller can quote it when reporting a
                // problem, and find it in the logs.
                let mut response = result?;
                if let Ok(value) = HeaderValue::from_str(&request_id) {
                    response
                        .headers_mut()
                        .insert(HeaderName::from_static(REQUEST_ID_HEADER), value);
                }
                Ok(response)
            }
            .instrument(span),
        )
    }
}
