use actix_web::{App, HttpResponse, HttpServer, error, web};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer().json())
        .init();

    tracing::info!("starting CURSUS server");

    dotenvy::dotenv().ok();
    tracing::info!("connecting to database");

    let db = match api::db::connection::connect(false).await {
        Ok(db) => {
            tracing::info!("database connection established");
            db
        }
        Err(e) => {
            tracing::error!(error = %e, "database connection failed");
            std::process::exit(1);
        }
    };

    if let Err(e) = db.get_schema_registry("api::db::entity::*").sync(&db).await {
        tracing::error!(error = %e, "schema sync failed");
        std::process::exit(1);
    }
    tracing::info!("database schema synced");

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default().error_handler(|err, _req| {
            tracing::warn!(error = %err, "json deserialization error");
            let resp = HttpResponse::BadRequest().json(api::dto::error::ApiError {
                error: err.to_string(),
            });
            error::InternalError::from_response(err, resp).into()
        });
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(db.clone()))
            .app_data(json_config)
            .configure(api::init)
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}
