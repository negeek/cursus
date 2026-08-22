use std::net::TcpListener;

use actix_web::dev::{Server, ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{App, HttpResponse, HttpServer, body::MessageBody, error, web};
use toasty::Db;
use toasty::migration::MigrationSet;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::dtos::error::ApiError;
use crate::middlewares::request_event::RequestEvent;
use crate::state::AppState;
use crate::{handlers, middlewares, openapi};

/// The schema, compiled into the binary at build time.
///
/// This is why there is no migration step to run before starting the server and
/// no migration files to ship beside it. The binary carries its own schema and
/// applies whatever is outstanding on startup, so a fresh deployment is one
/// container with nothing to sequence.
///
/// The macro reads `toasty/migrations` at compile time, so a build that cannot
/// see that directory produces an empty set rather than failing. `run` logs how
/// many were applied for exactly that reason.
static MIGRATIONS: MigrationSet = toasty::embed_migrations!();

/// Where the server should listen.
///
/// Tests bind port zero to get whatever is free and then need to know which port
/// they got, which a host and port pair cannot express. Handing over an already
/// bound listener can.
pub enum BindTarget {
    Address((String, u16)),
    Listener(TcpListener),
}

/// Everything needed to build and start the server, and nothing that is only
/// known once it is running.
pub struct ApplicationConfiguration {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub log_level: String,
}

impl Default for ApplicationConfiguration {
    fn default() -> Self {
        Self {
            database_url: String::new(),
            host: "0.0.0.0".to_string(),
            port: 8080,
            workers: 4,
            log_level: "info".to_string(),
        }
    }
}

impl ApplicationConfiguration {
    /// Reads configuration from the environment, falling back to the defaults
    /// above for everything except the database url, which has no sensible
    /// default and must be provided.
    pub fn from_env() -> Result<Self, std::io::Error> {
        let defaults = Self::default();
        let database_url = std::env::var("DATABASE_URL").map_err(|_| {
            std::io::Error::other("DATABASE_URL must be set")
        })?;

        Ok(Self {
            database_url,
            host: std::env::var("HOST").unwrap_or(defaults.host),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(defaults.port),
            workers: std::env::var("WORKERS")
                .ok()
                .and_then(|w| w.parse().ok())
                .unwrap_or(defaults.workers),
            log_level: std::env::var("LOG_LEVEL").unwrap_or(defaults.log_level),
        })
    }

    /// Opens the database connection this configuration describes.
    pub async fn resolve_db(&self) -> Result<Db, std::io::Error> {
        Db::builder()
            .models(crate::all_models!())
            .connect(&self.database_url)
            .await
            .map_err(|e| std::io::Error::other(format!("database connection failed: {e}")))
    }

    /// Applies any migrations the database has not seen yet.
    ///
    /// Safe to call on every start. Toasty tracks what has already run and skips
    /// it, so this is a no-op once the schema is current.
    pub async fn apply_migrations(db: &Db) -> Result<usize, std::io::Error> {
        let report = MIGRATIONS
            .apply(db)
            .await
            .map_err(|e| std::io::Error::other(format!("migration failed: {e}")))?;
        Ok(report.applied())
    }

    /// Assembles the actix app.
    ///
    /// Shared by `run` and by the test harness, so tests exercise the same
    /// middleware stack and the same routes as production rather than an
    /// approximation of them.
    ///
    /// An associated function rather than a method: `HttpServer::new` calls its
    /// closure once per worker thread, so what the closure returns cannot borrow
    /// from anything. Taking `&self` would tie the returned `impl Trait` to that
    /// borrow, and taking `self` by value would mean cloning the whole
    /// configuration, database url and all, per worker just to read nothing.
    pub fn build_app(
        state: AppState,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<impl MessageBody>,
            Error = actix_web::Error,
            InitError = (),
        >,
    > {
        // Turn a malformed request body into the same error shape as everything
        // else, rather than actix's default plain text.
        let json_config = web::JsonConfig::default().error_handler(|err, _req| {
            let response = HttpResponse::BadRequest().json(ApiError {
                error: err.to_string(),
            });
            error::InternalError::from_response(err, response).into()
        });

        App::new()
            .app_data(web::Data::new(state))
            .app_data(json_config)
            .wrap(RequestEvent)
            .service(handlers::root)
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-docs/openapi.json", openapi::ApiDoc::openapi()),
            )
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/account")
                            .service(handlers::user::sign_up)
                            .service(handlers::user::signin)
                            .service(handlers::user::verify_email)
                            .service(
                                web::resource("/logout")
                                    .wrap(middlewares::user::Auth)
                                    .route(web::post().to(handlers::user::logout)),
                            )
                            .service(handlers::user::refresh_access_token),
                    )
                    .service(
                        web::scope("")
                            .wrap(middlewares::user::Auth)
                            .service(handlers::task::get_tasks)
                            .service(handlers::task::create_task)
                            .service(handlers::task::get_task)
                            .service(handlers::task::edit_task)
                            .service(handlers::task::delete_task)
                            .service(handlers::workflow::get_workflows)
                            .service(handlers::workflow::create_workflow)
                            .service(handlers::workflow::get_workflow_detail)
                            .service(handlers::workflow::get_workflow)
                            .service(handlers::workflow::edit_workflow)
                            .service(handlers::workflow::delete_workflow)
                            .service(handlers::workflow_task::create_workflow_task)
                            .service(handlers::workflow_task::get_workflow_task)
                            .service(handlers::workflow_task::edit_workflow_task)
                            .service(handlers::workflow_task::delete_workflow_task)
                            .service(handlers::workflow_task_edge::create_workflow_task_edge)
                            .service(handlers::workflow_task_edge::edit_workflow_task_edge)
                            .service(handlers::workflow_task_edge::delete_workflow_task_edge),
                    ),
            )
    }

    pub async fn run(&self, bind_target: BindTarget) -> Result<(), std::io::Error> {
        let db = self.resolve_db().await?;
        tracing::info!("database connection established");

        let applied = Self::apply_migrations(&db).await?;
        tracing::info!(applied, "migrations up to date");

        let state = AppState::new(db);

        let builder = HttpServer::new(move || Self::build_app(state.clone())).workers(self.workers);

        let server: Server = match bind_target {
            BindTarget::Address((host, port)) => builder.bind((host, port))?,
            BindTarget::Listener(listener) => builder.listen(listener)?,
        }
        .run();

        server.await
    }
}
