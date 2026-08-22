//! Migration tooling for cursus.
//!
//! This is a development tool, not part of the running service. It exists to
//! turn model changes into migration files. Applying those files in production
//! happens inside the server itself, which embeds them at compile time, so this
//! binary never needs to be deployed.
//!
//! It is behind the `cli` feature so its argument parsing never ends up in the
//! server binary.
//!
//! Generate a migration after changing a model:
//! ```text
//! cargo run -p api --features cli --bin migrator -- migration generate --name <name>
//! ```
//!
//! Apply pending migrations by hand, which the server also does on startup:
//! ```text
//! cargo run -p api --features cli --bin migrator -- migration apply
//! ```
//!
//! Other commands: `migration snapshot`, `migration drop [--name|--latest]`,
//! `migration reset`.

use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = Config::load()?;

    let db = toasty::Db::builder()
        .models(api::all_models!())
        .connect(&database_url)
        .await?;

    ToastyCli::with_config(db, config).parse_and_run().await?;

    Ok(())
}
