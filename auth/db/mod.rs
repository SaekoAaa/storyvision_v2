use {
    sqlx::{MySqlPool, migrate::Migrator},
    std::path::Path,
};

pub async fn init_database(url: &str) -> Result<MySqlPool, sqlx::Error> {
    tracing::debug!("Connecting to database with url: {}", url);
    let pool = MySqlPool::connect(url).await?;
    tracing::debug!("Connected to database");
    Ok(pool)
}
