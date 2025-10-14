# Android Integration Guide for teeLedger

This guide provides detailed instructions for integrating the Rust core library with an Android application, focusing on compilation, JNI interaction, key management, and deployment.

## Rust Core Compilation

The Rust core library must be compiled as a `cdylib` to produce shared libraries (`.so` files) for Android ABIs.

### Prerequisites
- Install `cargo-ndk`: `cargo install cargo-ndk`
- Ensure Android NDK is installed and configured in your environment.

### Compilation Steps
1. Add the following to your `Cargo.toml`:
   ```toml
   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   rusqlite = { version = "0.28", features = ["bundled"] }  # Or link to SQLCipher
   # Other dependencies...
   ```

2. For SQLCipher integration, modify `build.rs` to link against SQLCipher:
   ```rust
   fn main() {
       println!("cargo:rustc-link-lib=dylib=sqlcipher");
       // Additional linking if needed
   }
   ```
   Ensure SQLCipher `.so` is available in the Android project.

3. Compile for Android ABIs:
   ```
   cargo ndk -t armeabi-v7a -t arm64-v8a -o target/android build --release
   ```
   This produces `.so` files in `target/android/<ABI>/libledgercore.so`.

4. Copy the `.so` files to your Android project under `app/src/main/jniLibs/<ABI>/`.

## JNI/FFI Interaction

The Rust library exposes C-compatible functions that the Kotlin UI calls via JNI.

### Exposed Functions
- `init_db`, `open_db`, `add_transaction`, etc., as defined in the spec sheet.
- Functions should use `#[no_mangle]` and `extern "C"` for C ABI.

### Kotlin JNI Calls
1. Load the library: `System.loadLibrary("ledgercore")`
2. Declare native methods in Kotlin:
   ```kotlin
   external fun initDb(dbPath: String, key: ByteArray): Int  // Return 0 on success
   // Similar for other functions
   ```
3. Call functions, handling errors appropriately.

### Memory Management
- Use safe wrappers for strings and buffers to avoid memory leaks.
- Ensure ownership is properly managed across the FFI boundary.

## Key Management Integration

### Using Android Keystore
1. In Kotlin, create or retrieve a key from the Keystore:
   ```kotlin
   val keyStore = KeyStore.getInstance("AndroidKeyStore")
   keyStore.load(null)
   val keyGenerator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, "AndroidKeyStore")
   keyGenerator.init(KeyGenParameterSpec.Builder("ledger_key", KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT)
       .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
       .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
       .build())
   val secretKey = keyGenerator.generateKey()
   ```
2. Use the key to encrypt/decrypt the DB key or pass it directly to Rust.

### Passphrase Handling
- If using passphrase, derive key in Kotlin using a library like `Argon2` or `SCrypt`.
- Pass the derived key bytes to Rust.

## Storage and Permissions

- Store the DB in `context.getFilesDir()` (private storage).
- Use `MODE_PRIVATE` for file creation.
- No additional permissions needed beyond standard app permissions.

## Build System Configuration

- Use Gradle to include the `.so` files.
- Ensure `cargo-ndk` builds are integrated into your CI/CD pipeline.
- For SQLCipher, include the SQLCipher `.so` in `jniLibs` and configure linking.

## Testing and Debugging

- Test on GrapheneOS for keystore compatibility.
- Use Android Studio's native debugging for Rust code.
- Verify encryption by attempting to open DB with wrong keys.

## Deployment Notes

- Bundle the `.so` files in the APK.
- Ensure ABI compatibility (arm64-v8a, armeabi-v7a).
- Follow GrapheneOS best practices for keystore and security.