use teeLedger::models::{Balance, Transaction};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_transaction_new_valid() {
        let transaction = Transaction::new(
            "2025-01-15".to_string(),
            "Alice".to_string(),
            1000,
            Some("Lunch".to_string()),
        );
        assert!(transaction.is_ok());
        let tx = transaction.unwrap();
        assert_eq!(tx.date, "2025-01-15");
        assert_eq!(tx.person, "Alice");
        assert_eq!(tx.amount, 1000);
        assert_eq!(tx.note, Some("Lunch".to_string()));
        assert_eq!(tx.id, None);
    }

    #[test]
    fn test_transaction_new_empty_person() {
        let transaction = Transaction::new("2025-01-15".to_string(), "".to_string(), 500, None);
        assert!(transaction.is_err());
        assert_eq!(transaction.unwrap_err(), "Person name cannot be empty.");

        let transaction_whitespace =
            Transaction::new("2025-01-15".to_string(), "   ".to_string(), 500, None);
        assert!(transaction_whitespace.is_err());
        assert_eq!(
            transaction_whitespace.unwrap_err(),
            "Person name cannot be empty."
        );
    }

    #[test]
    fn test_transaction_new_invalid_date_format() {
        let transaction = Transaction::new("15-01-2025".to_string(), "Bob".to_string(), -200, None);
        assert!(transaction.is_err());
        assert_eq!(
            transaction.unwrap_err(),
            "Date must be in YYYY-MM-DD format."
        );

        let transaction_malformed_date =
            Transaction::new("2025/01/15".to_string(), "Bob".to_string(), -200, None);
        assert!(transaction_malformed_date.is_err());
        assert_eq!(
            transaction_malformed_date.unwrap_err(),
            "Date must be in YYYY-MM-DD format."
        );
    }

    #[test]
    fn test_transaction_new_with_defaults_no_date() {
        let transaction = Transaction::new_with_defaults(
            None,
            "Charlie".to_string(),
            -1500,
            Some("Dinner".to_string()),
        );
        assert!(transaction.is_ok());
        let tx = transaction.unwrap();

        let today = Local::now().format("%Y-%m-%d").to_string();
        assert_eq!(tx.date, today);
        assert_eq!(tx.person, "Charlie");
        assert_eq!(tx.amount, -1500);
        assert_eq!(tx.note, Some("Dinner".to_string()));
    }

    #[test]
    fn test_transaction_new_with_defaults_with_date() {
        let transaction = Transaction::new_with_defaults(
            Some("2024-12-25".to_string()),
            "David".to_string(),
            2500,
            None,
        );
        assert!(transaction.is_ok());
        let tx = transaction.unwrap();
        assert_eq!(tx.date, "2024-12-25");
        assert_eq!(tx.person, "David");
        assert_eq!(tx.amount, 2500);
        assert_eq!(tx.note, None);
    }

    #[test]
    fn test_balance_struct() {
        let balance = Balance {
            person: "Eve".to_string(),
            total_amount: 5000,
        };
        assert_eq!(balance.person, "Eve");
        assert_eq!(balance.total_amount, 5000);
    }
}
