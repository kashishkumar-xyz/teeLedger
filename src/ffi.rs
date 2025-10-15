use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::slice;

use crate::backup::perform_online_backup;
use crate::db::open_encrypted_db;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn backup_db_ffi(
    src_db_path_ptr: *const c_char,
    dest_db_path_ptr: *const c_char,
    encryption_key_ptr: *const u8,
    key_len: c_int,
) -> c_int {
    let src_db_path = unsafe {
        CStr::from_ptr(src_db_path_ptr)
            .to_str()
            .expect("Invalid src_db_path_ptr")
    };
    let dest_db_path = unsafe {
        CStr::from_ptr(dest_db_path_ptr)
            .to_str()
            .expect("Invalid dest_db_path_ptr")
    };

    let encryption_key = unsafe { slice::from_raw_parts(encryption_key_ptr, key_len as usize) };

    let src_conn_result = open_encrypted_db(src_db_path, encryption_key);
    let src_conn = match src_conn_result {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!(
                "FFI Error: Failed to open source database for backup: {}",
                e
            );
            return -1;
        }
    };

    match perform_online_backup(&src_conn, dest_db_path, encryption_key) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("FFI Error: Failed to perform online backup: {}", e);
            -2
        }
    }
}

use crate::recovery::perform_online_restore;

pub unsafe extern "C" fn restore_db_ffi(
    dest_db_path_ptr: *const c_char,
    src_db_path_ptr: *const c_char,
    encryption_key_ptr: *const u8,
    key_len: c_int,
) -> c_int {
    let dest_db_path = unsafe {
        CStr::from_ptr(dest_db_path_ptr)
            .to_str()
            .expect("Invalid dest_db_path_ptr")
    };
    let src_db_path = unsafe {
        CStr::from_ptr(src_db_path_ptr)
            .to_str()
            .expect("Invalid src_db_path_ptr")
    };

    let encryption_key = unsafe { slice::from_raw_parts(encryption_key_ptr, key_len as usize) };

    let dest_conn_result = open_encrypted_db(dest_db_path, encryption_key);
    let mut dest_conn = match dest_conn_result {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!(
                "FFI Error: Failed to open destination database for restore: {}",
                e
            );
            return -1;
        }
    };

    match perform_online_restore(&mut dest_conn, src_db_path, encryption_key) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("FFI Error: Failed to perform online restore: {}", e);
            -2
        }
    }
}
