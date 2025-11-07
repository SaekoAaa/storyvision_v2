
use dotenvy::var;
use tracing::info;

pub struct Enviroment {
    pub database: String,
    pub database_port: String,
    pub database_address: String,
    pub mysql_user: String,
    pub mysql_password: String,
    pub migrations_path: String,
    pub is_revert: bool,
}

fn read_env(key: &str) -> anyhow::Result<String> {
    var(key).map_err(|e| anyhow::anyhow!("Missing env var {}: {}", key, e))
}
fn read_from_file(file_path: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(file_path)
        .map(|s| s.trim().to_string())
        .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", file_path, e))
}
impl Enviroment {
    #[tracing::instrument(name = "Enviroment::load_env", level = "debug", skip_all)]
    pub fn load_env() -> anyhow::Result<Self> {
        let database = read_env("MYSQL_DATABASE")?;
        let database_port = read_env("MYSQL_PORT")?;
        let database_address = read_env("MYSQL_ADDRESS")?;
        let migrations_path = read_env("MIGRATIONS_PATH")?;
        let is_revert = var("IS_REVERT").map_or(false, |v| v == "true");

        let mysql_user = match var("MYSQL_USER") {
            Ok(user) => user,
            Err(_) => {
                info!("MYSQL_USER not found, using MYSQL_USER_FILE");
                let user_path = read_env("MYSQL_USER_FILE")?;
                read_from_file(&user_path)?
            }
        };

        let mysql_password = match var("MYSQL_PASSWORD") {
            Ok(pwd) => pwd,
            Err(_) => {
                info!("MYSQL_PASSWORD not found, using MYSQL_PASSWORD_FILE");
                let pwd_path = read_env("MYSQL_PASSWORD_FILE")?;
                read_from_file(&pwd_path)?
            }
        };

        let env = Self {
            database,
            mysql_user,
            mysql_password,
            migrations_path,
            is_revert,
            database_port,
            database_address,
        };

        Ok(env)
    }
}
