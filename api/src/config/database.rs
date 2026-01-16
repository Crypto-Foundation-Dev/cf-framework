use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn connect_db(
    host: String,
    username: String,
    password: String,
    port: String,
    db_name: String,
) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, db_name
    );

    let connect_options = ConnectOptions::new(url)
        .max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true)
        .to_owned();

    let db = Database::connect(connect_options).await?;

    Ok(db)
}