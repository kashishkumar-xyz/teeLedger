// tests/unit/ffi_test.rs
// Unit tests for FFI/JNI interface in src/ffi.rs

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

use crate::ffi::add_transaction_ffi;
use crate::db::{open_encrypted_db, create_transactions_history_table};
use crate::models::Transaction;
use zeroize::Zeroize;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a CString from a Rust string
    fn to_cstring(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    // Helper to create a CString from an Option<String>
    fn to_optional_cstring(s: Option<String>) -> *const c_char {
        match s {
            Some(val) => CString::new(val).unwrap().into_raw(),
            None => ptr::null(),
        }
    }

    // Helper to free a CString created with into_raw
    unsafe fn free_cstring(ptr: *const c_char) {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr as *mut c_char);
        }
    }

    #[test]
    fn test_add_transaction_ffi_success() {
        let db_path = to_cstring("test_add_transaction_ffi.db");
        let mut encryption_key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut encryption_key);
        let key_len = encryption_key.len() as c_int;

        let date = to_cstring("2025-01-15");
        let person = to_cstring("Alice");
        let amount = 1000;
        let note = to_optional_cstring(Some("Lunch".to_string()));

        let result = unsafe {
            add_transaction_ffi(
                db_path.as_ptr(),
                encryption_key.as_mut_ptr(),
                key_len,
                date.as_ptr(),
                person.as_ptr(),
                amount,
                note,
            )
        };
        assert_eq!(result, 0);

        // Verify the transaction was added by opening the DB directly
        let mut opened_key = encryption_key.clone();
        let mut conn = open_encrypted_db(db_path.to_str().unwrap(), &mut opened_key).unwrap();
        create_transactions_history_table(&mut conn).unwrap();

        let mut stmt = conn.prepare("SELECT data FROM transactions_history").unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row = rows.next().unwrap().unwrap();
        let data: String = row.get(0).unwrap();
        let stored_transaction: Transaction = serde_json::from_str(&data).unwrap();

        assert_eq!(stored_transaction.date, "2025-01-15");
        assert_eq!(stored_transaction.person, "Alice");
        assert_eq!(stored_transaction.amount, 1000);
        assert_eq!(stored_transaction.note, Some("Lunch".to_string()));

        // Cleanup
        unsafe { free_cstring(note) };
        let _ = std::fs::remove_file(db_path.to_str().unwrap());
        encryption_key.zeroize();
        opened_key.zeroize();
    }

    #[test]
    fn test_add_transaction_ffi_invalid_date() {
        let db_path = to_cstring("test_add_transaction_ffi_invalid_date.db");
        let mut encryption_key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut encryption_key);
        let key_len = encryption_key.len() as c_int;

        let date = to_cstring("15-01-2025"); // Invalid date format
        let person = to_cstring("Bob");
        let amount = 500;
        let note = ptr::null();

        let result = unsafe {
            add_transaction_ffi(
                db_path.as_ptr(),
                encryption_key.as_mut_ptr(),
                key_len,
                date.as_ptr(),
                person.as_ptr(),
                amount,
                note,
            )
        };
        assert_eq!(result, -1);

        // Cleanup
        let _ = std::fs::remove_file(db_path.to_str().unwrap());
        encryption_key.zeroize();
    }

    #[test]
    fn test_add_transaction_ffi_empty_person() {
        let db_path = to_cstring("test_add_transaction_ffi_empty_person.db");
        let mut encryption_key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut encryption_key);
        let key_len = encryption_key.len() as c_int;

        let date = to_cstring("2025-01-16");
        let person = to_cstring(""); // Empty person
        let amount = 200;
        let note = ptr::null();

        let result = unsafe {
            add_transaction_ffi(
                db_path.as_ptr(),
                encryption_key.as_mut_ptr(),
                key_len,
                date.as_ptr(),
                person.as_ptr(),
                amount,
                note,
            )
        };
        assert_eq!(result, -1);

        // Cleanup
        let _ = std::fs::remove_file(db_path.to_str().unwrap());
        encryption_key.zeroize();
    }

    #[test]
    fn test_list_transactions_ffi_success() {
        let db_path = to_cstring("test_list_transactions_ffi_success.db");
        let mut encryption_key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut encryption_key);
        let key_len = encryption_key.len() as c_int;

        // Add some transactions first using the FFI add function
        let date1 = to_cstring("2025-01-15");
        let person1 = to_cstring("Alice");
        let note1 = to_optional_cstring(Some("Lunch".to_string()));
        unsafe {
            add_transaction_ffi(
                db_path.as_ptr(),
                encryption_key.as_mut_ptr(),
                key_len,
                date1.as_ptr(),
                person1.as_ptr(),
                1000,
                note1,
            );
            free_cstring(note1);
        }

        let date2 = to_cstring("2025-01-16");
        let person2 = to_cstring("Bob");
        let note2 = to_optional_cstring(Some("Dinner".to_string()));
        unsafe {
            add_transaction_ffi(
                db_path.as_ptr(),
                encryption_key.as_mut_ptr(),
                key_len,
                date2.as_ptr(),
                person2.as_ptr(),
                2000,
                note2,
            );
            free_cstring(note2);
        }

        let mut count: c_int = 0;
        let transactions_ptr = unsafe {
            list_transactions_ffi(
                db_path.as_ptr(),
                encryption_key.as_mut_ptr(),
                key_len,
                ptr::null(), // No person filter
                &mut count,
            )
        };

        assert!(transactions_ptr != ptr::null_mut());
        assert_eq!(count, 2);

        let transactions_slice = unsafe { slice::from_raw_parts(transactions_ptr, count as usize) };

        let stored_tx1: Transaction = serde_json::from_str(
            unsafe { CStr::from_ptr(transactions_slice[0]).to_str().unwrap() },
        )
        .unwrap();
        let stored_tx2: Transaction = serde_json::from_str(
            unsafe { CStr::from_ptr(transactions_slice[1]).to_str().unwrap() },
        )
        .unwrap();

        assert_eq!(stored_tx1.person, "Alice");
        assert_eq!(stored_tx2.person, "Bob");

        // Cleanup
        unsafe { free_transactions_ffi(transactions_ptr, count) };
        let _ = std::fs::remove_file(db_path.to_str().unwrap());
        encryption_key.zeroize();
    }

    #[test]
    fn test_list_transactions_ffi_by_person() {
        let db_path = to_cstring("test_list_transactions_ffi_by_person.db");
        let mut encryption_key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut encryption_key);
        let key_len = encryption_key.len() as c_int;

        // Add some transactions first using the FFI add function
        let date1 = to_cstring("2025-01-15");
        let person1 = to_cstring("Alice");
        let note1 = to_optional_cstring(Some("Lunch".to_string()));
        unsafe {
            add_transaction_ffi(
                db_path.as_ptr(),
                encryption_key.as_mut_ptr(),
                key_len,
                date1.as_ptr(),
                person1.as_ptr(),
                1000,
                note1,
            );
            free_cstring(note1);
        }

        let date2 = to_cstring("2025-01-16");
        let person2 = to_cstring("Bob");
        let note2 = to_optional_cstring(Some("Dinner".to_string()));
        unsafe {
            add_transaction_ffi(
                db_path.as_ptr(),
                encryption_key.as_mut_ptr(),
                key_len,
                date2.as_ptr(),
                person2.as_ptr(),
                2000,
                note2,
            );
            free_cstring(note2);
        }

        let mut count: c_int = 0;
        let person_filter = to_cstring("Alice");
        let transactions_ptr = unsafe {
            list_transactions_ffi(
                db_path.as_ptr(),
                encryption_key.as_mut_ptr(),
                key_len,
                person_filter.as_ptr(),
                &mut count,
            )
        };

        assert!(transactions_ptr != ptr::null_mut());
        assert_eq!(count, 1);

        let transactions_slice = unsafe { slice::from_raw_parts(transactions_ptr, count as usize) };
        let stored_tx: Transaction = serde_json::from_str(
            unsafe { CStr::from_ptr(transactions_slice[0]).to_str().unwrap() },
        )
        .unwrap();

        assert_eq!(stored_tx.person, "Alice");

        // Cleanup
        unsafe { free_transactions_ffi(transactions_ptr, count) };
        let _ = std::fs::remove_file(db_path.to_str().unwrap());
        encryption_key.zeroize();
    }
}

