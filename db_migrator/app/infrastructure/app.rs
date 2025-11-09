use std::{fs::read_to_string, path::Path};

use mysql::{Opts, Pool};
use opentelemetry::KeyValue;
use tracing::debug_span;

use crate::{
    infrastructure::{
        apply_tx::{apply_separately, apply_transaction},
        database::connect_to_database,
        load_env::{Enviroment, MigrationType},
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

    let timer = std::time::Instant::now();
    match env.migration_type {
        MigrationType::RevertMigration => {
            tracing::info!("Reverting migrations");
            let path = Path::new(&env.migrations_path).join("mysql_down.sql");
            tracing::info!("Initialising migrations at: {}", env.migrations_path);
            let span = debug_span!("Reverting migrations", path = &path.to_str());
            span.in_scope(|| {
                let sql = read_to_string(&path)
                    .inspect_err(|_| {
                        tracing::error!("Failed to read migration file at path: {}", path.display())
                    })
                    .unwrap();
                apply_transaction(&pool, &sql).unwrap();
            });
        }
        MigrationType::ApplyMigration => {
            tracing::info!("Applying migrations");
            let path = Path::new(&env.migrations_path).join("mysql_up.sql");
            tracing::info!("Initialising migrations at: {}", env.migrations_path);
            let span = debug_span!("Applying migrations", path = &path.to_str());
            span.in_scope(|| {
                let sql = read_to_string(&path)
                    .inspect_err(|_| {
                        tracing::error!("Failed to read migration file at path: {}", path.display())
                    })
                    .unwrap();
                apply_transaction(&pool, &sql).unwrap();
            });
        }
        MigrationType::ApplyWithData => {
            tracing::info!("Applying migrations and filling data");
            let path = Path::new(&env.migrations_path).join("mysql_up.sql");
            tracing::info!("Initialising migrations at: {}", env.migrations_path);
            let span = debug_span!("applying_migration", path = &path.to_str());
            span.in_scope(|| {
                let sql = read_to_string(&path)
                    .inspect_err(|_| {
                        tracing::error!("Failed to read migration file at path: {}", path.display())
                    })
                    .unwrap();
                apply_transaction(&pool, &sql).unwrap();
            });
            tracing::info!("Filling data to database");

            let span = debug_span!("Filling sql data", path = &path.to_str());
            let fill_data_sql = Path::new(&env.migrations_path).join("mysql_fill_data.sql");
            span.in_scope(|| {
                let fill_data_sql = read_to_string(&fill_data_sql)
                    .inspect_err(|_| {
                        tracing::error!(
                            "Failed to read fill data file at path: {}",
                            fill_data_sql.display()
                        )
                    })
                    .unwrap();
                apply_separately(&pool, &fill_data_sql).unwrap();
            });
        }
        MigrationType::ApplyAndClearData => {
            tracing::info!("Applying migrations and clearing data");
            let path = Path::new(&env.migrations_path).join("mysql_up.sql");
            tracing::info!("Initialising migrations at: {}", env.migrations_path);
            let span = debug_span!("applying_migration", path = &path.to_str());
            span.in_scope(|| {
                let sql = read_to_string(&path)
                    .inspect_err(|_| {
                        tracing::error!("Failed to read migration file at path: {}", path.display())
                    })
                    .unwrap();
                apply_transaction(&pool, &sql).unwrap();
            });
            tracing::info!("Clearing database");

            let span = debug_span!("Filling sql data", path = &path.to_str());
            let fill_data_sql = Path::new(&env.migrations_path).join("mysql_drop_data.sql");
            span.in_scope(|| {
                let fill_data_sql = read_to_string(&fill_data_sql)
                    .inspect_err(|_| {
                        tracing::error!(
                            "Failed to read drop data file at path: {}",
                            fill_data_sql.display()
                        )
                    })
                    .unwrap();
                apply_transaction(&pool, &fill_data_sql).unwrap();
            });
        }
    };

    let kv = &[KeyValue::new(
        "migration_mode",
        Into::<&'static str>::into(env.migration_type),
    )];
    if let Some(migrations_counter) = MIGRATIONS_COUNTER.get() {
        migrations_counter.add(1, kv);
    }
    if let Some(migrations_duration) = MIGRATIONS_DURATION.get() {
        migrations_duration.record(timer.elapsed().as_secs_f64(), kv);
    }
    tracing::info!("Process finished");
    Ok(())
}
