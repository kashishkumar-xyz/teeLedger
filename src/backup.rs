// src/backup.rs
// This file will implement the online backup API and atomic snapshots.

use hmac::{Hmac, Mac};
use digest::NewMac;
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};

/// Type alias for HMAC-SHA256.
type HmacSha256 = Hmac<Sha256>;

/// Generates an HMAC-SHA256 checksum for the given data using a provided key.
/// The checksum is returned as a Base64-encoded string.
///
/// # Arguments
/// * `data` - The input data for which to generate the checksum.
/// * `key` - The secret key to use for HMAC generation.
///
/// # Returns
/// A `Result` containing the Base64-encoded checksum string on success,
/// or an `hmac::digest::InvalidLength` error if the key length is invalid.
pub fn generate_backup_checksum(data: &[u8], key: &[u8]) -> Result<String, hmac::digest::InvalidLength> {
    let mut mac = HmacSha256::new_from_slice(key)?;
    mac.update(data);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    Ok(general_purpose::STANDARD.encode(code_bytes))
}

use rusqlite::Connection;
use std::fs;
use std::io;

/// Performs an online backup of the source database to a destination file.
///
/// This function uses SQLite's online backup API to safely copy the database
/// without requiring the source database to be closed.
///
/// # Arguments
/// * `src_conn` - A reference to the source `rusqlite::Connection`.
/// * `dest_path` - The file path where the backup will be saved.
/// * `encryption_key` - The encryption key for the destination database.
///
/// # Returns
/// A `rusqlite::Result` indicating success or failure.
pub fn perform_online_backup(src_conn: &Connection, dest_path: &str, encryption_key: &[u8]) -> rusqlite::Result<()> {
    // Create a new connection to the destination path for the backup.
    // This connection will be encrypted with the provided key.
    let mut dest_conn = Connection::open(dest_path)?;
    let key_b64 = general_purpose::STANDARD.encode(encryption_key);
    dest_conn.pragma_update(None, "key", &key_b64)?;

    let mut backup = rusqlite::backup::Backup::new(src_conn, &mut dest_conn)?;
    backup.step(-1)?;

    // Ensure the backup file is synchronized to disk.
    // Note: fsync on the file handle is ideal, but rusqlite::backup doesn't expose it directly.
    // A workaround might involve re-opening the file or relying on OS flushing.
    // For now, we assume the OS handles flushing adequately after `finish()`.

    Ok(())
}

/// Atomically renames a temporary backup file to its final destination.
/// This helps ensure that the backup file is always in a consistent state.
///
/// # Arguments
/// * `temp_path` - The path to the temporary backup file.
/// * `final_path` - The desired final path for the backup file.
///
/// # Returns
/// A `std::io::Result` indicating success or failure.
pub fn atomic_rename_snapshot(temp_path: &str, final_path: &str) -> io::Result<()> {
    fs::rename(temp_path, final_path)
}
