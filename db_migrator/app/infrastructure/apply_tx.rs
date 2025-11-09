use mysql::{Pool, TxOpts, prelude::Queryable};
use tracing::Span;
#[tracing::instrument(skip_all)]
pub fn apply_transaction(pool: &Pool, sql: &str) -> anyhow::Result<()> {
    let mut tx = pool.start_transaction(TxOpts::default())?;
    let tx_span = Span::current();
    for (i, stmt) in sql.split(';').enumerate() {
        let stmt = stmt.trim();
        if !stmt.is_empty() {
            let stmt_span = tracing::info_span!(
                "migration_step",
                statement_index = i,
                statement_preview = &stmt[..stmt.len().min(40)]
            );
            let _enter = stmt_span.enter();
            if let Err(e) = tx.query_drop(stmt) {
                tracing::error!("Failed to execute migration statement: {}", e);
                tx.rollback()?;
                tx_span.record("migration_status", "rolled_back");
                return Err(anyhow::anyhow!(e));
            }
            tracing::info!("Finished migration: {}", i);
        }
    }
    tx.commit()?;
    tracing::info!("Migration transaction committed successfully");
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn apply_separately(pool: &Pool, sql: &str) -> anyhow::Result<()> {
    let mut tx = pool.start_transaction(TxOpts::default())?;
    let mut tx_span = Span::current();

    for (i, stmt) in sql.split(';').enumerate() {
        let stmt = stmt.trim();

        if stmt.is_empty() {
            continue;
        }

        // Проверка на маркер транзакции
        if stmt == "-- tx" {
            tracing::info!("Transaction boundary detected, committing current transaction");
            tx.commit()?;
            tx_span.record("migration_status", "committed");
            tracing::info!("Starting new transaction");

            // Начинаем новую транзакцию
            tx = pool.start_transaction(TxOpts::default())?;
            tx_span = tracing::info_span!("migration_transaction", transaction_index = i);
            continue;
        }

        let stmt_span = tracing::info_span!(
            "migration_step",
            statement_index = i,
            statement_preview = &stmt[..stmt.len().min(40)]
        );
        let _enter = stmt_span.enter();

        if let Err(e) = tx.query_drop(stmt) {
            tracing::error!("Failed to execute migration statement: {}", e);
            tx.rollback()?;
            tx_span.record("migration_status", "rolled_back");
            return Err(anyhow::anyhow!(e));
        }

        tracing::info!("Finished migration step: {}", i);
    }

    // Коммитим последнюю транзакцию
    tx.commit()?;
    tracing::info!("Final migration transaction committed successfully");
    Ok(())
}
