pub mod path;
pub mod task;
pub mod user;
use actix_web::{App, test, web};
use api::init;
use sea_orm::{ConnectionTrait, DatabaseConnection};

pub async fn test_db() -> DatabaseConnection {
    dotenvy::dotenv().ok();
    let db = match api::db::connection::connect(true).await {
        Ok(db) => db,
        Err(e) => panic!("Database connection failed: {}", e),
    };
    if let Err(e) = db.get_schema_registry("api::db::entity::*").sync(&db).await {
        panic!("Failed to sync schema: {}", e);
    }
    db
}

pub async fn setup_app(
    db: DatabaseConnection,
) -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    test::init_service(App::new().app_data(web::Data::new(db)).configure(init)).await
}

pub async fn truncate_db(db: &DatabaseConnection) {
    if let Err(e) = db
        .execute_unprepared(
            "DO $$ DECLARE r RECORD; BEGIN \
             FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP \
                 EXECUTE 'TRUNCATE TABLE ' || quote_ident(r.tablename) || ' RESTART IDENTITY CASCADE'; \
             END LOOP; \
             END $$;",
        )
        .await
    {
        panic!("Failed to truncate tables: {}", e);
    }
}
