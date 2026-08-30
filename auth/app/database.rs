use sqlx::MySqlPool;

pub async fn init_database(url: &str) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPool::connect(url).await?;
    Ok(pool)
}
