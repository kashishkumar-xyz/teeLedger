// tests/integration/cli_test.rs
// Integration tests for the CLI application.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use rusqlite::Connection;
use teeLedger::db::{open_encrypted_db, create_transactions_history_table};
use teeLedger::models::Transaction;
use teeLedger::key_management::generate_dek;
use zeroize::Zeroize;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_command_success() {
        let db_file = "test_add_command_success.db";
        let mut cmd = Command::cargo_bin("teeLedger-cli").unwrap();

        // Generate a DEK for the CLI to use
        let mut dek = generate_dek();
        let key_len = dek.len();
        // In a real scenario, the DEK would be managed by Android Keystore.
        // For CLI integration test, we'll just use a dummy key and ensure it's zeroized.

        // Simulate the CLI call to add a transaction
        cmd.arg("--database")
            .arg(db_file)
            .arg("add")
            .arg("--person")
            .arg("Alice")
            .arg("--amount")
            .arg("1000")
            .arg("--date")
            .arg("2025-01-15")
            .arg("--note")
            .arg("Lunch");

        cmd.assert().success().stdout(predicate::str::contains("Transaction added successfully."));

        // Verify the transaction was added by opening the DB directly
        let mut opened_key = dek.clone();
        let mut conn = open_encrypted_db(db_file, &mut opened_key).unwrap();
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
        fs::remove_file(db_file).unwrap();
        dek.zeroize();
        opened_key.zeroize();
    }

    #[test]
    fn test_list_command_empty() {
        let db_file = "test_list_command_empty.db";
        let mut cmd = Command::cargo_bin("teeLedger-cli").unwrap();

        cmd.arg("--database")
            .arg(db_file)
            .arg("list");

        cmd.assert().success().stdout(predicate::str::contains("No transactions found."));

        // Cleanup
        let _ = fs::remove_file(db_file);
    }

    #[test]
    fn test_list_command_multiple_transactions() {
        let db_file = "test_list_command_multiple_transactions.db";
        let mut cmd = Command::cargo_bin("teeLedger-cli").unwrap();

        // Add some transactions first
        Command::cargo_bin("teeLedger-cli")
            .unwrap()
            .arg("--database")
            .arg(db_file)
            .arg("add")
            .arg("--person")
            .arg("Alice")
            .arg("--amount")
            .arg("1000")
            .arg("--date")
            .arg("2025-01-15")
            .assert()
            .success();

        Command::cargo_bin("teeLedger-cli")
            .unwrap()
            .arg("--database")
            .arg(db_file)
            .arg("add")
            .arg("--person")
            .arg("Bob")
            .arg("--amount")
            .arg("2000")
            .arg("--date")
            .arg("2025-01-16")
            .assert()
            .success();

        cmd.arg("--database")
            .arg(db_file)
            .arg("list");

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Alice").and(predicate::str::contains("Bob")));

        // Cleanup
        fs::remove_file(db_file).unwrap();
    }

    #[test]
    fn test_list_command_by_person() {
        let db_file = "test_list_command_by_person.db";
        let mut cmd = Command::cargo_bin("teeLedger-cli").unwrap();

        // Add some transactions first
        Command::cargo_bin("teeLedger-cli")
            .unwrap()
            .arg("--database")
            .arg(db_file)
            .arg("add")
            .arg("--person")
            .arg("Alice")
            .arg("--amount")
            .arg("1000")
            .arg("--date")
            .arg("2025-01-15")
            .assert()
            .success();

        Command::cargo_bin("teeLedger-cli")
            .unwrap()
            .arg("--database")
            .arg(db_file)
            .arg("add")
            .arg("--person")
            .arg("Bob")
            .arg("--amount")
            .arg("2000")
            .arg("--date")
            .arg("2025-01-16")
            .assert()
            .success();

        cmd.arg("--database")
            .arg(db_file)
            .arg("list")
            .arg("--person")
            .arg("Alice");

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Alice").and(predicate::str::not(predicate::str::contains("Bob"))));

        // Cleanup
        fs::remove_file(db_file).unwrap();
    }
}
