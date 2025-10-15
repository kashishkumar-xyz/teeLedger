use base64::{Engine as _, engine::general_purpose};
use rusqlite::{Connection, Result, params};

pub fn open_encrypted_db(db_path: &str, encryption_key: &[u8]) -> Result<Connection> {
    let mut conn = Connection::open(db_path)?;

    let key_b64 = general_purpose::STANDARD.encode(encryption_key);
    conn.pragma_update(None, "key", &key_b64)?;

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
