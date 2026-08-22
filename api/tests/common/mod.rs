//! Shared test helpers.
//!
//! Every integration test file compiles this module separately, and none of
//! them use all of it, so anything a given binary happens not to call would
//! otherwise be reported as dead code.
#![allow(dead_code)]

pub mod db;
pub mod path;
pub mod task;
pub mod user;
pub mod worflow_task_edge;
pub mod workflow;
pub mod workflow_task;

use api::app::ApplicationConfiguration;
use api::state::AppState;
use toasty::Db;

/// Opens a database that belongs to this test alone.
///
/// Replaces the old shared connection plus truncate. Each call gets a private
/// clone of the migrated schema, so tests no longer interfere with each other
/// and the suite runs in parallel.
pub async fn test_db() -> Db {
    let database_url = db::provision().await;
    let config = ApplicationConfiguration {
        database_url,
        ..Default::default()
    };
    config
        .resolve_db()
        .await
        .expect("failed to connect to the provisioned test database")
}

/// Builds the application exactly as the server does.
///
/// Goes through `build_app` rather than assembling routes here, so the tests
/// exercise the real middleware stack, including auth and request logging,
/// instead of an approximation that could drift from it.
pub async fn setup_app(
    db: Db,
) -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
    Error = actix_web::Error,
> {
    actix_web::test::init_service(ApplicationConfiguration::build_app(AppState::new(db))).await
}

/// The status of a call that was expected to be rejected.
///
/// A request refused by middleware comes back as `Err` rather than as a
/// response, so both shapes have to be reduced to a status before they can be
/// compared. Generic over the body type because the application's body is an
/// opaque type that callers cannot name.
pub fn rejected_status<B>(
    result: Result<actix_web::dev::ServiceResponse<B>, actix_web::Error>,
) -> actix_web::http::StatusCode {
    result
        .map(|response| response.status())
        .unwrap_or_else(|error| error.as_response_error().status_code())
}
