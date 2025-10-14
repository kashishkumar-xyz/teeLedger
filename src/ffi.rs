// src/ffi.rs
// This file will expose core Rust functions via a C ABI or JNI interface.

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::slice;

use crate::db::open_encrypted_db;
use crate::backup::perform_online_backup;

/// Backs up the source database to a destination file.
///
/// # Safety
/// This function is unsafe because it operates on raw pointers from C.
/// The caller must ensure that `src_db_path_ptr`, `dest_db_path_ptr`, and `encryption_key_ptr`
/// are valid, null-terminated C strings and byte array respectively, and that `key_len`
/// accurately reflects the length of `encryption_key_ptr`.
///
/// # Arguments
/// * `src_db_path_ptr` - A C string pointer to the source database file path.
/// * `dest_db_path_ptr` - A C string pointer to the destination backup file path.
/// * `encryption_key_ptr` - A raw pointer to the encryption key bytes.
/// * `key_len` - The length of the encryption key bytes.
///
/// # Returns
/// `0` on success, or a non-zero error code on failure.
#[no_mangle]
pub unsafe extern "C" fn backup_db_ffi(
    src_db_path_ptr: *const c_char,
    dest_db_path_ptr: *const c_char,
    encryption_key_ptr: *const u8,
    key_len: c_int,
) -> c_int {
    // Safety: Convert C string pointers to Rust &str.
    // We must trust the caller to provide valid, null-terminated C strings.
    let src_db_path = unsafe {
        CStr::from_ptr(src_db_path_ptr).to_str().expect("Invalid src_db_path_ptr")
    };
    let dest_db_path = unsafe {
        CStr::from_ptr(dest_db_path_ptr).to_str().expect("Invalid dest_db_path_ptr")
    };

    // Safety: Convert raw key pointer and length to Rust byte slice.
    // We must trust the caller to provide a valid pointer and correct length.
    let encryption_key = unsafe {
        slice::from_raw_parts(encryption_key_ptr, key_len as usize)
    };

    // Open the source database connection.
    let src_conn_result = open_encrypted_db(src_db_path, encryption_key);
    let src_conn = match src_conn_result {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("FFI Error: Failed to open source database for backup: {}", e);
            return -1; // Indicate error
        }
    };

    // Perform the online backup.
    match perform_online_backup(&src_conn, dest_db_path, encryption_key) {
        Ok(_) => 0, // Success
        Err(e) => {
            eprintln!("FFI Error: Failed to perform online backup: {}", e);
            -2 // Indicate error
        }
    }
}

use crate::recovery::perform_online_restore;

/// Restores the destination database from a source backup file.
///
/// # Safety
/// This function is unsafe because it operates on raw pointers from C.
/// The caller must ensure that `dest_db_path_ptr`, `src_db_path_ptr`, and `encryption_key_ptr`
/// are valid, null-terminated C strings and byte array respectively, and that `key_len`
/// accurately reflects the length of `encryption_key_ptr`.
///
/// # Arguments
/// * `dest_db_path_ptr` - A C string pointer to the destination database file path.
/// * `src_db_path_ptr` - A C string pointer to the source backup file path.
/// * `encryption_key_ptr` - A raw pointer to the encryption key bytes.
/// * `key_len` - The length of the encryption key bytes.
///
/// # Returns
/// `0` on success, or a non-zero error code on failure.
pub unsafe extern "C" fn restore_db_ffi(
    dest_db_path_ptr: *const c_char,
    src_db_path_ptr: *const c_char,
    encryption_key_ptr: *const u8,
    key_len: c_int,
) -> c_int {
    // Safety: Convert C string pointers to Rust &str.
    let dest_db_path = unsafe {
        CStr::from_ptr(dest_db_path_ptr).to_str().expect("Invalid dest_db_path_ptr")
    };
    let src_db_path = unsafe {
        CStr::from_ptr(src_db_path_ptr).to_str().expect("Invalid src_db_path_ptr")
    };

    // Safety: Convert raw key pointer and length to Rust byte slice.
    let encryption_key = unsafe {
        slice::from_raw_parts(encryption_key_ptr, key_len as usize)
    };

    // Open the destination database connection.
    let dest_conn_result = open_encrypted_db(dest_db_path, encryption_key);
    let mut dest_conn = match dest_conn_result {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("FFI Error: Failed to open destination database for restore: {}", e);
            return -1; // Indicate error
        }
    };

    // Perform the online restore.
    match perform_online_restore(&mut dest_conn, src_db_path, encryption_key) {
        Ok(_) => 0, // Success
        Err(e) => {
            eprintln!("FFI Error: Failed to perform online restore: {}", e);
            -2 // Indicate error
        }
    }
}

