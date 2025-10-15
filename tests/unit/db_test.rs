// tests/unit/db_test.rs
// Unit tests for database interaction logic in src/db.rs

use rusqlite::Connection;
use crate::db::{add_transaction, create_transactions_history_table};
use crate::models::Transaction;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_database() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        create_transactions_history_table(&conn).unwrap();
        conn
    }

    #[test]
    fn test_add_transaction_success() {
        let mut conn = setup_database();
        let transaction = Transaction::new(
            "2025-01-15".to_string(),
            "Alice".to_string(),
            1000,
            Some("Lunch".to_string()),
        ).unwrap();

        let result = add_transaction(&mut conn, &transaction);
        assert!(result.is_ok());

        let mut stmt = conn.prepare("SELECT tx_id, version, data, op FROM transactions_history").unwrap();
        let mut rows = stmt.query([]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let tx_id: String = row.get(0).unwrap();
        let version: i64 = row.get(1).unwrap();
        let data: String = row.get(2).unwrap();
        let op: String = row.get(3).unwrap();

        assert!(!tx_id.is_empty());
        assert_eq!(version, 1);
        assert_eq!(op, "insert");

        let stored_transaction: Transaction = serde_json::from_str(&data).unwrap();
        assert_eq!(stored_transaction.date, transaction.date);
        assert_eq!(stored_transaction.person, transaction.person);
        assert_eq!(stored_transaction.amount, transaction.amount);
        assert_eq!(stored_transaction.note, transaction.note);

        // Cleanup
        let _ = std::fs::remove_file(db_path.to_str().unwrap());
    }

    #[test]
    fn test_list_transactions_empty() {
        let mut conn = setup_database();
        let transactions = list_transactions(&mut conn, None).unwrap();
        assert!(transactions.is_empty());
    }

    #[test]
    fn test_list_transactions_multiple() {
        let mut conn = setup_database();
        let tx1 = Transaction::new("2025-01-15".to_string(), "Alice".to_string(), 1000, None).unwrap();
        let tx2 = Transaction::new("2025-01-16".to_string(), "Bob".to_string(), 2000, Some("Dinner".to_string())).unwrap();
        let tx3 = Transaction::new("2025-01-17".to_string(), "Alice".to_string(), 500, Some("Coffee".to_string())).unwrap();

        add_transaction(&mut conn, &tx1).unwrap();
        add_transaction(&mut conn, &tx2).unwrap();
        add_transaction(&mut conn, &tx3).unwrap();

        let transactions = list_transactions(&mut conn, None).unwrap();
        assert_eq!(transactions.len(), 3);
        assert_eq!(transactions[0].person, "Alice");
        assert_eq!(transactions[1].person, "Bob");
        assert_eq!(transactions[2].person, "Alice");
    }

    #[test]
    fn test_list_transactions_by_person() {
        let mut conn = setup_database();
        let tx1 = Transaction::new("2025-01-15".to_string(), "Alice".to_string(), 1000, None).unwrap();
        let tx2 = Transaction::new("2025-01-16".to_string(), "Bob".to_string(), 2000, Some("Dinner".to_string())).unwrap();
        let tx3 = Transaction::new("2025-01-17".to_string(), "Alice".to_string(), 500, Some("Coffee".to_string())).unwrap();

        add_transaction(&mut conn, &tx1).unwrap();
        add_transaction(&mut conn, &tx2).unwrap();
        add_transaction(&mut conn, &tx3).unwrap();

        let alice_transactions = list_transactions(&mut conn, Some("Alice")).unwrap();
        assert_eq!(alice_transactions.len(), 2);
        assert_eq!(alice_transactions[0].person, "Alice");
        assert_eq!(alice_transactions[1].person, "Alice");

        let bob_transactions = list_transactions(&mut conn, Some("Bob")).unwrap();
        assert_eq!(bob_transactions.len(), 1);
        assert_eq!(bob_transactions[0].person, "Bob");

        let charlie_transactions = list_transactions(&mut conn, Some("Charlie")).unwrap();
        assert!(charlie_transactions.is_empty());
    }
}
