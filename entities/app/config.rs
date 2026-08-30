use anyhow::Context;
use dotenvy::var;

pub struct Environment {
    pub neo4j_uri: String,
    pub neo4j_user: String,
    pub neo4j_password: String,
    pub neo4j_database: Option<String>,
    pub app_port: u16,
    pub app_address: String,
    pub test_user_data: bool,
    pub token_secret: String,
}

impl Environment {
    pub fn load_env() -> anyhow::Result<Self> {
        let token_secret = var("TOKEN_SECRET").context("TOKEN_SECRET is required")?;
        anyhow::ensure!(
            token_secret.len() >= 32,
            "TOKEN_SECRET must be at least 32 bytes"
        );

        Ok(Self {
            neo4j_uri: var("NEO4J_URI").unwrap_or(String::from("bolt://localhost:7687")),
            neo4j_user: var("NEO4J_USER").unwrap_or(String::from("neo4j")),
            neo4j_password: var("NEO4J_PASSWORD").context("NEO4J_PASSWORD is required")?,
            neo4j_database: var("NEO4J_DATABASE").ok(),
            app_address: var("APP_ADDRESS").unwrap_or(String::from("127.0.0.1")),
            app_port: var("APP_PORT")
                .unwrap_or(String::from("4000"))
                .parse()
                .context("Invalid APP_PORT")?,
            test_user_data: var("TEST_USER_DATA")
                .unwrap_or(String::from("false"))
                .parse()
                .unwrap_or(false),
            token_secret,
        })
    }
}
