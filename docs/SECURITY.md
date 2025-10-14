# Security Guide for teeLedger

## Threat Model

The teeLedger application is designed to protect sensitive financial data from various threats, particularly in the context of offline, mobile usage on GrapheneOS. Key threats include:

- **Physical Theft:** An attacker gains physical access to the device and attempts to extract data from the SQLite database file.
- **Data Exposure:** Accidental or malicious exposure of the database file, either through file sharing, backups, or malware.
- **Malware Snooping:** Malicious software running on the device attempts to access memory, logs, or the database.
- **Network Attacks:** Although the app is offline-first, any network interactions (e.g., for backups) must be secure.
- **Insider Threats:** Unauthorized access by other apps or users on the same device.

The primary goal is to ensure that even if the database file is copied, it remains unreadable without the proper encryption key.

## Security Best Practices

### Database Encryption
- Use SQLCipher for full-file AES-256 encryption of the SQLite database. This encrypts the entire file, including schema, indices, and data.
- Never store the encryption key in plaintext on disk or in the application binary.
- Use a strong, randomly generated key (e.g., 32 bytes) for AES-256.

### Key Management
- **Preferred Method:** Leverage Android's hardware-backed Keystore to store the DB encryption key or a Key-Encryption-Key (KEK). The UI retrieves the key at runtime and passes it to the Rust core.
- **Alternative Method:** Derive the key from a user-provided passphrase using Argon2id or scrypt. Parameters should be tuned for mobile devices (e.g., memory: 64MB, iterations: 4, parallelism: 1 for Argon2id).
- Zero out keys and passphrases from memory immediately after use using the `zeroize` crate.
- Require key entry per session or use keystore for seamless access, depending on the security vs. usability tradeoff.

### Input Validation and Sanitization
- Validate all inputs: Ensure `person` is a non-empty string without special characters that could lead to injection.
- Sanitize `note` fields to remove or escape potentially harmful characters.
- Use parameterized queries exclusively to prevent SQL injection.

### Access Control
- Store the database in the app's private internal storage (`context.getFilesDir()`) with `MODE_PRIVATE` to restrict access to other apps.
- Set file permissions equivalent to `chmod 600` on Android private files.
- Avoid external storage to prevent exposure.

### Transactional Integrity
- Wrap all multi-statement database operations in explicit transactions to maintain data consistency.
- Use prepared statements for all queries.

### Logging and Debugging
- Do not log sensitive information such as transaction details, amounts, or encryption keys.
- Avoid using `println!`, `tracing`, or similar for secrets.
- In debug builds, ensure no secrets are exposed in logs.

### Backup and Restore
- Backups should be encrypted (the SQLCipher file is already encrypted).
- Warn users about secure storage of backups.
- Implement optional GPG encryption for additional protection if desired.

## Hardening Checklist

- [ ] Integrate SQLCipher for full-file encryption.
- [ ] Implement Android Keystore integration for key storage.
- [ ] Use Argon2id for passphrase-derived keys with mobile-optimized parameters.
- [ ] Validate and sanitize all user inputs.
- [ ] Use parameterized queries and prepared statements.
- [ ] Store DB in private app storage with strict permissions.
- [ ] Wrap DB operations in transactions.
- [ ] Zeroize sensitive data in memory.
- [ ] Avoid logging secrets.
- [ ] Test encryption with incorrect keys to ensure proper failure.
- [ ] Audit code for potential memory leaks or side-channel attacks.
- [ ] Perform security testing on GrapheneOS to verify no unintended file access.

## Implementation Notes

- The Rust core library should handle encryption setup and key passing securely.
- JNI interactions must ensure keys are not leaked in Java/Kotlin memory.
- Regular security audits and dependency updates are recommended.
- For production, consider code reviews focused on cryptography and secure coding practices.