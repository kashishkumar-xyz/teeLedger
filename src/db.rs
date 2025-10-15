use base64::{Engine as _, engine::general_purpose};
use rusqlite::{Connection, Result, params};
use zeroize::Zeroize;
use crate::models::Transaction;
use uuid::Uuid;
use serde_json;

pub fn open_encrypted_db(db_path: &str, encryption_key: &mut [u8]) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    let key_b64 = general_purpose::STANDARD.encode(&*encryption_key);
    conn.pragma_update(None, "key", &key_b64)?;
    encryption_key.zeroize();

    conn.pragma_update(None, "journal_mode", &"WAL")?;

    Ok(conn)
}

pub fn run_integrity_check(conn: &mut Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("PRAGMA integrity_check;")?;
    let messages: Vec<String> = stmt
        .query_map(params![], |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()?;
    Ok(messages)
}

pub fn create_transactions_history_table(conn: &mut Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions_history (
            tx_id TEXT NOT NULL,
            version INTEGER NOT NULL,
            data TEXT NOT NULL, -- JSON payload of the transaction
            op TEXT NOT NULL,   -- 'insert', 'update', 'delete'
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (tx_id, version)
        );",
        [],
    )?;
    Ok(())
}

pub fn add_transaction(conn: &mut Connection, transaction: &Transaction) -> Result<i64> {
    let tx_id = Uuid::new_v4().to_string();
    let version = 1;
    let data = serde_json::to_string(transaction)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    conn.execute(
        "INSERT INTO transactions_history (tx_id, version, data, op) VALUES (?1, ?2, ?3, ?4)",
        params![tx_id, version, data, "insert"],
    )?;

    // For now, we'll return a dummy ID. In a real scenario, this might be the rowid or a generated ID.
    Ok(1)
}

pub fn list_transactions(conn: &mut Connection, person: Option<&str>) -> Result<Vec<Transaction>> {
    let mut transactions = Vec::new();
    let mut stmt = if let Some(_p) = person {
        conn.prepare(
            "SELECT data FROM transactions_history WHERE JSON_EXTRACT(data, '$.person') = ?1 ORDER BY created_at ASC, version ASC",
        )?
    } else {
        conn.prepare("SELECT data FROM transactions_history ORDER BY created_at ASC, version ASC")?
    };

    let map_row = |row: &rusqlite::Row| {
        let data: String = row.get(0)?;
        serde_json::from_str(&data)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
    };

    let transaction_iter = if let Some(p) = person {
        stmt.query_map(params![p], map_row)?
    } else {
        stmt.query_map(params![], map_row)?
    };

    for transaction_result in transaction_iter {
        transactions.push(transaction_result?);
    }

    Ok(transactions)
}
