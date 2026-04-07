use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting CURSUS server...");
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
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(api::init)
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}
