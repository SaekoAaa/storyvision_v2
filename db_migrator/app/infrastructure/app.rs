use std::{fs::read_to_string, path::Path};

use mysql::{Opts, Pool};
use opentelemetry::KeyValue;
use tracing::debug_span;

use crate::{
    infrastructure::{
        apply_tx::apply_transaction, database::connect_to_database, load_env::Enviroment,
    },
    observability::metrics::{MIGRATIONS_COUNTER, MIGRATIONS_DURATION},
};

#[tracing::instrument(name = "load_env", level = "info")]
pub fn run() -> anyhow::Result<()> {
    let env = Enviroment::load_env().inspect_err(|e| {
        tracing::error!(error = %e, "Failed to load environment");
    })?;
    let mysql_connection_string = format!(
        "mysql://{}:{}@{}:{}/{}",
        env.mysql_user, env.mysql_password, env.database_address, env.database_port, env.database
    );
    tracing::info!("connecting to db with: {}", mysql_connection_string);
    let pool = connect_to_database(&mysql_connection_string)
        .inspect_err(|error| tracing::error!(%error, database = env.database, database.user = env.mysql_user, database.address = env.database_address, database.port = env.database_port, "Connecting to database error"))?;
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
    let read_sql_span = debug_span!("reading_sql", path = &path.to_str());
    let sql = read_sql_span.in_scope(|| {
        read_to_string(&path)
            .inspect_err(|_| {
                tracing::error!("Failed to read migration file at path: {}", path.display())
            })
            .unwrap()
    });
    let timer = std::time::Instant::now();
    apply_transaction(pool, &sql).unwrap();

    let kv = &[KeyValue::new("is_revert", env.is_revert)];
    if let Some(migrations_counter) = MIGRATIONS_COUNTER.get() {
        migrations_counter.add(1, kv);
    }
    if let Some(migrations_duration) = MIGRATIONS_DURATION.get() {
        migrations_duration.record(timer.elapsed().as_secs_f64(), kv);
    }
    tracing::info!("Process finished");
    Ok(())
}
