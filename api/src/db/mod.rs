pub mod connection {
    use ::std::env;
    use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
    use std::time::Duration;

    /// Connects to the database using the DATABASE_URL environment variable,
    pub async fn connect() -> Result<DatabaseConnection, DbErr> {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be provided");
        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false); // disable SQLx logging
        let db = Database::connect(opt).await?;
        return Ok(db);
    }

    /// Checks if the database connection is alive by pinging it,
    /// then closes the connection and verifies that it is closed.
    pub async fn check(db: DatabaseConnection) {
        assert!(db.ping().await.is_ok());
        let _ = db.clone().close().await;
        assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));
    }
}
