# teeLedger Specification Sheet

## 1. Introduction
This document outlines the detailed specifications for the teeLedger application, a secure, offline-first personal ledger designed to track financial transactions and balances. It serves as a guide for developers implementing the core Rust library and integrating it with mobile UIs.

## 2. Core Data Structures

### 2.1. Transaction Object
The fundamental unit of data in teeLedger is the `Transaction` object. It is an append-only record of a single financial event.

**Structure:**
```rust
pub struct Transaction {
    pub id: Option<i64>, // Unique identifier, typically auto-incremented by DB
    pub date: String,    // ISO 8601 format (YYYY-MM-DD)
    pub person: String,  // Name of the person involved in the transaction
    pub amount: i64,     // Amount in cents (e.g., 1000 for $10.00). Positive if person owes you, negative if you owe person.
    pub note: Option<String>, // Optional description of the transaction
}
```

**Constraints:**
*   `date`: Must be a valid date string in `YYYY-MM-DD` format.
*   `person`: Must be a non-empty string. Input should be sanitized to prevent injection or unexpected characters.
*   `amount`: Must be an integer. Represents monetary value in the smallest unit (e.g., cents) to avoid floating-point inaccuracies.
*   `note`: Optional, can be an empty string or `None`.

### 2.2. Balances Dictionary
A derived structure representing the aggregated balance for each person. This is computed on-the-fly from the `Transaction` objects.

**Structure:**
```rust
pub struct Balances {
    pub person: String,
    pub balance: i64, // Aggregated amount for the person. Positive if person owes you, negative if you owe person.
}
```

**Computation Logic:**
```rust
// Pseudocode for balance computation
fn compute_balances(transactions: Vec<Transaction>) -> HashMap<String, i64> {
    let mut balances: HashMap<String, i64> = HashMap::new();
    for t in transactions {
        *balances.entry(t.person).or_insert(0) += t.amount;
    }
    balances
}
```

## 3. Core API (Rust Library)

The Rust core library will expose a minimal set of C-compatible functions for interaction with the UI. All functions must handle errors gracefully and return appropriate status codes or error messages.

### 3.1. Database Initialization and Opening

#### `init_db(db_path: &str, encryption_key: &[u8]) -> Result<(), Error>`
*   **Description:** Initializes a new SQLite database at `db_path` with SQLCipher encryption using `encryption_key`. This function should be called only once for a new database. If the database already exists, it should attempt to open it.
*   **Parameters:**
    *   `db_path`: Absolute path to the SQLite database file.
    *   `encryption_key`: Raw bytes of the encryption key (e.g., 32 bytes for AES-256).
*   **Returns:** `Ok(())` on success, `Err(Error)` on failure (e.g., file system error, encryption error).

#### `open_db(db_path: &str, encryption_key: &[u8]) -> Result<(), Error>`
*   **Description:** Opens an existing encrypted SQLite database at `db_path` using `encryption_key`.
*   **Parameters:**
    *   `db_path`: Absolute path to the SQLite database file.
    *   `encryption_key`: Raw bytes of the encryption key.
*   **Returns:** `Ok(())` on success, `Err(Error)` on failure (e.g., file not found, incorrect key, corruption).

### 3.2. Transaction Management

#### `add_transaction(db_path: &str, encryption_key: &[u8], person: &str, amount: i64, date: &str, note: Option<&str>) -> Result<i64, Error>`
*   **Description:** Adds a new transaction to the ledger.
*   **Parameters:**
    *   `db_path`: Path to the database.
    *   `encryption_key`: Encryption key.
    *   `person`: Name of the person.
    *   `amount`: Transaction amount.
    *   `date`: Transaction date (YYYY-MM-DD).
    *   `note`: Optional transaction note.
*   **Returns:** `Ok(transaction_id)` on success, `Err(Error)` on failure.

#### `list_transactions(db_path: &str, encryption_key: &[u8], person: Option<&str>, since_date: Option<&str>, limit: Option<i64>) -> Result<Vec<Transaction>, Error>`
*   **Description:** Retrieves a list of transactions, with optional filtering.
*   **Parameters:**
    *   `db_path`: Path to the database.
    *   `encryption_key`: Encryption key.
    *   `person`: Optional filter by person.
    *   `since_date`: Optional filter for transactions after a specific date (YYYY-MM-DD).
    *   `limit`: Optional limit on the number of transactions returned.
*   **Returns:** `Ok(Vec<Transaction>)` on success, `Err(Error)` on failure.

