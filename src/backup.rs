use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn generate_backup_checksum(
    data: &[u8],
    key: &[u8],
) -> Result<String, hmac::digest::InvalidLength> {
    let mut mac = HmacSha256::new_from_slice(key)?;
    mac.update(data);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    Ok(general_purpose::STANDARD.encode(code_bytes))
}

use rusqlite::Connection;
use std::fs;
use std::io;

pub fn perform_online_backup(
    src_conn: &Connection,
    dest_path: &str,
    encryption_key: &[u8],
) -> rusqlite::Result<()> {
    let mut dest_conn = Connection::open(dest_path)?;
    let key_b64 = general_purpose::STANDARD.encode(encryption_key);
    dest_conn.pragma_update(None, "key", &key_b64)?;

    let backup = rusqlite::backup::Backup::new(src_conn, &mut dest_conn)?;
    backup.step(-1)?;

    Ok(())
}

pub fn atomic_rename_snapshot(temp_path: &str, final_path: &str) -> io::Result<()> {
    fs::rename(temp_path, final_path)
}
