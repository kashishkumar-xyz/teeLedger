# Project Plan

## Big-Picture Architecture (Recommended)

1.  **Core library (Rust)**
    -   Implements ledger logic, DB access and migrations, crypto helpers.
    -   Compiled to a native library (`cdylib` / `.so`) for Android (via `cargo-ndk` / `cargo build` for Android targets).
    -   Exposes a minimal, stable FFI (C/JNI) surface for the UI to call (or use `jni` crate directly from Rust).

2.  **Encrypted storage engine**
    -   **Preferred:** Use SQLite compiled with **SQLCipher** (full-file AES-256 encryption). Rust core uses `rusqlite` or bindings that link to that SQLCipher-built library.
    -   **Alternative:** If SQLCipher is onerous to build for Android, encrypt sensitive fields at the application level with authenticated encryption (AES-GCM or XChaCha20-Poly1305). (This leaves DB structure visible; less ideal.)

3.  **Key management**
    -   **Best:** Use Android’s hardware-backed keystore (GrapheneOS supports it) to store the DB encryption key or a key-encryption-key (KEK). The UI (Kotlin/Java) accesses the keystore and passes the key (or unwraps a key-encryption token) to the Rust library at runtime.
    -   **Other:** Prompt user for passphrase at app start (zero it after use) and derive DB key with Argon2id or scrypt.

4.  **Frontend/UI**
    -   Any UI (Kotlin, Flutter, React Native) calls the Rust core via JNI/FFI. Keep UI and storage logic separate.
    -   Prefer minimal UI surface that never logs or caches secrets.

5.  **Deployment**
    -   Build the Rust lib for Android targets (arm64-v8a etc.), produce `.so` and bundle into the APK.
    -   Ensure file permissions and storage location make DB inaccessible to other apps/users (private app storage).

## Android Integration Approach (Recommended)

1.  **Rust core**: Build as `cdylib` which exposes C ABI functions and/or JNI functions (using the `jni` crate).
2.  **Kotlin/Android UI**:
    -   Use the Android Keystore to create or retrieve a key (hardware-backed).
    -   Use that key to unwrap or generate the DB key (or prompt user passphrase and derive).
    -   Pass the DB key bytes into Rust `open_db()` via JNI.
    -   Call Rust functions for add/list/get.
3.  **Permissions & Storage**:
    -   Store DB in `context.getFilesDir()` (private app storage).
    -   `MODE_PRIVATE` ensures other apps cannot access files.
4.  **Build tool**:
    -   Use `cargo-ndk` or `cargo-apk` to build Android ABIs and create `.so` files.
    -   Bundle `.so` in the APK under `lib/<ABI>/libledgercore.so`.
    -   JNI layer (Kotlin) loads `System.loadLibrary("ledgercore")` and calls methods.

*Note: On GrapheneOS, follow its recommended best practices for keystore usage.*