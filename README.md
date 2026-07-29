<div align="center">

# zlijadb

**An open-source, CRDT-based sync & merge database engine for the edge.**
Offline-first. Conflict-free. Built in Rust.

*Like zellige tiles — your data pieces always fit together.*

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Status: Pre-alpha](https://img.shields.io/badge/status-pre--alpha-red.svg)](#project-status)

</div>

---

## What is zlijadb?

zlijadb is **not** another full database competing with SQLite or Postgres. It is a lightweight **sync & merge layer**: an embedded storage engine with first-class [CRDT](https://crdt.tech/) (Conflict-free Replicated Data Type) support, designed so that data written on multiple devices — even while offline for days — **always merges back together without conflicts and without data loss**.

Think of it as the missing piece between "I store data locally" and "I sync data reliably across devices".

```
┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│  Device A   │         │  Device B   │         │   Server    │
│  (offline)  │ ◄─────► │  (browser)  │ ◄─────► │  (cloud)    │
│   zlijadb   │  delta  │   zlijadb   │  delta  │   zlijadb   │
└─────────────┘  sync   └─────────────┘  sync   └─────────────┘
        All replicas converge — guaranteed, without a central arbiter.
```

## Why?

The offline-first / local-first space has a real gap:

- **[MongoDB Atlas Device Sync was discontinued](https://www.mongodb.com/docs/atlas/app-services/sync/device-sync-deprecation/)** (September 2025), leaving many apps without an open-source alternative.
- **[cr-sqlite](https://github.com/vlcn-io/cr-sqlite)** proved the demand for SQLite+CRDT — but has been dormant since January 2024.
- **[Turso](https://turso.tech/)** and similar edge databases still resolve conflicts with simple *last-push-wins*, which silently drops concurrent writes.

zlijadb aims to fill that gap: a permissively-licensed, actively-maintained, pure-Rust engine with **real conflict-free merging**.

## Design principles

| Principle | What it means in practice |
|---|---|
| **Offline-first, always** | The local replica is the source of truth. The network is an optimization, not a dependency. |
| **Conflict-free by construction** | Merging is based on CRDT semantics, not "last write wins". Concurrent edits converge deterministically. |
| **Pure Rust, zero C dependencies** | Built on [`redb`](https://github.com/cberner/redb) for storage. Easy to build, easy to embed, memory-safe. |
| **Runs anywhere** | Native (Linux/macOS/Windows), mobile, and **WebAssembly** — the same engine in your backend and in the browser. |
| **Weak-network friendly** | Delta-based sync designed to survive slow and flaky connections (yes, even 2G). |
| **No consensus overhead** | No Raft, no Paxos, no quorums. CRDTs guarantee convergence without coordination. |

## Architecture

```
zlijadb/
├── crates/
│   ├── zlijadb-core/      # Core database logic and public API
│   ├── zlijadb-storage/   # Storage abstraction over redb
│   ├── zlijadb-crdt/      # CRDT merge layer
│   ├── zlijadb-sync/      # Delta-based sync protocol
│   ├── zlijadb-cli/       # Command-line tool
│   └── zlijadb-ffi/       # Bindings (Wasm/JS first, others later)
```

- **Storage:** [`redb`](https://github.com/cberner/redb) — a pure-Rust embedded key-value store with ACID transactions and official wasm32 support.
- **Merging:** a mature CRDT library as the merge kernel (Automerge / yrs), wrapped behind a stable zlijadb API. A custom CRDT layer may replace it later as a performance optimization.
- **Sync:** delta-only transfer per sync unit, with an offline queue and automatic retry.

> ⚠️ **Honest caveat:** CRDT convergence guarantees your *data* merges correctly. It does **not** guarantee exactly-once *side effects*. If your records represent real-world actions (payments, job runs), design them to be idempotent.

## Quick start

> 🚧 zlijadb is in early development — the API below is a design preview, not a stable interface.

```rust
use zlijadb::Database;

// Open (or create) a local database
let db = Database::open("my-app.zlija")?;

// Write locally — works fully offline
let mut tasks = db.collection("tasks")?;
tasks.put("task-1", &Task { title: "Ship v0.1".into(), done: false })?;

// Sync with a peer or server whenever a connection is available
db.sync("https://sync.example.com/my-app").await?;

// Concurrent edits from other devices merge automatically — no conflicts
```

## Project status

**Pre-alpha.** zlijadb is under active initial development and is not yet ready for production use.

| Milestone | Status |
|---|---|
| Phase 0 — Foundation (repo, CI, license, CRDT spike) | 🔜 In progress |
| Phase 1 — Local storage engine (`redb`-backed, ACID, no sync) | ⏳ Planned |
| Phase 2 — CRDT merge layer + delta sync protocol | ⏳ Planned |
| Phase 3 — `v0.1` first usable release on crates.io | ⏳ Planned |
| Phase 4 — Hardening: deterministic simulation testing, chaos/crash testing | ⏳ Planned |
| Phase 5 — WebAssembly build + JS/TS bindings | ⏳ Planned |
| Phase 6 — `v1.0` public launch | ⏳ Planned |

Release gates before `v0.1`: 72 hours of continuous real-world dogfooding without data loss, and 10,000 forced-crash tests without corruption.

## Comparison

| | zlijadb | Turso | cr-sqlite | PowerSync |
|---|---|---|---|---|
| Conflict resolution | **CRDT (conflict-free)** | Last-push-wins | CRDT | Server-authoritative |
| Actively maintained | ✅ | ✅ | ❌ (dormant since 2024) | ✅ |
| Pure Rust, no C deps | ✅ | ✅ | ❌ (SQLite extension) | ❌ |
| Runs in the browser (Wasm) | ✅ (planned) | Partial | ✅ | ✅ |
| Open source, self-hostable | ✅ MIT/Apache-2.0 | Partially | ✅ | Partially |

## Contributing

Contributions are welcome — from code to docs to bug reports. The project is young, which means your impact can be huge.

1. Read [`CONTRIBUTING.md`](CONTRIBUTING.md)
2. Look for issues labeled [`good first issue`](../../issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
3. Join the discussion in [GitHub Discussions](../../discussions)

## About the name

*Zlija* (زليجة) is the Moroccan Darija word for a single **zellige tile** — the hand-cut mosaic pieces that fit together perfectly to form intricate patterns. That's exactly what zlijadb does with your data: independent pieces, edited anywhere, always fitting back together.

Built in Agadir, Morocco 🇲🇦 — designed for the whole world, including the parts of it with bad Wi-Fi.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you shall be dual-licensed as above, without any additional terms or conditions.
