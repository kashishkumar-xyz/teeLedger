<!--
Sync Impact Report:
- Version change: none → 1.0.0
- Added sections: Core Principles (5 principles), Security Requirements, Development Workflow, Governance
- Templates requiring updates: ✅ .specify/templates/plan-template.md, ✅ .specify/templates/spec-template.md, ✅ .specify/templates/tasks-template.md
- Follow-up TODOs: Update docs/README.md to reference constitution principles
-->

# teeLedger Constitution

## Core Principles

### Append-Only Ledger
Every transaction is an immutable addition to the ledger history. This ensures auditability, prevents data loss, and maintains a complete record of all financial interactions.

### Perspective-Based Transactions
All transactions are recorded from the user's viewpoint: positive amounts indicate money owed to the user, negative amounts indicate money the user owes. This simplifies balance calculations and user understanding.

### Security-First Design
Implement parameterized queries, input validation, least privilege access, transaction wrapping for writes, and regular backups. Rationale: Protects against SQL injection, data corruption, and unauthorized access.

### Minimal Overhead
Minimize runtime resources, dependencies, setup complexity, and storage footprint. Rationale: Enables offline operation, quick deployment, and efficient performance on resource-constrained devices.

### SQLite Integration
Use mature SQLite bindings with encryption support for data persistence. Rationale: Provides a stable, single-file database that is secure, portable, and suitable for offline applications.

## Security Requirements

- Always use parameterized queries to prevent SQL injection.
- Validate and sanitize all user inputs, stripping invalid characters.
- Apply least privilege principles to database file permissions.
- Wrap multiple database operations in transactions.
- Implement regular backup procedures for data safety.

## Development Workflow

- Follow Test-Driven Development: write tests before implementation.
- Expose library functionality via CLI interfaces.
- Ensure all features start as standalone libraries.
- Conduct integration testing for new contracts and changes.
- Maintain observability through structured logging.

## Governance

The constitution supersedes all other practices. Amendments require documentation, approval, and migration plans. Versioning follows semantic rules: MAJOR for incompatible changes, MINOR for additions, PATCH for clarifications. All PRs must verify compliance; complexity must be justified.

**Version**: 1.0.0 | **Ratified**: 2025-10-14 | **Last Amended**: 2025-10-14