# ADR 0001: Core Data Model for zlijadb

- Status: Proposed
- Date: 2026-07-30
- Phase: Phase 1 — Core Local Storage Engine

## Context

Phase 1 of `zlijadb` is focused on building a reliable local database that runs on a single device only, while synchronization, CRDTs, networking, and wasm are explicitly deferred to later phases.[1] In week 1, the plan requires the project to settle the data model decisions and write them down in `docs/adr/001-data-model.md` before implementation expands, because those decisions directly shape the `Database` and `Collection` API as well as the storage layer built on top of `redb`.[1]

The roadmap identifies four specific decisions that this document must capture: the basic unit is a document inside a collection, the internal storage format should be CBOR via `ciborium` with MessagePack as an alternative, keys should be `String` values with lexicographic byte ordering, and every document must remain convertible later into an independent CRDT unit.[1] The plan also sets a clear rule for this phase: do not add future sync complexity now; build a solid local storage model and avoid internal structures that tightly couple documents together.[1]

## Decision

### 1) Basic unit: document inside a collection

`zlijadb` adopts a schema-less document model where the primary public storage unit is a **document** identified by a text key inside a **collection**.[1] This aligns directly with the planned public API, where data access flows through `Database::collection` and then uses operations such as `get`, `put`, `delete`, `scan`, and `range` on `Collection`.[1]

A document is the logical unit of read, write, and storage from the API perspective. The core data model in this phase does not introduce relational tables, engine-enforced links between records, or multi-record structures as the fundamental storage unit; any relationship between documents stays at the application layer rather than the engine layer.

### 2) Internal storage format: CBOR via ciborium

This phase adopts **CBOR** as the default serialization format for Rust values before they are stored in the backend, using `ciborium`.[1] The roadmap recommends this explicitly because it is lighter than JSON for internal storage and is a good fit for later evolution toward Automerge or another CRDT-based layer.[1]

MessagePack via `rmp-serde` remains an acceptable alternative, but it is not the primary choice for Phase 1.[1] As a result, `put` and `get` in `zlijadb-core` should follow a simple rule: generic values `T: Serialize` are encoded into CBOR bytes on write, and `T: DeserializeOwned` values are decoded from those bytes on read, while serialization failures are surfaced through `zlijadb::Error`.[1]

### 3) Keys: UTF-8 String with byte-wise lexicographic ordering

The system uses `String` keys encoded as UTF-8 and treats them in storage according to lexicographic byte ordering.[1] This matters because it enables natural implementations of `scan(prefix)` and `range(start, end)` on top of ordered storage engines such as `redb`, and it supports practical key patterns like `runs/2026-07-*`.[1]

Because of this, result ordering must come from byte-wise comparison of the stored key, not locale-aware or language-aware comparison rules. It should also be documented that key layout is an application responsibility, and good patterns include stable, prefix-friendly designs such as `tasks/`, `runs/`, or `users/123/profile` when efficient range scanning is needed.

### 4) Future independence of each document as a sync unit

Each document must be treated as an independent unit that can later map to its own CRDT object, because the roadmap already fixes the future rule of “one document per sync unit.”[1] For that reason, the core data model must not introduce storage structures that permanently bundle multiple documents into one inseparable value, or enforce cross-document invariants in ways that would be hard to preserve during future synchronization.[1]

This does not prevent applications from using logically related keys or local transactions in the current phase. It does mean that the engine itself should treat each document as an independent entity in addressing, serialization, storage, deletion, and indexing, so that the future CRDT layer can be added on top of the current model instead of forcing a redesign.

## Resulting design details

### Stored entity boundary

The backend stores data as the mapping `(<collection>, <key>) -> <encoded document bytes>`. This fits the Phase 2-3 plan, which maps each collection to a separate redb table, because the collection becomes the natural logical boundary while the key remains the document identifier within that boundary.[1]

