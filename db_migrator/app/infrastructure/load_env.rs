use anyhow::Context;
use dotenvy::var;

pub struct Environment {
    pub mysql_database: String,
    pub mysql_port: String,
    pub mysql_user: String,
    pub mysql_password: String,
    pub db_address: String,
    pub migrations_path: String,
    pub is_revert: bool,
}
impl Environment {
    pub fn load_env() -> anyhow::Result<Self> {
        Ok(Self {
            mysql_database: var("MYSQL_DATABASE").context("MYSQL_DATABASE")?,
            mysql_port: var("MYSQL_PORT").unwrap_or(String::from("3306")),
            mysql_user: var("MYSQL_USER").context("MYSQL_USER")?,
            mysql_password: var("MYSQL_PASSWORD").context("MYSQL_PASSWORD")?,
            db_address: var("DB_ADDRESS").unwrap_or(String::from("127.0.0.1")),
            migrations_path: var("MIGRATIONS_PATH").context("MIGRATIONS_PATH")?,
            is_revert: var("IS_REVERT").unwrap_or(String::from("false")) == "true",
        })
    }
}
