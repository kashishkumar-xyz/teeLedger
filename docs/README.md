# teeLedger

A secure, offline-first personal ledger application for tracking financial transactions and balances.

## Overview

teeLedger allows users to record transactions (who owes what, when, and why) and compute balances easily. Built with security in mind, it uses encrypted SQLite storage and runs entirely offline.

## Key Features

- **Transaction Recording:** Add transactions with date, person, amount, and notes.
- **Balance Calculation:** View individual and overall balances.
- **Secure Storage:** Full-disk encryption with SQLCipher.
- **Offline Operation:** No network required.
- **Backup/Restore:** Encrypted backups for data safety.

## Technology Stack

- **Core Library:** Rust for performance and memory safety.
- **Database:** SQLite with SQLCipher encryption.
- **Platform:** Android (GrapheneOS).
- **UI:** Kotlin/Java with JNI integration.

## Getting Started

### Prerequisites
- Rust toolchain
- Android SDK/NDK
- `cargo-ndk`

### Building the Rust Core
1. Clone the repository.
2. Navigate to the Rust crate directory.
3. Run `cargo ndk -t arm64-v8a build --release` to build for Android.

### Android Integration
1. Include the generated `.so` files in your Android project.
2. Implement JNI calls as per the Android Integration Guide.
3. Configure keystore for key management.

### Running Tests
- Run Rust tests: `cargo test`
- Android integration tests: Use Android Studio.

## Documentation

- [Specification Sheet](spec-sheet.md)
- [Security Guide](SECURITY.md)
- [Android Integration Guide](ANDROID_INTEGRATION.md)

## Contributing

Contributions are welcome. Please follow the security guidelines and ensure all changes are tested.

## License

[Specify license here]