This also means collection names are part of the address space, not just descriptive labels. As the roadmap later requires, collection names should be validated early, and empty names or names that could break byte ordering or create naming ambiguity should be rejected.[1]

### Read and write behavior

- `put(key, value)` serializes the value to CBOR and stores it under the given key inside the target collection.[1]
- `get(key)` reads the raw bytes and deserializes them back into the requested type.[1]
- `delete(key)` removes only the targeted document and returns whether the key originally existed, following the planned API.[1]
- `scan(prefix)` and `range(start, end)` operate over the byte-ordered key space, not over document contents.[1]

### Effect on indexing

Because the core model is document-oriented and stores values as CBOR opaque bytes at the storage layer, secondary indexes are not part of the document’s primary representation. This matches the roadmap, which defers a general indexing engine and allows only a simple manual secondary index through a separate table named like `idx_<collection>_<field>`, updated inside the same transaction.[1]

The practical consequence is a clean separation between the **source document** and **derived structures** such as indexes. That keeps the core data model small and understandable, and it avoids polluting the main document representation with internal operational metadata.

## Alternatives considered

### JSON instead of CBOR

JSON is easier to inspect manually, but it is not the recommended internal format in this phase.[1] The roadmap explicitly prefers CBOR because it is lighter than JSON and better suited to local storage and future evolution.[1]

For that reason, JSON is not selected as the primary internal representation, although it may still be appropriate at the CLI boundary or in user-facing interfaces.

### MessagePack instead of CBOR

The roadmap accepts MessagePack via `rmp-serde` as a valid alternative.[1] However, because the current recommendation is CBOR and the goal of Phase 1 is to reduce open design choices and move implementation forward, CBOR is fixed as the default now and any deeper comparison can wait until a concrete performance or compatibility need appears.

### Multi-document aggregate as the core unit

It would be possible in theory to treat a tightly related group of records as one storage unit. That option was rejected because it conflicts directly with the future constraint that each document must be convertible into an independent CRDT sync unit.[1]

## Consequences

### Positive outcomes

- The API stays simple and clear: database → collection → document by key.[1]
- CRUD operations and transactions are easier to implement on top of `redb` because the storage boundaries are explicit and straightforward.[1]
- `scan` and `range` become natural operations because of byte-ordered keys.[1]
- The path toward future synchronization remains open without redefining the engine’s core storage unit.[1]

### Costs and constraints

- The core model does not provide rich relational structure or advanced query capabilities, because the roadmap defers SQL, compound indexes, and full-text search beyond this phase or outside scope.[1]
- Some modeling responsibility shifts to the application, especially for key design and document-to-document relationships.
- Stored data is not as human-readable as JSON because CBOR is a binary format.

## Implementation rules

To prevent drift during implementation, the following rules should be enforced:

1. Every `Collection::put` stores exactly one document for one key.[1]
2. The internal representation must not permanently pack multiple documents into one shared blob.
3. Any index or derived structure must remain separate from the source document and be updated in the same transaction when needed.[1]
4. Ordering in `scan` and `range` must follow byte-wise key ordering, not locale-specific sorting rules.[1]
5. Any future CRDT layer must build on the assumption that the current document is the fundamental independent sync unit.[1]

## Relation to the API

This decision directly supports the proposed `zlijadb-core/src/lib.rs` API, especially because all core `Collection` operations revolve around a text key and a serializable value.[1] It also justifies keeping `Collection` as the main abstraction surface instead of introducing higher-level query builders, schema systems, or entity relationship layers too early.[1]

## Future review

This decision should be revisited only if one of the following happens:

- CBOR fails to meet a demonstrated performance or compatibility requirement.
- A real conflict appears between the “one document = one sync unit” rule and the CRDT choice that is actually adopted later.
- Simple text keys prove insufficient for essential usage patterns.

Until then, this ADR remains the reference for the Phase 1 design of `zlijadb-core` and `zlijadb-storage`.[1]