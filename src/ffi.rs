use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::slice;
use std::ptr;

use crate::backup::perform_online_backup;
use crate::db::{open_encrypted_db, add_transaction, list_transactions};
use crate::models::Transaction;

#[unsafe(no_mangle)]
pub extern "C" fn backup_db_ffi(
    src_db_path_ptr: *const c_char,
    dest_db_path_ptr: *const c_char,
    encryption_key_ptr: *mut u8,
    key_len: c_int,
) -> c_int {
    unsafe {
        let src_db_path = CStr::from_ptr(src_db_path_ptr)
                .to_str()
                .expect("Invalid src_db_path_ptr")
        ;
        let dest_db_path = CStr::from_ptr(dest_db_path_ptr)
                .to_str()
                .expect("Invalid dest_db_path_ptr")
        ;

        let encryption_key = slice::from_raw_parts_mut(encryption_key_ptr, key_len as usize);

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
}

use crate::recovery::perform_online_restore;

#[unsafe(no_mangle)]
pub extern "C" fn restore_db_ffi(
    dest_db_path_ptr: *const c_char,
    src_db_path_ptr: *const c_char,
    encryption_key_ptr: *mut u8,
    key_len: c_int,
) -> c_int {
    unsafe {
        let dest_db_path = CStr::from_ptr(dest_db_path_ptr)
                .to_str()
                .expect("Invalid dest_db_path_ptr")
        ;
        let src_db_path = CStr::from_ptr(src_db_path_ptr)
                .to_str()
                .expect("Invalid src_db_path_ptr")
        ;

        let encryption_key = slice::from_raw_parts_mut(encryption_key_ptr, key_len as usize);

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
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_transaction_ffi(
    db_path_ptr: *const c_char,
    encryption_key_ptr: *mut u8,
    key_len: c_int,
    date_ptr: *const c_char,
    person_ptr: *const c_char,
    amount: i64,
    note_ptr: *const c_char,
) -> c_int {
    unsafe {
        let db_path = CStr::from_ptr(db_path_ptr)
            .to_str()
            .expect("Invalid db_path_ptr");
        let date = CStr::from_ptr(date_ptr)
            .to_str()
            .expect("Invalid date_ptr")
            .to_string();
        let person = CStr::from_ptr(person_ptr)
            .to_str()
            .expect("Invalid person_ptr")
            .to_string();
        let note = if note_ptr.is_null() {
            None
        } else {
            Some(
                CStr::from_ptr(note_ptr)
                    .to_str()
                    .expect("Invalid note_ptr")
                    .to_string(),
            )
        };

        let encryption_key = slice::from_raw_parts_mut(encryption_key_ptr, key_len as usize);

        let mut conn = match open_encrypted_db(db_path, encryption_key) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("FFI Error: Failed to open database: {}", e);
                return -1;
            }
        };

        let transaction = match Transaction::new(date, person, amount, note) {
            Ok(tx) => tx,
            Err(e) => {
                eprintln!("FFI Error: Invalid transaction data: {}", e);
                return -1;
            }
        };

        match add_transaction(&mut conn, &transaction) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("FFI Error: Failed to add transaction: {}", e);
                -1
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn list_transactions_ffi(
    db_path_ptr: *const c_char,
    encryption_key_ptr: *mut u8,
    key_len: c_int,
    person_ptr: *const c_char,
    count: *mut c_int,
) -> *mut *mut c_char {
    unsafe {
        *count = 0;
        let db_path = CStr::from_ptr(db_path_ptr)
            .to_str()
            .expect("Invalid db_path_ptr");
        let person = if person_ptr.is_null() {
            None
        } else {
            Some(
                CStr::from_ptr(person_ptr)
                    .to_str()
                    .expect("Invalid person_ptr"),
            )
        };

        let encryption_key = slice::from_raw_parts_mut(encryption_key_ptr, key_len as usize);

        let mut conn = match open_encrypted_db(db_path, encryption_key) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("FFI Error: Failed to open database: {}", e);
                return ptr::null_mut();
            }
        };

        let transactions = match list_transactions(&mut conn, person) {
            Ok(txs) => txs,
            Err(e) => {
                eprintln!("FFI Error: Failed to list transactions: {}", e);
                return ptr::null_mut();
            }
        };

        let mut c_strings: Vec<*mut c_char> = Vec::with_capacity(transactions.len());
        for tx in transactions {
            let json_string = serde_json::to_string(&tx).unwrap();
            c_strings.push(CString::new(json_string).unwrap().into_raw());
        }

        *count = c_strings.len() as c_int;
        let ptr = c_strings.as_mut_ptr();
        std::mem::forget(c_strings); // Prevent Rust from freeing the memory
        ptr
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn free_transactions_ffi(
    transactions_ptr: *mut *mut c_char,
    count: c_int,
) {
    unsafe {
        if transactions_ptr.is_null() {
            return;
        }
        let c_strings = Vec::from_raw_parts(transactions_ptr, count as usize, count as usize);
        for ptr in c_strings {
            if !ptr.is_null() {
                let _ = CString::from_raw(ptr);
            }
        }
    }
}
