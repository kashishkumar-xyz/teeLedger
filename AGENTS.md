# teeLedger Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-10-15

## Active Technologies
- Rust 1.75+ + rusqlite, jni, zeroize, base64, hmac (for FFI/JNI interface, secure key management, backup checksums) (001-phase-1-core)

## Project Structure
```
src/
├── lib.rs
├── models.rs
├── db.rs
├── ffi.rs
├── key_management.rs
├── backup.rs
└── recovery.rs

data/
└── backups/

tests/
├── unit/
│   ├── models_test.rs
│   ├── db_test.rs
│   ├── ffi_test.rs
│   ├── key_management_test.rs
│   ├── backup_test.rs
│   └── recovery_test.rs
└── integration/
    └── ledger_integration_test.rs
```

## Commands
cargo test && cargo clippy && cargo run --bin teeLedger-cli -- --help

## Code Style
Rust 1.75+: Follow standard conventions

## Recent Changes
## Recent Changes
- 001-phase-1-core: Added Rust 1.75+ + rusqlite, jni, zeroize, base64, hmac; Implemented layered backup and recovery strategy with Android Keystore integration.

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->