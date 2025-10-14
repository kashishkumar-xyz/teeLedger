# Draft Execution Plan

This execution plan outlines the steps to implement the `speck-teeLedger` project, focusing on the recommended architecture and Android integration.

## Phase 1: Core Rust Library Development
1.  **Initialize Rust Project**: Set up a new Rust library project (`cdylib`).
2.  **Define Core Data Structures**: Implement the `Transaction` struct and logic for managing a list of transactions.
3.  **Implement Ledger Logic**: Develop functions for adding transactions, computing balances (`get_balance`, `list_balances`), and listing transactions (`list_transactions`).
4.  **Integrate `rusqlite`**: Set up basic SQLite database interactions using `rusqlite` for CRUD operations.
5.  **FFI/JNI Interface**: Expose core Rust functions via a C ABI or JNI interface using the `jni` crate, ensuring safe memory ownership.
6.  **Unit Testing**: Write comprehensive unit tests for all core ledger logic and database interactions.

## Phase 2: Secure Storage and Key Management
1.  **SQLCipher Integration**: 
    -   Compile SQLCipher for Android targets.
    -   Configure `rusqlite` (via `build.rs` or `pkg-config`) to link against the SQLCipher library instead of standard SQLite.
    -   Implement `init_db` and `open_db` functions in Rust to handle SQLCipher database opening with a key.
2.  **Android Keystore Integration (UI Side)**:
    -   Develop Kotlin/Java code in the Android UI to interact with the Android Keystore.
    -   Implement logic to create, retrieve, or unwrap the DB encryption key using the Keystore.
    -   Pass the derived DB key bytes securely to the Rust core via JNI.
3.  **Alternative Key Derivation**: If Keystore is not used, implement Argon2id or scrypt for deriving DB keys from user-provided passphrases, ensuring proper memory zeroing after use.

## Phase 3: Android Application Development
1.  **Android Project Setup**: Create a new Android project (Kotlin/Java).
2.  **JNI Layer Development**: Create the JNI layer in Kotlin/Java to load the Rust shared library (`libledgercore.so`) and call the exposed Rust functions.
3.  **UI Development**: Design and implement the Android UI for:
    -   Adding new transactions.
    -   Displaying balances per person.
    -   Listing transaction history.
    -   Handling passphrase input (if applicable).
4.  **Permissions & Storage**: Configure the Android app to store the SQLite database in `context.getFilesDir()` with `MODE_PRIVATE` permissions.
5.  **Build System Integration**: 
    -   Install `cargo-ndk`.
    -   Configure the Android build system (Gradle) to build the Rust core for Android ABIs (`armeabi-v7a`, `arm64-v8a`, etc.) and bundle the resulting `.so` files into the APK under `app/src/main/jniLibs/<ABI>/`.

## Phase 4: Security Hardening and Testing
1.  **Input Validation and Sanitization**: Implement robust input validation and sanitization for all user inputs (e.g., `person`, `note`).
2.  **Transaction Management**: Ensure all multi-statement DB updates are wrapped in transactions.
3.  **Memory Management**: Implement `zeroize` crate usage in Rust to clear sensitive data from memory after use.
4.  **Logging Review**: Verify that no sensitive information is logged (avoid `tracing` or `println!` of secrets).
5.  **Backup/Restore Functionality**: Implement secure encrypted backup and restore features, with appropriate user warnings.
6.  **Comprehensive Testing**: 
    -   Develop integration tests for opening the DB with correct/incorrect keys.
    -   Conduct security audits and penetration testing, especially on GrapheneOS, to ensure no unintended file leaks or vulnerabilities.
    -   Test file permissions and access controls rigorously.

## Phase 5: Deployment and Optimization
1.  **APK Generation**: Generate the final APK for deployment.
2.  **Performance Tuning**: Optimize Rust code and database queries for mobile performance.
3.  **Documentation**: Create detailed documentation for the Rust core API, Android integration, and build process.

This plan provides a structured approach to developing the `speck-teeLedger` application, prioritizing security, performance, and maintainability.