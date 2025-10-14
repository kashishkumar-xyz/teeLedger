# Overview of Tasks

## Security & Hardening Checklist
- Use **SQLCipher** for full-file encryption.
- Keep DB in the app’s private directory (Android: internal storage), not external storage.
- Strict file permissions (`0600` equivalent on Android private files).
- Use **Android Keystore** to hold keys or to wrap keys.
- Use **Argon2id** for deriving keys from passphrases; tune memory/time for mobile CPU limits.
- Zero secrets in memory after use (use `zeroize` crate).
- Use prepared statements and parameterized queries (`rusqlite` does this).
- Wrap multi-statement updates in transactions.
- No sensitive logging. Avoid `tracing` or `println!` of secrets.
- Avoid long-lived keys in memory; require unlocking per session if threat model demands.
- Make encrypted backups optional and always protected (GPG or SQLCipher + passphrase).
- Test on GrapheneOS and ensure no unintended file leaks.

## Build Notes & Tooling
- Install `cargo-ndk`.
- Compile Rust for Android ABIs using `cargo ndk -t armeabi-v7a -t arm64-v8a -o target/android build --release`.
- Include the produced `.so` libraries in your Android project under `app/src/main/jniLibs/<ABI>/`.
- Link to SQLCipher: Add SQLCipher `.so` to `jniLibs` and ensure `rusqlite` links against `libsqlcipher.so` instead of `libsqlite3.so`. This may require custom `build.rs` or `pkg-config` environment variables during cargo build.

## UX/Security Tradeoffs & Suggestions (Tasks)
- Implement passphrase prompting once per session (if human input is desired).
- Implement UI locking after a certain time, requiring re-entry of passphrase.
- Integrate Android Keystore for key management to reduce user friction.
- Implement encrypted backup/restore functionality, including warnings about safe storage.
- Write unit tests for DB migration.
- Write integration tests for opening DB with the wrong key.
