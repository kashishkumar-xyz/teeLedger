// src/db.rs
// This file will contain the SQLite database interaction logic.

use rusqlite::{Connection, Result, params};
use base64::{engine::general_purpose, Engine as _};

/// Opens an encrypted SQLite database connection.
///
/// This function attempts to open a database at the given path and apply the provided encryption key.
/// It also sets the journal mode to WAL (Write-Ahead Logging) for better concurrency and safer recovery.
///
/// # Arguments
/// * `db_path` - The file path to the SQLite database.
/// * `encryption_key` - The encryption key as a byte slice. **The caller is responsible for zeroizing the original key material after this function returns.**
///
/// # Returns
/// A `rusqlite::Result` containing the `Connection` object on success, or an error if the database
/// cannot be opened or the key cannot be applied.
pub fn open_encrypted_db(db_path: &str, encryption_key: &[u8]) -> Result<Connection> {
    let mut conn = Connection::open(db_path)?;

    // Apply the encryption key using PRAGMA key.
    // The key needs to be base64 encoded for the PRAGMA command.
    let key_b64 = general_purpose::STANDARD.encode(encryption_key);
    conn.pragma_update(None, "key", &key_b64)?;

    // Set journal mode to WAL for better performance and crash recovery.
    conn.pragma_update(None, "journal_mode", &"WAL")?;

    Ok(conn)
}

/// Runs an integrity check on the database.
///
/// This function executes `PRAGMA integrity_check;` on the provided database connection.
/// It returns a vector of strings, where each string is a line of output from the integrity check.
/// An empty vector indicates that the database is consistent.
///
/// # Arguments
/// * `conn` - A mutable reference to the `rusqlite::Connection`.
///
/// # Returns
/// A `rusqlite::Result` containing a `Vec<String>` with integrity check messages on success,
/// or an error if the PRAGMA command fails.
pub fn run_integrity_check(conn: &mut Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("PRAGMA integrity_check;")?;
    let messages: Vec<String> = stmt.query_map(params![], |row| row.get(0))?.collect::<Result<Vec<String>, _>>()?;
    Ok(messages)
}

/// Creates the `transactions_history` table if it does not already exist.
///
/// This table is designed to be append-only, storing a versioned history of all transactions.
/// It allows for fine-grained rollback and auditing by reconstructing the state at any given point.
///
/// # Arguments
/// * `conn` - A mutable reference to the `rusqlite::Connection`.
///
/// # Returns
/// A `rusqlite::Result` indicating success or failure of the table creation.
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

// Placeholder for other database interaction functions
// pub fn add_transaction(...) -> Result<()> { ... }
// pub fn list_transactions(...) -> Result<Vec<Transaction>> { ... }
// pub fn get_balance(...) -> Result<i64> { ... }
// pub fn list_balances(...) -> Result<Vec<Balance>> { ... }
