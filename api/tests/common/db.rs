//! Gives every test its own database, so the suite runs in parallel.
//!
//! The old approach shared one database and truncated it between tests, which
//! meant nothing could run concurrently: any two tests would clobber each
//! other's rows. That is why the suite needed `--test-threads=1`.
//!
//! Instead, one fully migrated template database is built once per test binary,
//! and each test clones it with `CREATE DATABASE ... TEMPLATE ...`. Postgres
//! copies at the filesystem level, so a clone is fast and completely isolated.
//! No truncation, no shared state, no ordering between tests.

use std::sync::Arc;

use api::app::ApplicationConfiguration;
use tokio::sync::{Mutex, OnceCell};
use url::Url;

static TEMPLATE: OnceCell<String> = OnceCell::const_new();
static TEMPLATE_LOCK: OnceCell<Arc<Mutex<()>>> = OnceCell::const_new();

/// The server the tests connect to, taken from the environment.
///
/// This has to be reachable from the host running the tests, so it is not the
/// same value the application container uses, where the database is reachable
/// under its compose service name instead.
fn base_url() -> Url {
    load_env();
    let raw = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("TEST_DATABASE_URL or DATABASE_URL must be set to run the tests");
    Url::parse(&raw).expect("database url must be a valid url")
}

/// Loads `.env` once per test binary.
///
/// Tests need more than the database url from it: the token codec reads
/// `SECRET`, and without it every authenticated test panics.
pub fn load_env() {
    static LOADED: std::sync::Once = std::sync::Once::new();
    LOADED.call_once(|| {
        // Tests run from the crate directory, the file sits at the repo root.
        api::util::env_file::load(".env");
        api::util::env_file::load("../.env");
    });
}

/// Connection to the maintenance database, which is where `CREATE DATABASE` has
/// to be issued from since it cannot run inside the database being copied.
fn admin_url(base: &Url) -> Url {
    let mut url = base.clone();
    url.set_path("/postgres");
    url
}

fn url_for(base: &Url, name: &str) -> Url {
    let mut url = base.clone();
    url.set_path(&format!("/{name}"));
    url
}

async fn connect_admin(url: &str) -> tokio_postgres::Client {
    let (client, connection) = tokio_postgres::connect(url, tokio_postgres::NoTls)
        .await
        .expect("failed to connect to the test postgres server");

    // The connection drives the protocol and has to be polled for the client to
    // work, so it runs on its own task for as long as the client is alive.
    tokio::spawn(async move {
        let _ = connection.await;
    });

    client
}

/// Names the template after the test binary.
///
/// Each integration test file compiles to its own binary and they run at the
/// same time, so a shared template name would have them dropping and recreating
/// each other's template mid-run.
fn template_name() -> String {
    let binary = std::env::current_exe()
        .ok()
        .and_then(|path| path.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "api".to_string());

    // Cargo appends a hash to the test binary name, which changes on every
    // rebuild. Trimming it keeps the template stable across runs.
    let stem = binary.split('-').next().unwrap_or(&binary);
    format!("cursus_test_template_{stem}")
}

async fn build_template(base: &Url, name: &str) {
    let admin = connect_admin(admin_url(base).as_str()).await;

    // A previous run may have left the template behind, possibly with an older
    // schema. Drop it rather than trusting it. Connections have to be closed
    // first or the drop is refused.
    let _ = admin
        .batch_execute(&format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity \
             WHERE datname = '{name}' AND pid <> pg_backend_pid()"
        ))
        .await;
    let _ = admin
        .batch_execute(&format!("DROP DATABASE IF EXISTS \"{name}\""))
        .await;
    admin
        .batch_execute(&format!("CREATE DATABASE \"{name}\""))
        .await
        .expect("failed to create the template database");
    drop(admin);

    // Migrations come from the same embedded set the server applies at startup,
    // so the schema under test is the schema that ships.
    let config = ApplicationConfiguration {
        database_url: url_for(base, name).to_string(),
        ..Default::default()
    };
    let db = config
        .resolve_db()
        .await
        .expect("failed to connect to the template database");
    ApplicationConfiguration::apply_migrations(&db)
        .await
        .expect("failed to migrate the template database");
}

/// Builds the template once, however many tests ask for it at once.
async fn ensure_template() -> String {
    if let Some(name) = TEMPLATE.get() {
        return name.clone();
    }

    let lock = TEMPLATE_LOCK
        .get_or_init(|| async { Arc::new(Mutex::new(())) })
        .await;
    let _guard = lock.lock().await;

    // Checked again inside the lock: several tests can get past the check above
    // before any of them takes it.
    if let Some(name) = TEMPLATE.get() {
        return name.clone();
    }

    let base = base_url();
    let name = template_name();
    build_template(&base, &name).await;
    TEMPLATE
        .set(name.clone())
        .expect("template was initialised more than once");
    name
}

/// A fresh, fully migrated database, private to the caller.
pub async fn provision() -> String {
    let template = ensure_template().await;
    let base = base_url();
    let name = format!("cursus_test_{}", uuid::Uuid::now_v7().simple());

    let admin = connect_admin(admin_url(&base).as_str()).await;
    admin
        .batch_execute(&format!(
            "CREATE DATABASE \"{name}\" TEMPLATE \"{template}\""
        ))
        .await
        .expect("failed to clone the test database from the template");

    url_for(&base, &name).to_string()
}
