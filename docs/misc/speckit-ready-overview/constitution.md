# Project's Constitution and Principles

## The Goal
The project aims to create a **single, consistent ledger** where every entry records **a transaction that changes what someone owes you** or what you owe them. It should be a **simple ledger** that:
- Records **transactions** (with who, when, and how much)
- Can easily compute **balances per person**
- Doesn’t require heavy accounting models

The data structure should be:
- **Append-only** (each new entry adds to history)
- **Queryable by person** (to compute running balances)
- **Human-readable**

## Core Idea
All transactions are viewed from **your perspective**:
- **Positive (+)** → someone owes you (you should receive money)
- **Negative (−)** → you owe someone (you should pay money)

## Key Goals
- **Least overhead**: Minimal runtime, dependencies, setup, and footprint.
- **Maximum security**: Strong prevention of SQL injection, safe parameter handling, memory safety.
- **CRUD efficiency**: Clean code for Create/Read/Update/Delete.
- **SQLite support**: Mature, stable bindings to SQLite.

## Security Best Practices (regardless of language)
1.  **Always use parameterized queries**, never string concatenation.
2.  **Validate and sanitize input** (e.g., strip weird characters in `person` or `note`).
3.  **Use least privileges**: file permissions on `ledger.db` (`chmod 600`).
4.  **Wrap all DB operations in transactions** when doing multiple writes.
5.  **Back up regularly** (SQLite file = single point of truth).

## Context Recap
- **Purpose**: Local ledger app — tracks owes/dues, offline.
- **Platform**: Android (GrapheneOS), compiled as APK.
- **Data**: SQLite (likely encrypted, e.g. via SQLCipher).
- **Threats**: Physical theft, data exposure, malware snooping.
- **Goals**: Minimal overhead, strong security, robust future-proofing.

Additional context:
- Full offline operation (no network).
- Strong on-disk protection (an attacker who copies the `.db` file cannot read it).
- Minimal attack surface in memory and on-disk.
- A Rust core library callable from a mobile UI.

## UX/Security Tradeoffs & Suggestions
- **Prompt for passphrase once per session** (if human input is desired). The UI can lock after X minutes requiring re-entry.
- **Use keystore to avoid user friction** (user doesn’t need to type passphrase each time).
- **When to require passphrase**: If maximum protection is needed in case the device is unlocked by an attacker, require a passphrase at each run.
- **Offline backups**: Allow the user to export an encrypted backup (`.db` file already encrypted with SQLCipher, or encrypted bundle). Warn about storing backups safely.
- **Audit & tests**: Write unit tests for DB migration, and integration tests for opening DB with wrong key.
