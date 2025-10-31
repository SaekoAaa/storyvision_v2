use sqlx::{Error, MySqlPool};

pub async fn get_db_health(pool: &MySqlPool) -> Result<String, String> {
    let query = "SELECT 1 FROM users LIMIT 1";
    match sqlx::query(query).fetch_one(pool).await {
        Ok(_) => Ok("Database is healthy and migrations are applied".to_string()),
        Err(err) => match err {
            Error::Database(db_err) => {
                // Пробуем извлечь код MySQL-ошибки
                let code = db_err.code().unwrap_or("unknown".into()).to_string();

                match code.as_str() {
                    "1146" => Err("Table missing — likely migrations not applied".to_string()),
                    "1049" => Err("Database not found".to_string()),
                    "1045" => Err("Invalid credentials for database".to_string()),
                    "2003" => Err("Database server unreachable".to_string()),
                    _ => Err(format!(
                        "Database error (code {}): {}",
                        code,
                        db_err.message()
                    )),
                }
            }
            Error::Io(e) => Err(format!("I/O error: {}", e)),
            Error::PoolTimedOut => Err("Connection pool timeout".to_string()),
            Error::PoolClosed => Err("Connection pool closed".to_string()),
            other => Err(format!("Unexpected error: {:?}", other)),
        },
    }
}
