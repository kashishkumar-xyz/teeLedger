use base64::{Engine as _, engine::general_purpose};
use rusqlite::Connection;

pub fn perform_online_restore(
    dest_conn: &mut Connection,
    src_path: &str,
    encryption_key: &[u8],
) -> rusqlite::Result<()> {
    let src_conn = Connection::open(src_path)?;
    let key_b64 = general_purpose::STANDARD.encode(encryption_key);
    src_conn.pragma_update(None, "key", &key_b64)?;

    let backup = rusqlite::backup::Backup::new(&src_conn, dest_conn)?;
    backup.step(-1)?;

    Ok(())
}
