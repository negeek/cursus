use toasty::Db;

/// What a running server holds and shares across every request.
///
/// Built once at startup and cloned per request, which is cheap because the
/// database handle is itself a pooled handle rather than a connection.
///
/// This is deliberately separate from `ApplicationConfiguration`, which
/// describes how to build these things. Configuration is what you read from the
/// environment, state is what you got after acting on it. Keeping them apart is
/// what lets the tests construct state directly against a throwaway database
/// without going anywhere near environment variables.
#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

impl AppState {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}
