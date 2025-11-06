use {
    crate::infrastructure::load_env::Enviroment,
    mysql::{Opts, Pool, TxOpts, prelude::Queryable},
    std::{fs, path::Path},
    tracing::level_filters::LevelFilter,
};
mod infrastructure;

fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    tracing::debug!(
        "Current path: {}",
        std::env::current_dir().unwrap().display()
    );
    if let Err(e) = dotenvy::dotenv() {
        tracing::debug!("Dotenv import 2 failed: {}. Fine for docker", e);
    };
    let env = Enviroment::load_env().expect("Loading enviroment variables error");
    let mysql_connection_string = format!(
        "mysql://{}:{}@{}:{}/{}",
        env.mysql_user,
        env.mysql_password,
        env.database.address,
        env.database.port,
        env.database.name
    );
    let opts = Opts::try_from(mysql_connection_string.as_str()).unwrap();
    let pool = Pool::new(opts).expect("Connecting to database");
    tracing::info!("Connected to database");
    let path = match env.is_revert {
        true => {
            tracing::info!("Reverting migrations");
            Path::new(&env.migrations_path).join("mysql_down.sql")
        }
        false => {
            tracing::info!("Applying migrations");
            Path::new(&env.migrations_path).join("mysql_up.sql")
        }
    };
    tracing::info!("Initialising migrations at: {}", env.migrations_path);
    let sql = fs::read_to_string(&path)
        .inspect_err(|_| {
            tracing::error!("Failed to read migration file at path: {}", path.display())
        })
        .unwrap();
    let mut tx = pool
        .start_transaction(TxOpts::default())
        .expect("Starting transaction");
    for stmt in sql.split(';') {
        let stmt = stmt.trim();
        if !stmt.is_empty() {
            if let Err(e) = tx.query_drop(stmt) {
                tracing::error!("Failed to execute migration statement: {}", e);
                tx.rollback().expect("Failed to rollback transaction");
                return;
            }
        }
    }

    tx.commit().expect("Failed to commit transaction");
    tracing::info!("Migrations complete");
}
