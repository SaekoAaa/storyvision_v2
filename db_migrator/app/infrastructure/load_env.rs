use std::fs::{File, read_to_string};

use anyhow::Context;
use dotenvy::var;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Database {
    pub port: String,
    pub name: String,
    pub address: String,
}
#[derive(serde::Deserialize)]
pub struct Enviroment {
    pub database: Database,
    #[serde(skip)]
    pub mysql_user: String,
    #[serde(skip)]
    pub mysql_password: String,
    pub migrations_path: String,
    #[serde(skip)]
    pub is_revert: bool,
}
impl Enviroment {
    pub fn load_env() -> anyhow::Result<Self> {
        let config_path = var("CONFIG_PATH")
            .context("Path of configuration file")
            .unwrap_or("./config.yaml".to_string());
        let file = File::open(&config_path).expect("Should open configuration file");
        let mut env: Enviroment = serde_yaml::from_reader(file).context(format!(
            "Failed to parse yaml file at path: {}",
            config_path
        ))?;
        env.mysql_user = var("MYSQL_USER").context("Searching for MYSQL_USER env")?;
        env.mysql_password = var("MYSQL_PASSWORD").context("Searching for MYSQL_PASSWORD env")?;
        env.is_revert = var("IS_REVERT").unwrap_or("false".to_string()) == "true";
        Ok(env)
    }
}
