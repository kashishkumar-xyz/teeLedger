# Project's Baseline Specifications and Technical Needs

## Core Data Structure: A List of Transaction Objects
At its heart, the system requires a list of transaction objects, exemplified as:
```json
[
    { "date": "2025-10-12", "person": "Adam", "amount": 10, "note": "Tea sale" },
    { "date": "2025-10-12", "person": "Ron",  "amount": -30, "note": "Bought milk" },
    { "date": "2025-10-13", "person": "Adam", "amount": -10, "note": "Payment" },
    { "date": "2025-10-14", "person": "Ron",  "amount": +20, "note": "Credit sale" },
    { "date": "2025-10-15", "person": "Ron",  "amount": -10, "note": "Paid" },
    { "date": "2025-10-16", "person": "Adam", "amount": -20, "note": "Paid" }
]
```
This single structure supports all core operations.

## Derived Structure: Balances Dictionary
Balances can be computed by aggregating transactions, resulting in a dictionary like:
```python
balances = {}
for t in transactions:
    balances[t["person"]] = balances.get(t["person"], 0) + t["amount"]
```
Example result:
```json
{'Adam': -20, 'Ron': -20}
```
Interpretation:
- A **negative** balance = you owe them.
- A **positive** balance = they owe you.

## Language
**Rust** 🦀 is the chosen language due to:
- **Low runtime overhead** and compilation to native binary.
- **Memory-safety**, strong type safety, and enforced prepared statements for **security**.
- Slightly higher development effort but best for security & performance.

## Encrypted Storage Engine
- **Preferred**: Use SQLite compiled with **SQLCipher** for full-file AES-256 encryption. The Rust core will use `rusqlite` or bindings linked to this SQLCipher-built library.
- **Alternative**: If SQLCipher integration is too complex for Android, encrypt sensitive fields at the application level using authenticated encryption (AES-GCM or XChaCha20-Poly1305). This is less ideal as it leaves the DB structure visible.

## Key Management
- **Best**: Utilize Android’s hardware-backed keystore (supported by GrapheneOS) to store the DB encryption key or a key-encryption-key (KEK). The UI (Kotlin/Java) accesses the keystore and passes the key (or unwraps a key-encryption token) to the Rust library at runtime.
- **Other**: Prompt the user for a passphrase at app start (zero it after use) and derive the DB key using Argon2id or scrypt.

**Important**: Do not hardcode keys in the binary or store unencrypted keys on disk.

## Frontend/UI Interaction with Core
Any UI (Kotlin, Flutter, React Native) will call the Rust core via JNI/FFI. The UI and storage logic must be kept separate, and the UI should be minimal, never logging or caching secrets.

## Deployment
- Build the Rust library for Android targets (arm64-v8a etc.), producing a `.so` file to be bundled into the APK.
- Ensure file permissions and storage location make the DB inaccessible to other apps/users (private app storage).

## SQLCipher Specifics
- **SQLCipher (recommended)**: Encrypts the entire SQLite file (tables, schema, indices). Requires compiling SQLCipher as the native SQLite implementation and linking `rusqlite` or using a `sqlcipher` crate/binding. A passphrase/key is needed at runtime to open the DB.

## Android Keystore Specifics
- **Android Keystore (hardware-backed)**: The UI obtains a symmetric key or uses the keystore to unwrap an encrypted DB key. Rust receives the usable key on each run (via JNI) but never stores it on disk.

## UI Interaction with Core API
The UI should interact with the core via exported Rust functions for:
- `init_db(encrypted_key_bytes)` / `open_db(...)`
- `add_transaction(person, amount, date, note)`
- `get_balance(person)` and `list_balances()`
- `list_transactions(person, since, limit)`
- `backup/restore(encrypted_blob)`

The API should be small, and memory ownership should be handled carefully (using safe wrappers for strings and buffers).