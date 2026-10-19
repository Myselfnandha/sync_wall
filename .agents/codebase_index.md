# Codebase Context: sync_wall
> Auto-generated on 2026-10-19 14:23:26. 16 files, ~45,550 tokens.

## Structure
```text
sync_wall/
├── Cargo.lock
├── Cargo.toml
├── core/
│   ├── Cargo.toml
│   └── src/
│       ├── crop.rs
│       ├── lib.rs
│       ├── model.rs
│       ├── net.rs
│       └── store.rs
├── docs/
│   ├── design_spec.md
│   └── protocol.md
├── linux/
│   ├── backend/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── cli/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── daemon/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
└── test_wallpaper.png
```

## File Index

- `Cargo.lock` — Lockfile manifest: 279 resolved dependencies
- `Cargo.toml` — TOML config: workspace
- `core/Cargo.toml` — Cargo.toml (Rust project)
- `core/src/crop.rs` — structs: CropEngine; enums: CropError; 4 functions
- `core/src/lib.rs` — structs: SyncWallEngine; enums: EngineError
- `core/src/model.rs` — structs: Hash, Hlc, DerivativeSpec; enums: Fit, OutFormat, DeviceKind
- `core/src/net.rs` — structs: DeviceIdentity, ServerState, HelloResponse; enums: NetError; 9 functions
- `core/src/store.rs` — structs: Store; enums: StoreError
- `docs/design_spec.md` — sync_wall Architecture & Engineering Specification (113 lines)
- `docs/protocol.md` — sync_wall Wire & Synchronization Protocol (v1) (131 lines)
- `linux/backend/Cargo.toml` — Cargo.toml (Rust project)
- `linux/backend/src/lib.rs` — structs: Output, XfconfBackend, MockBackend; enums: BackendError; traits: WallpaperBackend; 1 functions
- `linux/cli/Cargo.toml` — Cargo.toml (Rust project)
- `linux/cli/src/main.rs` — structs: Cli; enums: Commands; 4 functions
- `linux/daemon/Cargo.toml` — Cargo.toml (Rust project)
- `linux/daemon/src/main.rs` — structs: Args, RpcRequest, RpcResponse; 3 functions

