use api::app::{ApplicationConfiguration, BindTarget};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    api::util::env_file::load(".env");

    let config = ApplicationConfiguration::from_env()?;

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(config.log_level.clone())),
        )
        .with(fmt::layer().json())
        .init();

    tracing::info!(port = config.port, "starting cursus");

    let bind_target = BindTarget::Address((config.host.clone(), config.port));
    config.run(bind_target).await
}
