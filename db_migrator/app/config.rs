use dotenvy::var;
use strum_macros::IntoStaticStr;
use tracing::info;

#[derive(Debug, Clone, Copy, IntoStaticStr)]
pub enum MigrationType {
    ApplyMigration,
    RevertMigration,
    ApplyWithData,
    ApplyAndClearData,
    DryRun,
}

impl From<i32> for MigrationType {
    fn from(value: i32) -> Self {
        match value {
            1 => MigrationType::ApplyMigration,
            2 => MigrationType::RevertMigration,
            3 => MigrationType::ApplyWithData,
            4 => MigrationType::ApplyAndClearData,
            5 => MigrationType::DryRun,
            _ => MigrationType::DryRun, // default
        }
    }
}

pub struct Environment {
    pub database: String,
    pub database_port: String,
    pub database_address: String,
    pub mysql_user: String,
    pub mysql_password: String,
    pub migrations_path: String,
    pub migration_type: MigrationType,
    pub collector_url: Option<String>,
    pub with_tracing: bool,
    pub with_metrics: bool,
}

fn read_env(key: &str) -> anyhow::Result<String> {
    var(key).map_err(|e| anyhow::anyhow!("Missing env var {}: {}", key, e))
}

fn read_from_file(file_path: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(file_path)
        .map(|s| s.trim().to_string())
        .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", file_path, e))
}

impl Environment {
    #[tracing::instrument(name = "Environment::load_env", level = "debug", skip_all)]
    pub fn load_env() -> anyhow::Result<Self> {
        let database = read_env("MYSQL_DATABASE")?;
        let database_port = read_env("MYSQL_PORT").unwrap_or("3306".to_string());
        let database_address = read_env("MYSQL_ADDRESS")?;
        let migrations_path = read_env("MIGRATIONS_PATH")?;

        let migration_type: MigrationType = read_env("MIGRATION_TYPE")
            .map_or(1, |num| num.parse::<i32>().unwrap_or(1))
            .into();
            
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

        let collector_url = var("COLLECTOR_URL").ok();
        let with_tracing = var("WITH_TRACING").map_or(false, |t| t == "true");
        let with_metrics = var("WITH_METRICS").map_or(false, |t| t == "true");

        Ok(Self {
            database,
            mysql_user,
            mysql_password,
            migrations_path,
            database_port,
            database_address,
            migration_type,
            collector_url,
            with_tracing,
            with_metrics,
        })
    }
}