### 3.3. Balance Computation

#### `get_balance(db_path: &str, encryption_key: &[u8], person: &str) -> Result<i64, Error>`
*   **Description:** Computes the current balance for a specific person.
*   **Parameters:**
    *   `db_path`: Path to the database.
    *   `encryption_key`: Encryption key.
    *   `person`: The person for whom to compute the balance.
*   **Returns:** `Ok(balance_amount)` on success, `Err(Error)` on failure.

#### `list_balances(db_path: &str, encryption_key: &[u8]) -> Result<Vec<Balances>, Error>`
*   **Description:** Computes and returns the balances for all persons in the ledger.
*   **Parameters:**
    *   `db_path`: Path to the database.
    *   `encryption_key`: Encryption key.
*   **Returns:** `Ok(Vec<Balances>)` on success, `Err(Error)` on failure.

### 3.4. Backup and Restore

#### `backup_db(db_path: &str, encryption_key: &[u8], backup_path: &str) -> Result<(), Error>`
*   **Description:** Creates an encrypted backup of the database file.
*   **Parameters:**
    *   `db_path`: Path to the source database.
    *   `encryption_key`: Encryption key.
    *   `backup_path`: Destination path for the backup file.
*   **Returns:** `Ok(())` on success, `Err(Error)` on failure.

#### `restore_db(backup_path: &str, encryption_key: &[u8], db_path: &str) -> Result<(), Error>`
*   **Description:** Restores a database from an encrypted backup file.
*   **Parameters:**
    *   `backup_path`: Path to the encrypted backup file.
    *   `encryption_key`: Encryption key for the backup file.
    *   `db_path`: Destination path for the restored database.
*   **Returns:** `Ok(())` on success, `Err(Error)` on failure.

## 4. Functional Requirements

*   **Transaction Recording:** Users must be able to record new transactions with a date, person, amount, and optional note.
*   **Balance Calculation:** The application must accurately calculate the balance for individual persons and provide an overview of all balances.
*   **Transaction Listing:** Users must be able to view a list of all transactions, with options to filter by person, date, and limit the number of results.
*   **Data Persistence:** All transaction data must be securely stored and retrieved from an encrypted SQLite database.
*   **Offline Operation:** The application must function entirely offline, without requiring network connectivity.
*   **Backup/Restore:** Users must be able to create encrypted backups of their ledger data and restore from them.

## 5. Non-Functional Requirements

### 5.1. Security
*   **Full-Disk Encryption:** Utilize SQLCipher for full-file AES-256 encryption of the SQLite database.
*   **Key Management:** Integrate with Android's hardware-backed Keystore for secure storage and retrieval of the DB encryption key or a Key-Encryption-Key (KEK).
*   **Passphrase Derivation:** If a passphrase is used, derive the DB key using Argon2id or scrypt with appropriate memory/time parameters for mobile devices.
*   **Input Validation & Sanitization:** All user inputs (e.g., `person`, `note`) must be validated and sanitized to prevent SQL injection and other vulnerabilities.
*   **Parameterized Queries:** All database interactions must use prepared statements and parameterized queries.
*   **Least Privilege:** The database file (`ledger.db`) must have strict file permissions (e.g., `chmod 600` equivalent on Android private files) to restrict access.
*   **Transactional Operations:** All multi-statement database writes must be wrapped in explicit transactions to ensure data integrity.
*   **Memory Zeroization:** Sensitive data (e.g., encryption keys, passphrases) must be zeroized from memory after use.
*   **No Sensitive Logging:** The application must not log any sensitive information (e.g., transaction details, keys) to disk or external services.

### 5.2. Performance
*   **Low Overhead:** The Rust core library should be optimized for minimal runtime overhead, memory footprint, and dependencies.
*   **Efficient Queries:** Database queries should be optimized for speed, especially for balance computations and transaction listings.

### 5.3. Deployment
*   **Android Platform:** The application will be deployed on Android (specifically GrapheneOS).
*   **Native Compilation:** The Rust core library will be compiled as a native shared library (`.so`) for Android ABIs (arm64-v8a, armeabi-v7a).
*   **Private Storage:** The SQLite database file must be stored in the app's private internal storage (`context.getFilesDir()`) to prevent access by other applications.

## 6. Error Handling
All API functions must return a `Result` type, allowing for explicit error handling. Custom error types should be defined to provide meaningful error messages for various failure scenarios (e.g., `DbError`, `InvalidInputError`, `KeyManagementError`).