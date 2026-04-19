use actix_web::{App, HttpResponse, HttpServer, error, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting CURSUS server...");
    dotenvy::dotenv().ok();
    println!("Connecting to database...");
    let db = match api::db::connection::connect().await {
        Ok(db) => {
            println!("Database connection successful!");
            db
        }
        Err(e) => {
            println!("Database connection failed: {}", e);
            std::process::exit(1);
        }
    };
    if let Err(e) = db.get_schema_registry("api::db::entity::*").sync(&db).await {
        println!("Failed to sync schema: {}", e);
        std::process::exit(1);
    }
    println!("Database schema synced successfully!");
    HttpServer::new(move || {
        let json_config = web::JsonConfig::default().error_handler(|err, _req| {
            // Log the error for debugging purposes
            println!("JSON deserialization error: {}", err);
            let resp = HttpResponse::BadRequest().json(api::dto::error::ApiError {
                error: err.to_string(),
            });
            error::InternalError::from_response(err, resp).into()
        });
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(json_config)
            .configure(api::init)
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}
