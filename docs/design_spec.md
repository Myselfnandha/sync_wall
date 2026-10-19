# sync_wall Architecture & Engineering Specification

## 1. System Topology & Seams

```
+-------------------------------------------------------------------------+
|                  Linux CLI / Tauri UI / Android Compose                 |
|             (Presentation layer: pure consumer of the engine)          |
+-------------------------------------------------------------------------+
                                    |
      Linux: Unix Domain Socket     |     Android: UniFFI Foreign Interface
      (JSON-RPC API over tokio)     |     (Direct in-process native bindings)
                                    v
+-------------------------------------------------------------------------+
|                    SyncWallEngine (Deep Core Module)                    |
|                                                                         |
|  +--------------------+  +--------------------+  +-------------------+  |
|  | Derivative Engine  |  | State Sync Engine  |  | TLS Transport     |  |
|  | - EXIF rotation    |  | - HLC clocks       |  | - rustls 1.3      |  |
|  | - fast_img_resize  |  | - Monotonic seq    |  | - Cert fingerprint|  |
|  | - Phone: WebP q90  |  | - Delta batches    |  | - HTTP/1.1 Range  |  |
|  | - PC: Lossless PNG |  | - 90d tombstones   |  | - X-Blake3 check  |  |
|  +--------------------+  +--------------------+  +-------------------+  |
|                                                                         |
|  +-------------------------------------------------------------------+  |
|  | Storage: SQLite (rusqlite) + CAS File Store (objects/ab/cdef...)  |  |
|  +-------------------------------------------------------------------+  |
+-------------------------------------------------------------------------+
                                    |
                    +---------------+---------------+
                    | (Linux daemon only)           | (Android app only)
                    v                               v
+---------------------------------------+   +-----------------------------+
|        WallpaperBackend Trait         |   | WallpaperManager.setBitmap  |
| - XfconfBackend (XFCE, local host)    |   | - Downsampled decode        |
| - SwwwBackend (Wayland, Phase 2)      |   | - FLAG_SYSTEM | FLAG_LOCK   |
| - GnomeBackend (gsettings, Phase 2)   |   | - WorkManager rotation      |
+---------------------------------------+   +-----------------------------+
```

---

## 2. Decision Log Summary

### 2.1 Core Library Seam & UniFFI
- **Decision**: Single deep `SyncWallEngine` struct wrapping SQLite (`rusqlite`), BLAKE3 blob storage, derivative pipeline, and state synchronization behind a minimal transactional interface.
- **Rationale**: Eliminates cross-language state duplication. Both the Linux daemon and Android app call this exact interface via UniFFI, guaranteeing identical migrations, conflict resolution, and hash verification.

### 2.2 Linux WallpaperBackend Trait
- **Decision**: Implement `XfconfBackend` first to satisfy Spike 1 on the current XFCE host. Define the trait for seamless drop-in auto-detection in Phase 2:
  ```rust
  pub trait WallpaperBackend {
      fn name(&self) -> &'static str;
      fn detect() -> bool where Self: Sized;
      fn list_outputs(&self) -> Result<Vec<Output>>;
      fn set(&self, output: &OutputId, img: &Path, fit: Fit) -> Result<()>;
  }
  ```
- **Rules**: Never shell out via command strings (always pass arguments as discrete arrays); iterate all workspaces on XFCE; parse colon-separated `$XDG_CURRENT_DESKTOP`.

### 2.3 Crop & Derivative Engine
- **Decision**: On-demand derivative caching keyed by `blake3(hash || canonical(spec) || pipeline_version)` at `derived/<hash[..2]>/<hash>/<key>.<ext>`.
  ```rust
  pub struct DerivativeSpec {
      pub target: (u32, u32),
      pub fit: Fit,
      pub focus: Option<(f32, f32)>,
      pub format: OutFormat, // Png for Desktop, WebpLossy(90) for Android
  }
  ```
- **Rules**:
  - Processing order: decode -> EXIF orientation -> crop first -> resize with `fast_image_resize` -> encode.
  - Desktop derivatives are lossless PNG (universal compatibility across Linux image setters).
  - Android derivatives are WebP q90 (small network payload, hardware decoded natively).
  - Atomic writes (`.tmp` + rename). Bounded memory (cap input at 64 MP to prevent mobile OOM). Single-flight request deduplication. LRU eviction budget (2 GB desktop, 300 MB phone) on derivatives only (originals are immutable).

### 2.4 State Sync & Replication
- **Decision**: Relational SQLite store with Hybrid Logical Clocks (HLC) and local monotonic `seq` triggers.
- **HLC Structure**: `(physical_ms, counter, device_id)`. Clamped at 24 hours to prevent permanent clock hijacking by misconfigured peers.
- **Monotonic Sequence Cursor**: A local trigger increments `seq` on every insert/update. Peers request `SELECT ... WHERE seq > ?`, enabling transitive propagation across nodes without skipping records merged from third parties.
- **Conflict Resolution**:
  - `assignments`: Last-Write-Wins (LWW) per `(device_id, screen_id)`.
  - `images`: Immutable by BLAKE3 hash. Tags and tombstones resolved via LWW.
  - `collections`: Metadata LWW; `collection_items` are independent rows (concurrent adds both survive).
  - `devices`: Device owns its own record.
- **Tombstones**: Retained for 90 days or until all known peers have acknowledged past the sequence.

### 2.5 Transport & Pairing
- **Decision**: HTTP/1.1 semantics over TLS 1.3 (`rustls`). The client lives entirely inside the Rust core (Android calls `engine.sync_now()`, no OkHttp rewrites).
- **Identity**: The self-signed TLS certificate is the device identity. Its SHA-256 fingerprint is the device ID. Custom `rustls` verifiers accept only fingerprints in the trusted-devices SQLite table.
- **Pairing**: QR code encodes `{host, port, token, cert_fingerprint}`. Endpoint `POST /v1/pair` requires a single-use token expiring in 2 minutes, verified in constant time.
- **Direction**: Linux runs the server (bound to LAN interface only). Android is strictly a client, pulling on app open, Wi-Fi connected, or periodic WorkManager runs.
- **Blobs**: Chunked downloads support standard HTTP `Range:` headers and verify `X-Blake3` on completion of atomic `.part` files.

### 2.6 Desktop UI & ReUI Components
- **Decision**: Strictly defer desktop UI to Phase 6. Develop and verify the CLI and Android app first.
- **Phase 6 Target**: React + Vite + Tailwind + ReUI/shadcn in Tauri as a thin proxy client over the Unix domain socket.
- **Contract**: JSON-RPC types exported to TypeScript via `ts-rs` / `specta`. No business logic in React or Tauri.

---

## 3. Directory Structure

```
sync_wall/
├── Cargo.toml
├── core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs          # SyncWallEngine facade
│       ├── model.rs        # Core data types & HLC
│       ├── store.rs        # SQLite schema & CAS blob management
│       ├── crop.rs         # fast_image_resize, EXIF, DerivativeSpec
│       ├── sync/           # HLC delta generation & batch apply
│       ├── net/            # rustls server & client, HTTP/1.1 endpoints
│       └── uniffi_api.rs   # UniFFI interface definition
├── linux/
│   ├── backend/            # WallpaperBackend trait & XfconfBackend
│   ├── daemon/             # sync_walld (tokio Unix socket server)
│   ├── cli/                # sync_wall CLI
│   └── ui/                 # Phase 6: Tauri + React + ReUI
├── android/                # Phase 3: Kotlin app, Compose, UniFFI bridge
└── docs/
    ├── protocol.md         # Protocol specification
    └── design_spec.md      # This document
```
