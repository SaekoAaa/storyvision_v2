use anyhow::Context;
use dotenvy::var;

pub struct Environment {
    pub mysql_database: String,
    pub mysql_port: String,
    pub mysql_user: String,
    pub mysql_password: String,
    pub db_address: String,
    pub app_port: u16,
    pub app_address: String,
    pub salt: String,
}
impl Environment {
    pub fn load_env() -> anyhow::Result<Self> {
        Ok(Self {
            mysql_database: var("MYSQL_DATABASE").context("MYSQL_DATABASE")?,
            mysql_port: var("MYSQL_PORT").unwrap_or(String::from("3306")),
            mysql_user: var("MYSQL_USER").context("MYSQL_USER")?,
            mysql_password: var("MYSQL_PASSWORD").context("MYSQL_PASSWORD")?,
            db_address: var("DB_ADDRESS").unwrap_or(String::from("127.0.0.1")),
            app_address: var("APP_ADDRESS").context("APP_ADDRESS")?,
            salt: var("SALT")
                .context("SALT")
                .unwrap_or("12345salt".to_string()),
            app_port: var("APP_PORT")
                .unwrap_or(String::from("4000"))
                .parse()
                .unwrap(),
        })
    }
}
