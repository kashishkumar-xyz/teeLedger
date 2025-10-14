## 🧠 The goal

You want a **single, consistent ledger** where every entry records **a transaction that changes what someone owes you** or what you owe them.

We want a **simple ledger** that:

- Records **transactions** (with who, when, and how much)
- Can easily compute **balances per person**
- Doesn’t require heavy accounting models

So we’re after a data structure that’s:

- **Append-only** (each new entry adds to history)
- **Queryable by person** (to compute running balances)
- **Human-readable**

## 💡 Core idea

Think of all transactions in terms of **your perspective**:

- **Positive (+)** → someone owes you (you should receive money)
- **Negative (−)** → you owe someone (you should pay money)

## 🧱 Core data structure: a *list of transaction objects*

At the heart, you need this:

```
[
    { "date": "2025-10-12", "person": "Adam", "amount": 10, "note": "Tea sale" },
    { "date": "2025-10-12", "person": "Ron",  "amount": -30, "note": "Bought milk" },
    { "date": "2025-10-13", "person": "Adam", "amount": -10, "note": "Payment" },
    { "date": "2025-10-14", "person": "Ron",  "amount": +20, "note": "Credit sale" },
    { "date": "2025-10-15", "person": "Ron",  "amount": -10, "note": "Paid" },
    { "date": "2025-10-16", "person": "Adam", "amount": -20, "note": "Paid" }
]
```

That’s it — this one structure can support *all operations*.

---

## 🧮 Derived structure: balances dictionary

You can compute balances by aggregating:

```
balances = {}
for t in transactions:
    balances[t["person"]] = balances.get(t["person"], 0) + t["amount"]
```

Result:

```
{'Adam': -20, 'Ron': -20}
```

Interpretation:
A **negative** balance = you owe them.
A **positive** balance = they owe you.

---



## 🧭 Key goals

| Requirement          | Meaning                                                                    |
| -------------------- | -------------------------------------------------------------------------- |
| **Least overhead**   | Minimal runtime, dependencies, setup, and footprint                        |
| **Maximum security** | Strong prevention of SQL injection, safe parameter handling, memory safety |
| **CRUD efficiency**  | Clean code for Create/Read/Update/Delete                                   |
| **SQLite support**   | Mature, stable bindings to SQLite                                          |

---

| Language                 | Overhead                                                | Security                                                                                            | Notes                                                                       |
| ------------------------ | ------------------------------------------------------- | --------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| **Rust** 🦀              | 🔹 Low runtime overhead<br>🔹 Compiles to native binary | 🔹 Memory-safe<br>🔹 Strong type safety<br>🔹 Prepared statements enforced                          | Best for security & performance. Slightly higher dev effort.                |

---


## 🔐 Security best practices (regardless of language)

1. **Always use parameterized queries**, never string concatenation.
2. **Validate and sanitize input** (e.g., strip weird characters in `person` or `note`).
3. **Use least privileges**: file permissions on `ledger.db` (`chmod 600`).
4. **Wrap all DB operations in transactions** when doing multiple writes.
5. **Back up regularly** (SQLite file = single point of truth).

---

## 🧭 Context recap

| Aspect   | Description                                               |
| -------- | --------------------------------------------------------- |
| Purpose  | Local ledger app — tracks owes/dues, offline              |
| Platform | Android (GrapheneOS), compiled as APK                     |
| Data     | SQLite (likely encrypted, e.g. via SQLCipher)             |
| Threats  | Physical theft, data exposure, malware snooping           |
| Goals    | Minimal overhead, strong security, robust future-proofing |

- Full offline operation (no network),
- Strong on-disk protection (an attacker who copies the `.db` file cannot read it),
- Minimal attack surface in memory and on-disk,
- A Rust core library you can call from a mobile UI.

---
# Big-picture architecture (recommended)

1. **Core library (Rust)**

   - Implements ledger logic, DB access and migrations, crypto helpers.
   - Compiled to a native library (`cdylib` / `.so`) for Android (via `cargo-ndk` / `cargo build` for Android targets).
   - Exposes a minimal, stable FFI (C/JNI) surface for the UI to call (or use `jni` crate directly from Rust).

2. **Encrypted storage engine**

   - **Preferred:** Use SQLite compiled with **SQLCipher** (full-file AES-256 encryption). Rust core uses rusqlite or bindings that link to that SQLCipher-built library.
   - **Alternative:** If SQLCipher is onerous to build for Android, encrypt sensitive fields at the application level with authenticated encryption (AES-GCM or XChaCha20-Poly1305). (This leaves DB structure visible; less ideal.)

3. **Key management**

   - **Best:** Use Android’s hardware-backed keystore (GrapheneOS supports it) to store the DB encryption key or a key-encryption-key (KEK). The UI (Kotlin/Java) accesses the keystore and passes the key (or unwraps a key-encryption token) to the Rust library at runtime.
   - **Other:** Prompt user for passphrase at app start (zero it after use) and derive DB key with Argon2id or scrypt.

