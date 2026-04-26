pub mod path;
use actix_web::{App, test, web};
use api::init;
use sea_orm::{ConnectionTrait, DatabaseConnection};

pub async fn test_db() -> DatabaseConnection {
    dotenvy::dotenv().ok();
    println!("Connecting to database...");
    let db = match api::db::connection::connect(true).await {
        Ok(db) => {
            println!("Database connection successful!");
            db
        }
        Err(e) => {
            panic!("Database connection failed: {}", e);
        }
    };
    if let Err(e) = db.get_schema_registry("api::db::entity::*").sync(&db).await {
        panic!("Failed to sync schema: {}", e);
    }
    db
}

pub async fn setup_app() -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    let db = test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(init),
    )
    .await;
    app
}

pub async fn teardown_db(db: DatabaseConnection) {
    println!("Tearing down database...");
    println!("Deleting all tables...");
    if let Err(e) = db
        .execute_unprepared("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
        .await
    {
        panic!("Failed to drop schema: {}", e);
    }
    let _ = db.close().await;
}
