# Quickstart: Core Rust Library Development

This guide provides a quick overview of how to set up, build, and test the core Rust library for teeLedger.

## Prerequisites

- Rust toolchain (version 1.75+ recommended)
- `cargo-ndk` (for Android cross-compilation)

## Building the Library

1.  **Clone the repository:**
    ```bash
    git clone [repository_url]
    cd teeLedger
    ```

2.  **Navigate to the Rust crate:**
    ```bash
    cd src/rust_core # Assuming 'rust_core' is the name of your Rust crate
    ```

3.  **Build for Android (example for arm64-v8a):**
    ```bash
    cargo ndk -t arm64-v8a build --release
    ```
    This will generate the shared library (`.so` file) in `target/aarch66-linux-android/release/`.

## Running Tests

1.  **Unit Tests:**
    ```bash
    cargo test
    ```

2.  **Integration Tests:**
    ```bash
    # If you have a separate integration test suite
    cargo test --workspace --test integration_tests
    ```

## FFI/JNI Integration

- The Rust library exposes functions via a C ABI, which can be called from Java/Kotlin using JNI.
- Refer to the `ffi.rs` file for the exposed function signatures.
- Example JNI calls will be provided in the Android application layer.