4. **Frontend/UI**

   - Any UI (Kotlin, Flutter, React Native) calls the Rust core via JNI/FFI. Keep UI and storage logic separate.
   - Prefer minimal UI surface that never logs or caches secrets.

5. **Deployment**

   - Build the Rust lib for Android targets (arm64-v8a etc.), produce `.so` and bundle into the APK.
   - Ensure file permissions and storage location make DB inaccessible to other apps/users (private app storage).

---
- **SQLCipher (recommended):** encrypts entire SQLite file (tables, schema, indices). You must compile SQLCipher as the native SQLite implementation and link rusqlite or use a `sqlcipher` crate/binding. Runtime needs a passphrase/key to open the DB

1. **Android Keystore (hardware-backed):** UI obtains symmetric key or uses keystore to unwrap an encrypted DB key. Rust receives the usable key on each run (via JNI) but never stores it on disk.

**Do not** hardcode keys in the binary or store unencrypted keys on disk.


## 3) How the UI should interact with the core

- UI calls into exported Rust functions for:

  - `init_db(encrypted_key_bytes)` / `open_db(...)`
  - `add_transaction(person, amount, date, note)`
  - `get_balance(person)` and `list_balances()`
  - `list_transactions(person, since, limit)`
  - `backup/restore(encrypted_blob)`
- Keep API small and careful with memory ownership (use safe wrappers for strings and buffers).

---

# Security & hardening checklist

- Use **SQLCipher** for full-file encryption.
- Keep DB in the app’s private directory (Android: internal storage), not external storage.
- Strict file permissions (`0600` equivalent on Android private files).
- Use **Android Keystore** to hold keys or to wrap keys.
- Use **Argon2id** for deriving keys from passphrases; tune memory/time for mobile CPU limits.
- Zero secrets in memory after use (use `zeroize` crate).
- Use prepared statements and parameterized queries (rusqlite does this).
- Wrap multi-statement updates in transactions.
- No sensitive logging. Avoid `tracing` or `println!` of secrets.
- Avoid long-lived keys in memory; require unlocking per session if threat model demands.
- Make encrypted backups optional and always protected (GPG or SQLCipher + passphrase).
- Test on GrapheneOS and ensure no unintended file leaks.

---

> For production with SQLCipher, do **not** use `bundled` unless you build SQLCipher source. Instead build rusqlite to link against your SQLCipher binary. (I’ll explain build options later.)
---

# Android integration approach (recommended)

1. **Rust core**: build as `cdylib` which exposes C ABI functions and/or JNI functions (using the `jni` crate).
2. **Kotlin/Android UI**:

   - Use the Android Keystore to create or retrieve a key (hardware-backed).
   - Use that key to unwrap or generate the DB key (or prompt user passphrase and derive).
   - Pass the DB key bytes into Rust `open_db()` via JNI.
   - Call Rust functions for add/list/get.
3. **Permissions & Storage**:

   - Store DB in `context.getFilesDir()` (private app storage).
   - `MODE_PRIVATE` ensures other apps cannot access files.
4. **Build tool**:

   - Use `cargo-ndk` or `cargo-apk` to build Android ABIs and create `.so` files.
   - Bundle `.so` in the APK under `lib/<ABI>/libledgercore.so`.
   - JNI layer (Kotlin) loads `System.loadLibrary("ledgercore")` and calls methods.

> On GrapheneOS you’ll want to follow its recommended best practices for keystore usage.

---

# Build notes & tooling

- Use `cargo-ndk` to compile rust for Android ABIs:

  ```
  cargo install cargo-ndk
  cargo ndk -t armeabi-v7a -t arm64-v8a -o target/android build --release
  ```
- Include the produced `.so` libraries in your Android project under `app/src/main/jniLibs/<ABI>/`.
- Link to SQLCipher: you’ll add SQLCipher `.so` to the jniLibs and ensure `rusqlite` links against `libsqlcipher.so` instead of `libsqlite3.so`. This can require custom `build.rs` or `pkg-config` environment variables during cargo build.

If you’d like, I can produce a concrete `build.rs` and `Cargo.toml` configuration for linking against SQLCipher for the Android target — say which ABIs to support and I’ll give the commands and file snippets.

---

# UX/security tradeoffs & suggestions

- **Prompt for passphrase once per session** (if you want human input). The UI can lock after X minutes requiring re-entry.
- **Use keystore to avoid user friction** (user doesn’t need to type passphrase each time).
- **When to require passphrase:** If you want maximum protection in case device is unlocked by attacker, require passphrase at each run.
- **Offline backups:** allow the user to export an encrypted backup (`.db` file already encrypted with SQLCipher, or encrypted bundle). If they choose to export, warn about storing backups safely.
- **Audit & tests:** Write unit tests for DB migration, and integration tests for opening DB with wrong key.

---
