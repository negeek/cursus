//! What the request logging middleware promises.
//!
//! Two halves worth guarding. The request id has to survive a round trip and
//! come back to the caller, or nobody can quote it in a bug report. And the
//! enrichment helpers have to actually reach the emitted line, since the whole
//! reason they exist is to let a service add context without being handed a
//! logger, and a silent no-op there would look exactly like working code.

mod common;

use std::io;
use std::sync::{Arc, Mutex};

use actix_web::http::StatusCode;
// Aliased: an unqualified `use actix_web::test` also brings in actix's `#[test]`
// attribute, which then shadows the standard one and rejects any non-async test.
use actix_web::test as actix_test;
use api::middlewares::request_event::{add_context, set_user_id};
use common::{setup_app, test_db};
use tracing_subscriber::fmt::MakeWriter;

/// Collects everything the subscriber writes so a test can assert on it.
#[derive(Clone, Default)]
struct CapturedLogs(Arc<Mutex<Vec<u8>>>);

impl CapturedLogs {
    fn contents(&self) -> String {
        String::from_utf8_lossy(&self.0.lock().unwrap()).to_string()
    }
}

impl io::Write for CapturedLogs {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for CapturedLogs {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

#[actix_web::test]
async fn assigns_a_request_id_and_returns_it() {
    let app = setup_app(test_db().await).await;

    let response =
        actix_test::call_service(&app, actix_test::TestRequest::get().uri("/").to_request()).await;

    assert_eq!(response.status(), StatusCode::OK);
    let returned = response
        .headers()
        .get("request-id")
        .expect("every response should carry the id its request was logged under");
    assert!(
        !returned.to_str().unwrap().is_empty(),
        "the generated id should not be blank"
    );
}

#[actix_web::test]
async fn reuses_an_inbound_request_id() {
    let app = setup_app(test_db().await).await;

    // A proxy in front of the service may already have assigned one. Minting a
    // second here would split a single request across two ids in the logs.
    let response = actix_test::call_service(
        &app,
        actix_test::TestRequest::get()
            .uri("/")
            .insert_header(("request-id", "id-from-the-proxy"))
            .to_request(),
    )
    .await;

    assert_eq!(
        response.headers().get("request-id").unwrap(),
        "id-from-the-proxy"
    );
}

#[actix_web::test]
async fn enrichment_reaches_the_emitted_line() {
    let logs = CapturedLogs::default();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(logs.clone())
        .with_max_level(tracing::Level::INFO)
        .finish();

    // `set_default` binds the subscriber to the current thread, so it has to be
    // set from inside the runtime that will actually run the request. Setting it
    // outside and then entering a runtime captures whatever happens to share the
    // calling thread and silently misses the request itself.
    let _guard = tracing::subscriber::set_default(subscriber);

    let app = setup_app(test_db().await).await;
    let response =
        actix_test::call_service(&app, actix_test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(response.status(), StatusCode::OK);

    let captured = logs.contents();
    assert!(
        captured.contains("request completed"),
        "one line per request, and this request produced none: {captured}"
    );
    assert!(
        captured.contains("request_id"),
        "the line should carry the id it can be found by: {captured}"
    );
    assert!(
        captured.contains("duration_ms"),
        "the line should say how long it took: {captured}"
    );
}

/// The end to end version of the enrichment promise.
///
/// Auth runs inside this middleware and records the caller onto a record that
/// was opened before auth existed. If the middleware ordering in `app.rs` ever
/// gets inverted, the identity lands nowhere and this is what notices.
#[actix_web::test]
async fn records_the_caller_resolved_by_auth() {
    let logs = CapturedLogs::default();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(logs.clone())
        .with_max_level(tracing::Level::INFO)
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

    let mut db = test_db().await;
    let user = common::user::test_user(&mut db, None, None).await;
    let tokens = common::user::test_tokens(&mut db, Some(user.clone())).await;
    let app = setup_app(db).await;

    let response = actix_test::call_service(
        &app,
        actix_test::TestRequest::get()
            .uri("/api/v1/tasks")
            .insert_header(("Authorization", format!("Bearer {}", tokens.access)))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    let captured = logs.contents();
    assert!(
        captured.contains(&user.id.to_string()),
        "the caller auth resolved should reach the request line: {captured}"
    );

    // Identity is the id and nothing more. Email in a log line ends up in every
    // aggregator and export the logs reach, for no benefit the id does not give.
    assert!(
        !captured.contains(&user.email),
        "personal data should stay out of the logs: {captured}"
    );
}

#[test]
fn helpers_are_inert_outside_a_request() {
    // Service code is shared with the runner and with CLI commands, where there
    // is no request to enrich. Calling these there has to be harmless rather
    // than a panic waiting for the first background job.
    set_user_id("no-request-in-scope");
    add_context("workflow", serde_json::json!({ "steps": 3 }));
}
