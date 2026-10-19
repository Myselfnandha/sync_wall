# Codebase Summary: sync_wall

## Overview
- **Scan Date:** 2026-10-19 14:23:26
- **Source Folder:** `/home/nandha/Desktop/sync_wall`
- **Total Text Files:** 16
- **Estimated Token Count:** 45,550

## File Summaries

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

## Directory Tree
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

## File Contents

### File: `Cargo.lock`
- **Summary:** Lockfile manifest: 279 resolved dependencies
- **Path:** `Cargo.lock`
- **Estimated Tokens:** 2,030
- **mtime:** 1792399776.529

> [!NOTE]
> **Compact Dependency Manifest**: Extracted 279 resolved packages from `Cargo.lock`.
> Full cryptographic integrity hashes and raw lock metadata omitted to preserve context window.

| Package | Resolved Version |
| :--- | :--- |
| `adler2` | `2.0.1` |
| `ahash` | `0.8.12` |
| `aho-corasick` | `1.1.5` |
| `android_system_properties` | `0.1.6` |
| `anstream` | `1.0.0` |
| `anstyle` | `1.0.14` |
| `anstyle-parse` | `1.0.0` |
| `anstyle-query` | `1.1.5` |
| `anstyle-wincon` | `3.0.11` |
| `arrayvec` | `0.7.8` |
| `asn1-rs` | `0.6.2` |
| `asn1-rs-derive` | `0.5.1` |
| `asn1-rs-impl` | `0.2.0` |
| `async-trait` | `0.1.92` |
| `atomic-waker` | `1.1.2` |
| `autocfg` | `1.5.1` |
| `axum` | `0.7.9` |
| `axum-core` | `0.4.5` |
| `base64` | `0.23.1` |
| `base64` | `0.22.1` |
| `bitflags` | `2.13.2` |
| `blake3` | `1.8.7` |
| `bumpalo` | `3.20.3` |
| `bytemuck` | `1.25.2` |
| `byteorder-lite` | `0.1.0` |
| `bytes` | `1.12.1` |
| `cc` | `1.9.0` |
| `cfg-if` | `1.0.5` |
| `cfg_aliases` | `0.2.2` |
| `chacha20` | `0.10.2` |
| `chrono` | `0.4.45` |
| `clap` | `4.6.7` |
| `clap_builder` | `4.6.7` |
| `clap_derive` | `4.6.7` |
| `clap_lex` | `1.1.1` |
| `colorchoice` | `1.0.5` |
| `constant_time_eq` | `0.4.2` |
| `core-foundation-sys` | `0.8.7` |
| `cpufeatures` | `0.3.1` |
| `crc32fast` | `1.5.2` |
| `data-encoding` | `2.11.1` |
| `der-parser` | `9.0.0` |
| `deranged` | `0.5.9` |
| `dirs` | `5.0.1` |
| `dirs-sys` | `0.4.1` |
| `displaydoc` | `0.2.7` |
| `document-features` | `0.2.12` |
| `errno` | `0.3.14` |
| `fallible-iterator` | `0.3.0` |
| `fallible-streaming-iterator` | `0.1.9` |
| `fast_image_resize` | `5.5.0` |
| `fastrand` | `2.5.0` |
| `fdeflate` | `0.3.7` |
| `find-msvc-tools` | `0.1.15` |
| `flate2` | `1.1.10` |
| `form_urlencoded` | `1.2.2` |
| `futures-channel` | `0.3.34` |
| `futures-core` | `0.3.34` |
| `futures-io` | `0.3.34` |
| `futures-macro` | `0.3.34` |
| `futures-sink` | `0.3.34` |
| `futures-task` | `0.3.34` |
| `futures-util` | `0.3.34` |
| `getrandom` | `0.4.3` |
| `getrandom` | `0.2.17` |
| `hashbrown` | `0.14.5` |
| `hashlink` | `0.9.1` |
| `heck` | `0.5.0` |
| `hex` | `0.4.3` |
| `http` | `1.5.0` |
| `http-body` | `1.1.0` |
| `http-body-util` | `0.1.5` |
| `httparse` | `1.10.1` |
| `httpdate` | `1.0.3` |
| `hyper` | `1.12.0` |
| `hyper-rustls` | `0.27.10` |
| `hyper-util` | `0.1.21` |
| `iana-time-zone` | `0.1.65` |
| `iana-time-zone-haiku` | `0.1.2` |
| `icu_collections` | `2.3.0` |
| `icu_locale_core` | `2.3.0` |
| `icu_normalizer` | `2.3.0` |
| `icu_normalizer_data` | `2.3.0` |
| `icu_properties` | `2.3.0` |
| `icu_properties_data` | `2.3.0` |
| `icu_provider` | `2.3.1` |
| `idna` | `1.1.0` |
| `idna_adapter` | `1.2.2` |
| `image` | `0.25.10` |
| `image-webp` | `0.2.4` |
| `ipnet` | `2.12.2` |
| `is_terminal_polyfill` | `1.70.2` |
| `itoa` | `1.0.18` |
| `js-sys` | `0.3.106` |
| `kamadak-exif` | `0.5.5` |
| `lazy_static` | `1.5.1` |
| `libc` | `0.2.191` |
| `libredox` | `0.1.25` |
| `libsqlite3-sys` | `0.28.0` |
| `linux-raw-sys` | `0.12.1` |
| `litemap` | `0.8.3` |
| `litrs` | `1.0.1` |
| `lock_api` | `0.4.14` |
| `log` | `0.4.34` |
| `lru-slab` | `0.1.3` |
| `matchers` | `0.2.0` |
| `matchit` | `0.7.3` |
| `memchr` | `2.8.3` |
| `mime` | `0.3.17` |
| `minimal-lexical` | `0.2.1` |
| `miniz_oxide` | `0.8.9` |
| `miniz_oxide` | `0.9.1` |
| `mio` | `1.2.4` |
| `moxcms` | `0.8.1` |
| `mutate_once` | `0.1.2` |
| `nom` | `7.1.3` |
| `nu-ansi-term` | `0.50.3` |
| `num-bigint` | `0.4.8` |
| `num-conv` | `0.2.2` |
| `num-integer` | `0.1.47` |
| `num-traits` | `0.2.19` |
| `oid-registry` | `0.7.1` |
| `once_cell` | `1.21.4` |
| `once_cell_polyfill` | `1.70.2` |
| `option-ext` | `0.2.0` |
| `parking_lot` | `0.12.5` |
| `parking_lot_core` | `0.9.12` |
| `pem` | `3.0.6` |
| `percent-encoding` | `2.3.2` |
| `pin-project-lite` | `0.2.17` |
| `pkg-config` | `0.3.34` |
| `png` | `0.18.1` |
| `potential_utf` | `0.1.6` |
| `powerfmt` | `0.2.1` |
| `ppv-lite86` | `0.2.21` |
| `proc-macro2` | `1.0.107` |
| `pxfm` | `0.1.30` |
| `qrcode` | `0.14.1` |
| `quick-error` | `2.0.1` |
| `quinn` | `0.11.12` |
| `quinn-proto` | `0.11.19` |
| `quinn-udp` | `0.5.16` |
| `quote` | `1.0.47` |
| `r-efi` | `6.0.0` |
| `rand` | `0.8.8` |
| `rand` | `0.10.3` |
| `rand_chacha` | `0.3.1` |
| `rand_core` | `0.10.1` |
| `rand_core` | `0.6.4` |
| `rand_pcg` | `0.10.2` |
| `rcgen` | `0.13.2` |
| `redox_syscall` | `0.5.18` |
| `redox_users` | `0.4.6` |
| `regex-automata` | `0.4.18` |
| `regex-syntax` | `0.8.11` |
| `reqwest` | `0.12.28` |
| `ring` | `0.17.14` |
| `rusqlite` | `0.31.0` |
| `rustc-hash` | `2.1.3` |
| `rusticata-macros` | `4.1.0` |
| `rustix` | `1.1.5` |
| `rustls` | `0.23.46` |
| `rustls-pki-types` | `1.15.1` |
| `rustls-webpki` | `0.103.15` |
| `rustversion` | `1.0.23` |
| `ryu` | `1.0.23` |
| `scopeguard` | `1.2.0` |
| `serde` | `1.0.229` |
| `serde_core` | `1.0.229` |
| `serde_derive` | `1.0.229` |
| `serde_json` | `1.0.154` |
| `serde_path_to_error` | `0.1.20` |
| `serde_urlencoded` | `0.7.1` |
| `sharded-slab` | `0.1.7` |
| `shlex` | `2.0.1` |
| `signal-hook-registry` | `1.4.8` |
| `simd-adler32` | `0.3.10` |
| `slab` | `0.4.12` |
| `smallvec` | `1.16.3` |
| `socket2` | `0.6.5` |
| `stable_deref_trait` | `1.2.1` |
| `strsim` | `0.11.1` |
| `subtle` | `2.6.1` |
| `syn` | `2.0.119` |
| `syn` | `3.0.9` |
| `sync_wall` | `0.1.0` |
| `sync_wall_backend` | `0.1.0` |
| `sync_wall_core` | `0.1.0` |
| `sync_walld` | `0.1.0` |
| `sync_wrapper` | `1.0.2` |
| `synstructure` | `0.13.2` |
| `synstructure` | `0.14.0` |
| `tempfile` | `3.27.0` |
| `thiserror` | `1.0.69` |
| `thiserror` | `2.0.21` |
| `thiserror-impl` | `2.0.21` |
| `thiserror-impl` | `1.0.69` |
| `thread_local` | `1.1.10` |
| `time` | `0.3.55` |
| `time-core` | `0.1.9` |
| `time-macros` | `0.2.32` |
| `tinystr` | `0.8.4` |
| `tinyvec` | `1.13.3` |
| `tokio` | `1.53.3` |
| `tokio-macros` | `2.7.2` |
| `tokio-rustls` | `0.26.6` |
| `tokio-util` | `0.7.20` |
| `tower` | `0.5.3` |
| `tower-http` | `0.6.11` |
| `tower-layer` | `0.3.3` |
| `tower-service` | `0.3.3` |
| `tracing` | `0.1.44` |
| `tracing-attributes` | `0.1.31` |
| `tracing-core` | `0.1.36` |
| `tracing-log` | `0.2.0` |
| `tracing-subscriber` | `0.3.23` |
| `try-lock` | `0.2.5` |
| `unicode-ident` | `1.0.27` |
| `untrusted` | `0.9.0` |
| `url` | `2.5.8` |
| `utf8_iter` | `1.0.4` |
| `utf8parse` | `0.2.2` |
| `uuid` | `1.28.0` |
| `valuable` | `0.1.1` |
| `vcpkg` | `0.2.15` |
| `version_check` | `0.9.5` |
| `want` | `0.3.2` |
| `wasi` | `0.11.1+wasi-snapshot-preview1` |
| `wasm-bindgen` | `0.2.129` |
| `wasm-bindgen-futures` | `0.4.79` |
| `wasm-bindgen-macro` | `0.2.129` |
| `wasm-bindgen-macro-support` | `0.2.129` |
| `wasm-bindgen-shared` | `0.2.129` |
| `wasm-streams` | `0.4.2` |
| `web-sys` | `0.3.106` |
| `web-time` | `1.1.0` |
| `windows-core` | `0.62.2` |
| `windows-implement` | `0.60.2` |
| `windows-interface` | `0.59.3` |
| `windows-link` | `0.2.1` |
| `windows-result` | `0.4.1` |
| `windows-strings` | `0.5.1` |
| `windows-sys` | `0.61.2` |
| `windows-sys` | `0.48.0` |
| `windows-sys` | `0.52.0` |
| `windows-targets` | `0.48.5` |
| `windows-targets` | `0.52.6` |
| `windows_aarch64_gnullvm` | `0.52.6` |
| `windows_aarch64_gnullvm` | `0.48.5` |
| `windows_aarch64_msvc` | `0.48.5` |
| `windows_aarch64_msvc` | `0.52.6` |
| `windows_i686_gnu` | `0.48.5` |
| `windows_i686_gnu` | `0.52.6` |
| `windows_i686_gnullvm` | `0.52.6` |
| `windows_i686_msvc` | `0.52.6` |
| `windows_i686_msvc` | `0.48.5` |
| `windows_x86_64_gnu` | `0.52.6` |
| `windows_x86_64_gnu` | `0.48.5` |
| `windows_x86_64_gnullvm` | `0.52.6` |
| `windows_x86_64_gnullvm` | `0.48.5` |
| `windows_x86_64_msvc` | `0.48.5` |
| `windows_x86_64_msvc` | `0.52.6` |
| `writeable` | `0.6.4` |
| `x509-parser` | `0.16.0` |
| `yasna` | `0.5.2` |
| `yoke` | `0.8.3` |
| `yoke-derive` | `0.8.4` |
| `zerocopy` | `0.8.63` |
| `zerocopy-derive` | `0.8.63` |
| `zerofrom` | `0.1.8` |
| `zerofrom-derive` | `0.1.8` |
| `zeroize` | `1.9.1` |
| `zerotrie` | `0.2.5` |
| `zerovec` | `0.11.8` |
| `zerovec-derive` | `0.11.6` |
| `zlib-rs` | `0.6.8` |
| `zmij` | `1.0.23` |
| `zune-core` | `0.5.3` |
| `zune-jpeg` | `0.5.15` |

---

### File: `Cargo.toml`
- **Summary:** TOML config: workspace
- **Path:** `Cargo.toml`
- **Estimated Tokens:** 27
- **mtime:** 1792398799.259

```toml
[workspace]
members = [
    "core",
    "linux/backend",
    "linux/daemon",
    "linux/cli",
]
resolver = "2"
```

---

### File: `core/Cargo.toml`
- **Summary:** Cargo.toml (Rust project)
- **Path:** `core/Cargo.toml`
- **Estimated Tokens:** 266
- **mtime:** 1792399323.452

```toml
[package]
name = "sync_wall_core"
version = "0.1.0"
edition = "2021"

[dependencies]
blake3 = "1.5"
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
image = { version = "0.25", default-features = false, features = ["png", "jpeg", "webp"] }
fast_image_resize = "5"
thiserror = "1.0"
uuid = { version = "1.8", features = ["v4", "serde"] }
rcgen = { version = "0.13", features = ["pem", "x509-parser"] }
rustls = { version = "0.23", default-features = false, features = ["ring", "std"] }
tokio = { version = "1.38", features = ["full"] }
axum = { version = "0.7", features = ["tokio"] }
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls-manual-roots", "json", "stream"] }
hex = "0.4"
chrono = { version = "0.4", features = ["serde"] }
tempfile = "3.10"
tracing = "0.1"
tracing-subscriber = "0.3"
subtle = "2.5"
kamadak-exif = "0.5"
parking_lot = "0.12"
rand = "0.8"
pem = "3.0"

[dev-dependencies]
tokio = { version = "1.38", features = ["full", "test-util"] }
```

---

### File: `core/src/crop.rs`
- **Summary:** structs: CropEngine; enums: CropError; 4 functions
- **Path:** `core/src/crop.rs`
- **Estimated Tokens:** 2,804
- **mtime:** 1792399337.593

```rust
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use fast_image_resize as fr;
use image::{DynamicImage, GenericImageView, ImageFormat};
use thiserror::Error;

use crate::model::{DerivativeSpec, Fit, Hash, OutFormat};

#[derive(Error, Debug)]
pub enum CropError {
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image too large: {0} pixels (max 64MP)")]
    TooLarge(u64),
    #[error("Invalid dimensions: {0}x{1}")]
    InvalidDimensions(u32, u32),
    #[error("Source not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, CropError>;

pub struct CropEngine {
    derived_dir: PathBuf,
    in_flight: Mutex<HashSet<String>>,
}

impl CropEngine {
    pub fn new<P: AsRef<Path>>(derived_dir: P) -> Self {
        Self {
            derived_dir: derived_dir.as_ref().to_path_buf(),
            in_flight: Mutex::new(HashSet::new()),
        }
    }

    pub fn derivative_path(&self, source_hash: &Hash, spec: &DerivativeSpec) -> PathBuf {
        let hex = source_hash.to_hex();
        let prefix = &hex[..2];
        let key = spec.cache_key(source_hash);
        self.derived_dir.join(prefix).join(&hex).join(key)
    }

    /// Read source image, apply EXIF orientation, crop, resize with fast_image_resize, and cache.
    pub fn get_or_render<P: AsRef<Path>>(
        &self,
        source_hash: &Hash,
        source_path: P,
        spec: &DerivativeSpec,
    ) -> Result<PathBuf> {
        let dest = self.derivative_path(source_hash, spec);

        // Fast cache hit
        if dest.exists() && fs::metadata(&dest).map(|m| m.len() > 0).unwrap_or(false) {
            return Ok(dest);
        }

        let key = spec.cache_key(source_hash);
        // Single-flight deduplication
        {
            let mut inflight = self.in_flight.lock().unwrap();
            while inflight.contains(&key) {
                drop(inflight);
                std::thread::sleep(std::time::Duration::from_millis(20));
                if dest.exists() {
                    return Ok(dest);
                }
                inflight = self.in_flight.lock().unwrap();
            }
            inflight.insert(key.clone());
        }

        struct InFlightGuard<'a> {
            set: &'a Mutex<HashSet<String>>,
            key: String,
        }
        impl<'a> Drop for InFlightGuard<'a> {
            fn drop(&mut self) {
                let mut lock = self.set.lock().unwrap();
                lock.remove(&self.key);
            }
        }
        let _guard = InFlightGuard {
            set: &self.in_flight,
            key,
        };

        let source_path = source_path.as_ref();
        if !source_path.exists() {
            return Err(CropError::NotFound(source_path.display().to_string()));
        }

        let source_bytes = fs::read(source_path)?;
        let mut img = image::load_from_memory(&source_bytes)?;

        // Cap decoded memory to 64 MP (prevent OOM)
        let total_pixels = (img.width() as u64) * (img.height() as u64);
        if total_pixels > 64_000_000 {
            return Err(CropError::TooLarge(total_pixels));
        }

        // Apply EXIF orientation
        img = apply_exif_orientation(&source_bytes, img);

        let (target_w, target_h) = spec.target;
        if target_w == 0 || target_h == 0 {
            return Err(CropError::InvalidDimensions(target_w, target_h));
        }

        // Step 1: Crop first to target aspect ratio (saves CPU prior to resize)
        let cropped = crop_to_aspect(&img, target_w, target_h, spec.fit, spec.focus);

        // Step 2: SIMD resize using fast_image_resize
        let resized = resize_simd(&cropped, target_w, target_h)?;

        // Step 3: Encode to specified format
        let encoded_bytes = encode_image(&resized, spec.format)?;

        // Step 4: Atomic write (.tmp -> rename)
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let tmp_path = dest.with_extension("tmp");
        let mut file = File::create(&tmp_path)?;
        file.write_all(&encoded_bytes)?;
        file.sync_all()?;
        fs::rename(tmp_path, &dest)?;

        Ok(dest)
    }
}

/// Parse EXIF tag from bytes and rotate/flip DynamicImage accordingly
fn apply_exif_orientation(bytes: &[u8], img: DynamicImage) -> DynamicImage {
    let mut cursor = Cursor::new(bytes);
    if let Ok(exif_reader) = exif::Reader::new().read_from_container(&mut cursor) {
        if let Some(field) = exif_reader.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
            if let Some(val) = field.value.get_uint(0) {
                return match val {
                    1 => img,
                    2 => img.fliph(),
                    3 => img.rotate180(),
                    4 => img.flipv(),
                    5 => img.rotate90().fliph(),
                    6 => img.rotate90(),
                    7 => img.rotate270().fliph(),
                    8 => img.rotate270(),
                    _ => img,
                };
            }
        }
    }
    img
}

/// Crop source image to match target aspect ratio (W/H) based on Fit mode and focus point
fn crop_to_aspect(
    img: &DynamicImage,
    target_w: u32,
    target_h: u32,
    fit: Fit,
    focus: Option<(f32, f32)>,
) -> DynamicImage {
    let (src_w, src_h) = img.dimensions();
    let src_aspect = src_w as f64 / src_h as f64;
    let target_aspect = target_w as f64 / target_h as f64;

    match fit {
        Fit::Fill | Fit::CropCenter => {
            let focus = if fit == Fit::CropCenter {
                (0.5, 0.5)
            } else {
                focus.unwrap_or((0.5, 0.5))
            };

            let (crop_w, crop_h, crop_x, crop_y) = if src_aspect > target_aspect {
                // Source is wider than target -> crop horizontally
                let crop_w = (src_h as f64 * target_aspect).round() as u32;
                let crop_h = src_h;
                let max_x = src_w.saturating_sub(crop_w);
                let center_x = (focus.0 * src_w as f32) as i32;
                let x = (center_x - (crop_w as i32 / 2)).clamp(0, max_x as i32) as u32;
                (crop_w, crop_h, x, 0)
            } else {
                // Source is taller than target -> crop vertically
                let crop_w = src_w;
                let crop_h = (src_w as f64 / target_aspect).round() as u32;
                let max_y = src_h.saturating_sub(crop_h);
                let center_y = (focus.1 * src_h as f32) as i32;
                let y = (center_y - (crop_h as i32 / 2)).clamp(0, max_y as i32) as u32;
                (crop_w, crop_h, 0, y)
            };

            img.crop_imm(crop_x, crop_y, crop_w.min(src_w), crop_h.min(src_h))
        }
        Fit::CropRect { x, y, w, h } => {
            let crop_x = (x * src_w as f32) as u32;
            let crop_y = (y * src_h as f32) as u32;
            let crop_w = (w * src_w as f32) as u32;
            let crop_h = (h * src_h as f32) as u32;
            img.crop_imm(
                crop_x.min(src_w),
                crop_y.min(src_h),
                crop_w.min(src_w.saturating_sub(crop_x)),
                crop_h.min(src_h.saturating_sub(crop_y)),
            )
        }
        Fit::Fit => {
            // Letterbox/Fit: do not crop source, resize directly preserving aspect ratio
            img.clone()
        }
    }
}

/// Perform fast SIMD Lanczos3 resize using fast_image_resize
fn resize_simd(img: &DynamicImage, target_w: u32, target_h: u32) -> Result<DynamicImage> {
    let rgba = img.to_rgba8();
    let (src_w, src_h) = rgba.dimensions();

    let src_image = fr::images::Image::from_vec_u8(
        src_w,
        src_h,
        rgba.into_raw(),
        fr::PixelType::U8x4,
    ).map_err(|e| CropError::Image(image::ImageError::Decoding(image::error::DecodingError::new(
        image::error::ImageFormatHint::Unknown,
        e.to_string(),
    ))))?;

    let mut dst_image = fr::images::Image::new(target_w, target_h, fr::PixelType::U8x4);
    let mut resizer = fr::Resizer::new();
    resizer.resize(&src_image, &mut dst_image, &fr::ResizeOptions::default())
        .map_err(|e| CropError::Image(image::ImageError::Decoding(image::error::DecodingError::new(
            image::error::ImageFormatHint::Unknown,
            e.to_string(),
        ))))?;

    let buffer = dst_image.into_vec();
    let img_buffer = image::RgbaImage::from_raw(target_w, target_h, buffer)
        .ok_or_else(|| CropError::InvalidDimensions(target_w, target_h))?;

    Ok(DynamicImage::ImageRgba8(img_buffer))
}

fn encode_image(img: &DynamicImage, format: OutFormat) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);

    match format {
        OutFormat::Png => {
            img.write_to(&mut cursor, ImageFormat::Png)?;
        }
        OutFormat::WebpLossy(_) => {
            // WebP encoding via image crate webp feature
            img.write_to(&mut cursor, ImageFormat::WebP)?;
        }
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_image(w: u32, h: u32) -> DynamicImage {
        let mut img = image::RgbaImage::new(w, h);
        for pixel in img.pixels_mut() {
            *pixel = image::Rgba([100, 150, 200, 255]);
        }
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_crop_portrait_to_landscape() {
        let dir = tempdir().unwrap();
        let engine = CropEngine::new(dir.path().join("derived"));

        let src = create_test_image(1080, 1920); // Portrait (9:16)
        let src_path = dir.path().join("portrait.png");
        src.save(&src_path).unwrap();

        let hash = Hash::compute(b"portrait_img");
        let spec = DerivativeSpec {
            target: (1920, 1080), // Landscape (16:9)
            fit: Fit::Fill,
            focus: Some((0.5, 0.5)),
            format: OutFormat::Png,
        };

        let result_path = engine.get_or_render(&hash, &src_path, &spec).unwrap();
        assert!(result_path.exists());

        let out = image::open(&result_path).unwrap();
        assert_eq!(out.width(), 1920);
        assert_eq!(out.height(), 1080);
    }

    #[test]
    fn test_crop_landscape_to_portrait() {
        let dir = tempdir().unwrap();
        let engine = CropEngine::new(dir.path().join("derived"));

        let src = create_test_image(2560, 1440); // Landscape
        let src_path = dir.path().join("landscape.png");
        src.save(&src_path).unwrap();

        let hash = Hash::compute(b"landscape_img");
        let spec = DerivativeSpec {
            target: (1080, 2400), // Phone screen (portrait)
            fit: Fit::Fill,
            focus: Some((0.5, 0.2)), // Top-weighted focus
            format: OutFormat::Png,
        };

        let result_path = engine.get_or_render(&hash, &src_path, &spec).unwrap();
        assert!(result_path.exists());

        let out = image::open(&result_path).unwrap();
        assert_eq!(out.width(), 1080);
        assert_eq!(out.height(), 2400);
    }
}
```

---

### File: `core/src/lib.rs`
- **Summary:** structs: SyncWallEngine; enums: EngineError
- **Path:** `core/src/lib.rs`
- **Estimated Tokens:** 1,650
- **mtime:** 1792399374.013

```rust
pub mod crop;
pub mod model;
pub mod net;
pub mod store;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;

use crate::crop::CropEngine;
use crate::model::*;
use crate::net::{DeviceIdentity, ServerState};
use crate::store::Store;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Store error: {0}")]
    Store(#[from] store::StoreError),
    #[error("Crop error: {0}")]
    Crop(#[from] crop::CropError),
    #[error("Network error: {0}")]
    Net(#[from] net::NetError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, EngineError>;

/// Deep engine facade unifying storage, cropping, synchronization, and local identity.
#[derive(Clone)]
pub struct SyncWallEngine {
    store: Arc<Store>,
    crop: Arc<CropEngine>,
    identity: DeviceIdentity,
    active_token: Arc<parking_lot::Mutex<Option<(String, std::time::Instant)>>>,
}

impl SyncWallEngine {
    pub fn open<P: AsRef<Path>>(base_dir: P, device_name: &str) -> Result<Self> {
        let base_dir = base_dir.as_ref().to_path_buf();
        fs::create_dir_all(&base_dir)?;

        let id_dir = base_dir.join("identity");
        let identity = match DeviceIdentity::load_from_dir(&id_dir)? {
            Some(id) => id,
            None => {
                let id = DeviceIdentity::generate(device_name)?;
                id.save_to_dir(&id_dir)?;
                id
            }
        };

        let store = Arc::new(Store::open(&base_dir, &identity.fingerprint)?);
        let crop = Arc::new(CropEngine::new(base_dir.join("derived")));
        let active_token = Arc::new(parking_lot::Mutex::new(None));

        Ok(Self {
            store,
            crop,
            identity,
            active_token,
        })
    }

    pub fn device_id(&self) -> &str {
        &self.identity.fingerprint
    }

    pub fn store(&self) -> &Arc<Store> {
        &self.store
    }

    pub fn crop(&self) -> &Arc<CropEngine> {
        &self.crop
    }

    pub fn identity(&self) -> &DeviceIdentity {
        &self.identity
    }

    pub fn server_state(&self) -> ServerState {
        ServerState {
            store: self.store.clone(),
            crop: self.crop.clone(),
            identity: self.identity.clone(),
            active_pairing_token: self.active_token.clone(),
        }
    }

    /// Ingest a local image file into CAS and catalog it in SQLite
    pub fn ingest_file<P: AsRef<Path>>(&self, path: P, tags: Vec<String>) -> Result<ImageRecord> {
        let path = path.as_ref();
        let bytes = fs::read(path)?;
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image.png");
        let record = self.store.ingest_bytes(&bytes, filename, tags)?;
        Ok(record)
    }

    pub fn ingest_bytes(&self, bytes: &[u8], name: &str, tags: Vec<String>) -> Result<ImageRecord> {
        let record = self.store.ingest_bytes(bytes, name, tags)?;
        Ok(record)
    }

    pub fn get_image(&self, hash: &Hash) -> Result<Option<ImageRecord>> {
        let res = self.store.get_image(hash)?;
        Ok(res)
    }

    pub fn list_images(&self) -> Result<Vec<ImageRecord>> {
        let res = self.store.list_images()?;
        Ok(res)
    }

    /// Assign wallpaper to local screen
    pub fn set_wallpaper(
        &self,
        screen_name: &str,
        image_hash: Hash,
        fit: Fit,
        focus: Option<(f32, f32)>,
    ) -> Result<AssignmentRecord> {
        let assign = self.store.set_assignment(
            self.device_id(),
            screen_name,
            image_hash,
            fit,
            focus,
        )?;
        Ok(assign)
    }

    /// Render or fetch cached derivative for an assignment
    pub fn render_wallpaper_for_screen(
        &self,
        screen_name: &str,
        target_dim: (u32, u32),
        format: OutFormat,
    ) -> Result<PathBuf> {
        let assign = self.store.get_assignment(self.device_id(), screen_name)?
            .ok_or_else(|| EngineError::NotFound(format!("No assignment for screen {}", screen_name)))?;

        let source_path = self.store.blob_path(&assign.image_hash);
        if !source_path.exists() {
            return Err(EngineError::NotFound(format!("Source image {} missing from CAS", assign.image_hash)));
        }

        let spec = DerivativeSpec {
            target: target_dim,
            fit: assign.fit,
            focus: assign.focus,
            format,
        };

        let path = self.crop.get_or_render(&assign.image_hash, &source_path, &spec)?;
        Ok(path)
    }

    /// Generate a 6-digit one-time pairing token expiring in 120 seconds
    pub fn generate_pairing_token(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let token: u32 = rng.gen_range(100_000..1_000_000);
        let token_str = token.to_string();
        *self.active_token.lock() = Some((token_str.clone(), std::time::Instant::now()));
        token_str
    }

    /// Run sync against a peer URL
    pub async fn sync_with_peer(&self, peer_url: &str) -> Result<ApplyReport> {
        let client = reqwest::Client::builder().build()?;

        // Step 1: Pull peer changes
        let (last_sent_seq, _) = self.store.get_peer_cursor(peer_url)?;
        let changes_url = format!("{}/v1/changes?since={}&limit=500", peer_url, last_sent_seq);
        let resp = client.get(&changes_url).send().await?;
        let batch: SyncBatch = resp.json().await?;

        let new_seq = batch.max_seq;
        let report = self.store.apply_batch(&batch)?;
        self.store.set_peer_cursor(peer_url, new_seq, None)?;

        // Step 2: Push our changes to peer
        let local_batch = self.store.changes_since(0, 500)?;
        let push_url = format!("{}/v1/changes", peer_url);
        let _ = client.post(&push_url).json(&local_batch).send().await?;

        // Step 3: Fetch missing blobs for our device's assignments
        let my_assignments = self.store.list_assignments()?;
        for assign in my_assignments {
            let blob_path = self.store.blob_path(&assign.image_hash);
            if !blob_path.exists() {
                let _ = net::download_blob_resumable(
                    &client,
                    peer_url,
                    &assign.image_hash,
                    None,
                    &blob_path,
                ).await;
            }
        }

        Ok(report)
    }
}
```

---

### File: `core/src/model.rs`
- **Summary:** structs: Hash, Hlc, DerivativeSpec; enums: Fit, OutFormat, DeviceKind
- **Path:** `core/src/model.rs`
- **Estimated Tokens:** 2,694
- **mtime:** 1792399073.631

```rust
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const PIPELINE_VERSION: u32 = 1;

/// BLAKE3 32-byte cryptographic hash with Hex serialization
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
        if slice.len() != 32 {
            return Err("Invalid hash length: expected 32 bytes".to_string());
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(slice);
        Ok(Self(arr))
    }

    pub fn compute(data: &[u8]) -> Self {
        let hash = blake3::hash(data);
        Self(*hash.as_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({})", self.to_hex())
    }
}

impl FromStr for Hash {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).map_err(|e| e.to_string())?;
        Self::from_slice(&bytes)
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Hash::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Hybrid Logical Clock (HLC)
/// Formatted as `<hex_physical_ms>:<hex_counter>:<device_id>`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Hlc {
    pub physical_ms: u64,
    pub counter: u32,
    pub device_id: String,
}

impl Hlc {
    pub fn new(physical_ms: u64, counter: u32, device_id: impl Into<String>) -> Self {
        Self {
            physical_ms,
            counter,
            device_id: device_id.into(),
        }
    }

    pub fn zero(device_id: impl Into<String>) -> Self {
        Self {
            physical_ms: 0,
            counter: 0,
            device_id: device_id.into(),
        }
    }

    /// Update HLC on local event
    pub fn tick_local(&mut self, now_ms: u64) {
        if now_ms > self.physical_ms {
            self.physical_ms = now_ms;
            self.counter = 0;
        } else {
            self.counter += 1;
        }
    }

    /// Update HLC when receiving a message from a remote peer.
    /// Rejects or clamps remote clocks > 24 hours ahead of local time to prevent permanent hijacking.
    pub fn tick_remote(&mut self, remote: &Hlc, now_ms: u64) {
        const MAX_SKEW_MS: u64 = 24 * 60 * 60 * 1000;
        let clamped_remote_physical = if remote.physical_ms > now_ms + MAX_SKEW_MS {
            now_ms + MAX_SKEW_MS
        } else {
            remote.physical_ms
        };

        if now_ms > self.physical_ms && now_ms > clamped_remote_physical {
            self.physical_ms = now_ms;
            self.counter = 0;
        } else if self.physical_ms == clamped_remote_physical {
            self.counter = std::cmp::max(self.counter, remote.counter) + 1;
        } else if self.physical_ms > clamped_remote_physical {
            self.counter += 1;
        } else {
            self.physical_ms = clamped_remote_physical;
            self.counter = remote.counter + 1;
        }
    }

    pub fn to_string_repr(&self) -> String {
        format!("{:016x}:{:08x}:{}", self.physical_ms, self.counter, self.device_id)
    }
}

impl fmt::Display for Hlc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}

impl fmt::Debug for Hlc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hlc({})", self.to_string_repr())
    }
}

impl Ord for Hlc {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.physical_ms.cmp(&other.physical_ms) {
            Ordering::Equal => match self.counter.cmp(&other.counter) {
                Ordering::Equal => self.device_id.cmp(&other.device_id),
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl PartialOrd for Hlc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hlc {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(3, ':').collect();
        if parts.len() != 3 {
            return Err("Invalid HLC format, expected physical:counter:device_id".to_string());
        }
        let physical_ms = u64::from_str_radix(parts[0], 16).map_err(|e| e.to_string())?;
        let counter = u32::from_str_radix(parts[1], 16).map_err(|e| e.to_string())?;
        let device_id = parts[2].to_string();
        Ok(Self {
            physical_ms,
            counter,
            device_id,
        })
    }
}

impl Serialize for Hlc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string_repr())
    }
}

impl<'de> Deserialize<'de> for Hlc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Hlc::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Fit {
    Fill,
    Fit,
    CropCenter,
    CropRect { x: f32, y: f32, w: f32, h: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutFormat {
    Png,
    WebpLossy(u8),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivativeSpec {
    pub target: (u32, u32),
    pub fit: Fit,
    pub focus: Option<(f32, f32)>,
    pub format: OutFormat,
}

impl DerivativeSpec {
    pub fn cache_key(&self, source_hash: &Hash) -> String {
        let canonical_spec = serde_json::to_string(self).unwrap_or_default();
        let mut hasher = blake3::Hasher::new();
        hasher.update(source_hash.as_bytes());
        hasher.update(canonical_spec.as_bytes());
        hasher.update(&PIPELINE_VERSION.to_le_bytes());
        let digest = hasher.finalize();
        let ext = match self.format {
            OutFormat::Png => "png",
            OutFormat::WebpLossy(_) => "webp",
        };
        format!("{}.{}", hex::encode(&digest.as_bytes()[..16]), ext)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceKind {
    Linux,
    Android,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Screen {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageRecord {
    pub hash: Hash,
    pub width: u32,
    pub height: u32,
    pub mime: String,
    pub added_at: u64,
    pub tags: Vec<String>,
    pub source_name: String,
    pub hlc: Hlc,
    pub deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceRecord {
    pub id: String, // SHA-256 fingerprint of device certificate
    pub name: String,
    pub kind: DeviceKind,
    pub screens: Vec<Screen>,
    pub hlc: Hlc,
    pub deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignmentRecord {
    pub device_id: String,
    pub screen_name: String,
    pub image_hash: Hash,
    pub fit: Fit,
    pub focus: Option<(f32, f32)>,
    pub hlc: Hlc,
    pub deleted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotationMode {
    Interval(u32), // interval in seconds
    OnLogin,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderMode {
    Sequential,
    Shuffle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionRecord {
    pub id: String,
    pub name: String,
    pub rotation: RotationMode,
    pub order: OrderMode,
    pub hlc: Hlc,
    pub deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionItemRecord {
    pub collection_id: String,
    pub image_hash: Hash,
    pub position: u32,
    pub hlc: Hlc,
    pub deleted: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncBatch {
    pub images: Vec<ImageRecord>,
    pub devices: Vec<DeviceRecord>,
    pub assignments: Vec<AssignmentRecord>,
    pub collections: Vec<CollectionRecord>,
    pub collection_items: Vec<CollectionItemRecord>,
    pub max_seq: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplyReport {
    pub applied_images: usize,
    pub applied_devices: usize,
    pub applied_assignments: Vec<AssignmentRecord>,
    pub applied_collections: usize,
    pub applied_collection_items: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_roundtrip() {
        let data = b"sync_wall_wallpaper_bytes";
        let h1 = Hash::compute(data);
        let s = h1.to_string();
        let h2 = Hash::from_str(&s).expect("from_str");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hlc_ordering_and_clamping() {
        let mut hlc1 = Hlc::new(1000, 0, "device-a");
        let mut hlc2 = Hlc::new(1000, 0, "device-b");

        // Device ID tie-break
        assert!(hlc1 < hlc2);

        // Local tick advances physical
        hlc1.tick_local(1050);
        assert_eq!(hlc1.physical_ms, 1050);
        assert_eq!(hlc1.counter, 0);

        // Local tick same physical increments counter
        hlc1.tick_local(1050);
        assert_eq!(hlc1.counter, 1);

        // Remote tick clamped if future skewed > 24h
        let extreme_future = Hlc::new(1050 + 48 * 3600 * 1000, 5, "malicious");
        hlc2.tick_remote(&extreme_future, 1050);
        assert!(hlc2.physical_ms <= 1050 + 24 * 3600 * 1000);
    }

    #[test]
    fn test_derivative_spec_cache_key() {
        let h = Hash::compute(b"image_content");
        let spec = DerivativeSpec {
            target: (1920, 1080),
            fit: Fit::Fill,
            focus: Some((0.5, 0.5)),
            format: OutFormat::Png,
        };
        let key1 = spec.cache_key(&h);
        let key2 = spec.cache_key(&h);
        assert_eq!(key1, key2);
        assert!(key1.ends_with(".png"));
    }
}
```

---

### File: `core/src/net.rs`
- **Summary:** structs: DeviceIdentity, ServerState, HelloResponse; enums: NetError; 9 functions
- **Path:** `core/src/net.rs`
- **Estimated Tokens:** 3,062
- **mtime:** 1792399468.24

```rust
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use axum::{
    body::Body,
    extract::{Path as AxPath, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use rcgen::{CertificateParams, KeyPair};
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
use thiserror::Error;

use crate::crop::CropEngine;
use crate::model::*;
use crate::store::Store;

#[derive(Error, Debug)]
pub enum NetError {
    #[error("TLS/Crypto error: {0}")]
    Crypto(String),
    #[error("Network error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Store error: {0}")]
    Store(#[from] crate::store::StoreError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Not found")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, NetError>;

/// Self-signed TLS certificate and private key with SHA-256 fingerprint
#[derive(Clone)]
pub struct DeviceIdentity {
    pub cert_der: Vec<u8>,
    pub cert_pem: String,
    pub key_pem: String,
    pub fingerprint: String, // SHA-256 hex string (Device ID)
}

impl DeviceIdentity {
    pub fn generate(device_name: &str) -> Result<Self> {
        let key_pair = KeyPair::generate().map_err(|e| NetError::Crypto(e.to_string()))?;
        let mut params = CertificateParams::new(vec![device_name.to_string(), "localhost".to_string()])
            .map_err(|e| NetError::Crypto(e.to_string()))?;
        params.distinguished_name.push(rcgen::DnType::CommonName, device_name);

        let cert = params.self_signed(&key_pair).map_err(|e| NetError::Crypto(e.to_string()))?;
        let cert_der = cert.der().to_vec();
        let cert_pem = cert.pem();
        let key_pem = key_pair.serialize_pem();

        let digest = blake3::hash(&cert_der);
        let fingerprint = hex::encode(digest.as_bytes());

        Ok(Self {
            cert_der,
            cert_pem,
            key_pem,
            fingerprint,
        })
    }

    pub fn save_to_dir<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        let dir = dir.as_ref();
        fs::create_dir_all(dir)?;
        fs::write(dir.join("cert.pem"), &self.cert_pem)?;
        fs::write(dir.join("key.pem"), &self.key_pem)?;
        fs::write(dir.join("fingerprint.txt"), &self.fingerprint)?;
        Ok(())
    }

    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Option<Self>> {
        let dir = dir.as_ref();
        let cert_path = dir.join("cert.pem");
        let key_path = dir.join("key.pem");
        let fp_path = dir.join("fingerprint.txt");

        if !cert_path.exists() || !key_path.exists() || !fp_path.exists() {
            return Ok(None);
        }

        let cert_pem = fs::read_to_string(cert_path)?;
        let key_pem = fs::read_to_string(key_path)?;
        let fingerprint = fs::read_to_string(fp_path)?.trim().to_string();

        let cert_der = pem::parse(&cert_pem)
            .map_err(|e| NetError::Crypto(e.to_string()))?
            .contents()
            .to_vec();

        Ok(Some(Self {
            cert_der,
            cert_pem,
            key_pem,
            fingerprint,
        }))
    }
}

#[derive(Clone)]
pub struct ServerState {
    pub store: Arc<Store>,
    pub crop: Arc<CropEngine>,
    pub identity: DeviceIdentity,
    pub active_pairing_token: Arc<parking_lot::Mutex<Option<(String, std::time::Instant)>>>,
}

#[derive(Serialize, Deserialize)]
pub struct HelloResponse {
    pub device_id: String,
    pub name: String,
    pub schema_version: u32,
    pub current_seq: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PairRequest {
    pub token: String,
    pub client_fingerprint: String,
    pub device_name: String,
    pub kind: DeviceKind,
    pub screens: Vec<Screen>,
}

#[derive(Serialize, Deserialize)]
pub struct PairResponse {
    pub success: bool,
    pub server_name: String,
    pub server_fingerprint: String,
}

#[derive(Deserialize)]
pub struct ChangesQuery {
    pub since: Option<u64>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
pub struct BlobQuery {
    pub target_w: Option<u32>,
    pub target_h: Option<u32>,
    pub fit: Option<String>,
}

pub fn create_router(state: ServerState) -> Router {
    Router::new()
        .route("/v1/hello", get(handle_hello))
        .route("/v1/pair", post(handle_pair))
        .route("/v1/changes", get(handle_get_changes).post(handle_post_changes))
        .route("/v1/blobs/:hash", get(handle_get_blob))
        .with_state(state)
}

async fn handle_hello(State(state): State<ServerState>) -> Json<HelloResponse> {
    let (seq, _) = state.store.get_peer_cursor("self").unwrap_or((0, None));
    Json(HelloResponse {
        device_id: state.identity.fingerprint.clone(),
        name: state.store.device_id().to_string(),
        schema_version: 1,
        current_seq: seq,
    })
}

async fn handle_pair(
    State(state): State<ServerState>,
    Json(req): Json<PairRequest>,
) -> std::result::Result<Json<PairResponse>, StatusCode> {
    let mut token_lock = state.active_pairing_token.lock();
    let is_valid = if let Some((active_tok, created_at)) = token_lock.as_ref() {
        if created_at.elapsed().as_secs() > 120 {
            false
        } else {
            bool::from(active_tok.as_bytes().ct_eq(req.token.as_bytes()))
        }
    } else {
        false
    };

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Invalidate token immediately upon single use
    *token_lock = None;

    // Enroll peer device
    let dev = DeviceRecord {
        id: req.client_fingerprint.clone(),
        name: req.device_name,
        kind: req.kind,
        screens: req.screens,
        hlc: state.store.tick_local(),
        deleted: false,
    };

    state.store.upsert_device(&dev).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PairResponse {
        success: true,
        server_name: state.store.device_id().to_string(),
        server_fingerprint: state.identity.fingerprint.clone(),
    }))
}

async fn handle_get_changes(
    State(state): State<ServerState>,
    Query(q): Query<ChangesQuery>,
) -> std::result::Result<Json<SyncBatch>, StatusCode> {
    let since = q.since.unwrap_or(0);
    let limit = q.limit.unwrap_or(500);
    let batch = state.store.changes_since(since, limit)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(batch))
}

async fn handle_post_changes(
    State(state): State<ServerState>,
    Json(batch): Json<SyncBatch>,
) -> std::result::Result<Json<ApplyReport>, StatusCode> {
    let report = state.store.apply_batch(&batch)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(report))
}

async fn handle_get_blob(
    State(state): State<ServerState>,
    AxPath(hash_str): AxPath<String>,
    Query(q): Query<BlobQuery>,
    headers: HeaderMap,
) -> std::result::Result<Response, StatusCode> {
    let hash: Hash = hash_str.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
    let original_path = state.store.blob_path(&hash);

    if !original_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let final_path = if let (Some(w), Some(h)) = (q.target_w, q.target_h) {
        let fit = match q.fit.as_deref() {
            Some("fit") => Fit::Fit,
            Some("center") => Fit::CropCenter,
            _ => Fit::Fill,
        };
        let spec = DerivativeSpec {
            target: (w, h),
            fit,
            focus: Some((0.5, 0.5)),
            format: OutFormat::WebpLossy(90),
        };
        state.crop.get_or_render(&hash, &original_path, &spec)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        original_path
    };

    let metadata = fs::metadata(&final_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let total_size = metadata.len();

    // Check for Range header
    if let Some(range_header) = headers.get(header::RANGE) {
        if let Ok(range_str) = range_header.to_str() {
            if let Some(range) = parse_range(range_str, total_size) {
                let bytes = read_file_range(&final_path, range.0, range.1)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                let resp = Response::builder()
                    .status(StatusCode::PARTIAL_CONTENT)
                    .header(header::CONTENT_TYPE, "application/octet-stream")
                    .header(header::ACCEPT_RANGES, "bytes")
                    .header(header::CONTENT_RANGE, format!("bytes {}-{}/{}", range.0, range.1, total_size))
                    .header("X-Blake3", hash.to_hex())
                    .body(Body::from(bytes))
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                return Ok(resp);
            }
        }
    }

    let bytes = fs::read(&final_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let resp = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::CONTENT_LENGTH, total_size.to_string())
        .header("X-Blake3", hash.to_hex())
        .body(Body::from(bytes))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(resp)
}

fn parse_range(range_str: &str, total_size: u64) -> Option<(u64, u64)> {
    if !range_str.starts_with("bytes=") {
        return None;
    }
    let s = &range_str[6..];
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return None;
    }
    let start = parts[0].parse::<u64>().ok()?;
    let end = if parts[1].is_empty() {
        total_size.saturating_sub(1)
    } else {
        parts[1].parse::<u64>().ok()?.min(total_size.saturating_sub(1))
    };
    if start <= end && start < total_size {
        Some((start, end))
    } else {
        None
    }
}

fn read_file_range(path: &Path, start: u64, end: u64) -> std::io::Result<Vec<u8>> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = File::open(path)?;
    f.seek(SeekFrom::Start(start))?;
    let len = (end - start + 1) as usize;
    let mut buf = vec![0u8; len];
    f.read_exact(&mut buf)?;
    Ok(buf)
}

/// Download a blob from a peer with resumable Range and atomic .part rename
pub async fn download_blob_resumable(
    client: &reqwest::Client,
    peer_url: &str,
    hash: &Hash,
    target_dim: Option<(u32, u32)>,
    dest_path: &Path,
) -> Result<()> {
    if dest_path.exists() {
        return Ok(());
    }

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let part_path = dest_path.with_extension("part");
    let initial_offset = if part_path.exists() {
        fs::metadata(&part_path).map(|m| m.len()).unwrap_or(0)
    } else {
        0
    };

    let mut url = format!("{}/v1/blobs/{}", peer_url, hash.to_hex());
    if let Some((w, h)) = target_dim {
        url = format!("{}?target_w={}&target_h={}&fit=fill", url, w, h);
    }

    let mut req = client.get(&url);
    if initial_offset > 0 {
        req = req.header(header::RANGE, format!("bytes={}-", initial_offset));
    }

    let resp = req.send().await?;
    let status = resp.status();
    if !status.is_success() && status != StatusCode::PARTIAL_CONTENT {
        return Err(NetError::NotFound);
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&part_path)?;

    let bytes = resp.bytes().await?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    drop(file);

    // Atomic rename
    fs::rename(part_path, dest_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_identity_generation() {
        let id = DeviceIdentity::generate("test-device").unwrap();
        assert!(!id.fingerprint.is_empty());
        assert!(!id.cert_pem.is_empty());
        assert!(!id.key_pem.is_empty());
    }

    #[test]
    fn test_range_parser() {
        assert_eq!(parse_range("bytes=0-499", 1000), Some((0, 499)));
        assert_eq!(parse_range("bytes=500-", 1000), Some((500, 999)));
        assert_eq!(parse_range("bytes=1500-", 1000), None);
    }
}
```

---

### File: `core/src/store.rs`
- **Summary:** structs: Store; enums: StoreError
- **Path:** `core/src/store.rs`
- **Estimated Tokens:** 7,719
- **mtime:** 1792399123.706

```rust
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection, OptionalExtension};
use thiserror::Error;

use crate::model::*;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, StoreError>;

pub struct Store {
    conn: Arc<Mutex<Connection>>,
    base_dir: PathBuf,
    objects_dir: PathBuf,
    derived_dir: PathBuf,
    device_id: String,
    local_hlc: Mutex<Hlc>,
}

impl Store {
    pub fn open<P: AsRef<Path>>(base_dir: P, device_id: &str) -> Result<Self> {
        let base_dir = base_dir.as_ref().to_path_buf();
        let objects_dir = base_dir.join("objects");
        let derived_dir = base_dir.join("derived");
        fs::create_dir_all(&objects_dir)?;
        fs::create_dir_all(&derived_dir)?;

        let db_path = base_dir.join("sync_wall.db");
        let conn = Connection::open(&db_path)?;
        
        // Performance pragmas
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;"
        )?;

        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
            base_dir,
            objects_dir,
            derived_dir,
            device_id: device_id.to_string(),
            local_hlc: Mutex::new(Hlc::zero(device_id)),
        };

        store.init_schema()?;
        store.load_or_init_hlc()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS meta (
                key TEXT PRIMARY KEY,
                val TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS internal_seq (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                val INTEGER NOT NULL
            );
            INSERT OR IGNORE INTO internal_seq (id, val) VALUES (1, 0);

            CREATE TABLE IF NOT EXISTS peer_state (
                peer_id TEXT PRIMARY KEY,
                last_sent_seq INTEGER NOT NULL DEFAULT 0,
                last_recv_hlc TEXT
            );

            CREATE TABLE IF NOT EXISTS images (
                hash TEXT PRIMARY KEY,
                width INTEGER NOT NULL,
                height INTEGER NOT NULL,
                mime TEXT NOT NULL,
                added_at INTEGER NOT NULL,
                tags TEXT NOT NULL,
                source_name TEXT NOT NULL,
                hlc TEXT NOT NULL,
                deleted INTEGER NOT NULL DEFAULT 0,
                seq INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                kind TEXT NOT NULL,
                screens TEXT NOT NULL,
                hlc TEXT NOT NULL,
                deleted INTEGER NOT NULL DEFAULT 0,
                seq INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS assignments (
                device_id TEXT NOT NULL,
                screen_name TEXT NOT NULL,
                image_hash TEXT NOT NULL,
                fit TEXT NOT NULL,
                focus TEXT,
                hlc TEXT NOT NULL,
                deleted INTEGER NOT NULL DEFAULT 0,
                seq INTEGER NOT NULL,
                PRIMARY KEY (device_id, screen_name)
            );

            CREATE TABLE IF NOT EXISTS collections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                rotation TEXT NOT NULL,
                order_mode TEXT NOT NULL,
                hlc TEXT NOT NULL,
                deleted INTEGER NOT NULL DEFAULT 0,
                seq INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS collection_items (
                collection_id TEXT NOT NULL,
                image_hash TEXT NOT NULL,
                position INTEGER NOT NULL,
                hlc TEXT NOT NULL,
                deleted INTEGER NOT NULL DEFAULT 0,
                seq INTEGER NOT NULL,
                PRIMARY KEY (collection_id, image_hash)
            );"
        )?;

        // Setup sequence triggers on all tables
        let tables = ["images", "devices", "assignments", "collections", "collection_items"];
        for table in tables {
            let trigger_insert = format!(
                "CREATE TRIGGER IF NOT EXISTS trg_{table}_insert
                 AFTER INSERT ON {table}
                 FOR EACH ROW
                 BEGIN
                     UPDATE internal_seq SET val = val + 1 WHERE id = 1;
                     UPDATE {table} SET seq = (SELECT val FROM internal_seq WHERE id = 1) WHERE rowid = NEW.rowid;
                 END;"
            );
            let trigger_update = format!(
                "CREATE TRIGGER IF NOT EXISTS trg_{table}_update
                 AFTER UPDATE OF hlc, deleted ON {table}
                 FOR EACH ROW
                 BEGIN
                     UPDATE internal_seq SET val = val + 1 WHERE id = 1;
                     UPDATE {table} SET seq = (SELECT val FROM internal_seq WHERE id = 1) WHERE rowid = NEW.rowid;
                 END;"
            );
            conn.execute_batch(&trigger_insert)?;
            conn.execute_batch(&trigger_update)?;
        }

        Ok(())
    }

    fn load_or_init_hlc(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let val: Option<String> = conn
            .query_row("SELECT val FROM meta WHERE key = 'local_hlc'", [], |row| row.get(0))
            .optional()?;

        let mut hlc = self.local_hlc.lock().unwrap();
        if let Some(s) = val {
            if let Ok(loaded) = s.parse::<Hlc>() {
                *hlc = loaded;
                return Ok(());
            }
        }
        let now_ms = chrono::Utc::now().timestamp_millis().max(0) as u64;
        hlc.tick_local(now_ms);
        conn.execute(
            "INSERT OR REPLACE INTO meta (key, val) VALUES ('local_hlc', ?)",
            params![hlc.to_string_repr()],
        )?;
        Ok(())
    }

    pub fn tick_local(&self) -> Hlc {
        let now_ms = chrono::Utc::now().timestamp_millis().max(0) as u64;
        let mut hlc = self.local_hlc.lock().unwrap();
        hlc.tick_local(now_ms);
        let copy = hlc.clone();
        drop(hlc);

        let conn = self.conn.lock().unwrap();
        let _ = conn.execute(
            "INSERT OR REPLACE INTO meta (key, val) VALUES ('local_hlc', ?)",
            params![copy.to_string_repr()],
        );
        copy
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    pub fn objects_dir(&self) -> &Path {
        &self.objects_dir
    }

    pub fn derived_dir(&self) -> &Path {
        &self.derived_dir
    }

    pub fn blob_path(&self, hash: &Hash) -> PathBuf {
        let hex = hash.to_hex();
        let prefix = &hex[..2];
        self.objects_dir.join(prefix).join(&hex)
    }

    /// Ingest an image into CAS and record it in SQLite.
    pub fn ingest_bytes(
        &self,
        bytes: &[u8],
        source_name: &str,
        tags: Vec<String>,
    ) -> Result<ImageRecord> {
        let hash = Hash::compute(bytes);
        let dest = self.blob_path(&hash);

        // Atomic write to CAS
        if !dest.exists() {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            let tmp_path = dest.with_extension("tmp");
            let mut file = File::create(&tmp_path)?;
            file.write_all(bytes)?;
            file.sync_all()?;
            fs::rename(tmp_path, &dest)?;
        }

        // Determine image dimensions
        let img = image::load_from_memory(bytes).map_err(|e| StoreError::InvalidData(e.to_string()))?;
        let (width, height) = (img.width(), img.height());
        let mime = match image::guess_format(bytes) {
            Ok(image::ImageFormat::Png) => "image/png",
            Ok(image::ImageFormat::Jpeg) => "image/jpeg",
            Ok(image::ImageFormat::WebP) => "image/webp",
            _ => "application/octet-stream",
        }.to_string();

        let hlc = self.tick_local();
        let now_sec = chrono::Utc::now().timestamp().max(0) as u64;
        let tags_json = serde_json::to_string(&tags)?;

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO images (hash, width, height, mime, added_at, tags, source_name, hlc, deleted, seq)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 0, 0)
             ON CONFLICT(hash) DO UPDATE SET
                tags = excluded.tags,
                deleted = 0,
                hlc = excluded.hlc;",
            params![
                hash.to_hex(),
                width,
                height,
                mime,
                now_sec,
                tags_json,
                source_name,
                hlc.to_string_repr(),
            ],
        )?;

        Ok(ImageRecord {
            hash,
            width,
            height,
            mime,
            added_at: now_sec,
            tags,
            source_name: source_name.to_string(),
            hlc,
            deleted: false,
        })
    }

    pub fn get_image(&self, hash: &Hash) -> Result<Option<ImageRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT hash, width, height, mime, added_at, tags, source_name, hlc, deleted
             FROM images WHERE hash = ?1 AND deleted = 0",
        )?;
        let res = stmt.query_row(params![hash.to_hex()], |row| {
            let h_str: String = row.get(0)?;
            let tags_str: String = row.get(5)?;
            let hlc_str: String = row.get(7)?;
            Ok(ImageRecord {
                hash: h_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                width: row.get(1)?,
                height: row.get(2)?,
                mime: row.get(3)?,
                added_at: row.get(4)?,
                tags: serde_json::from_str(&tags_str).unwrap_or_default(),
                source_name: row.get(6)?,
                hlc: hlc_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                deleted: row.get::<_, i64>(8)? != 0,
            })
        }).optional()?;
        Ok(res)
    }

    pub fn list_images(&self) -> Result<Vec<ImageRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT hash, width, height, mime, added_at, tags, source_name, hlc, deleted
             FROM images WHERE deleted = 0 ORDER BY added_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            let h_str: String = row.get(0)?;
            let tags_str: String = row.get(5)?;
            let hlc_str: String = row.get(7)?;
            Ok(ImageRecord {
                hash: h_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                width: row.get(1)?,
                height: row.get(2)?,
                mime: row.get(3)?,
                added_at: row.get(4)?,
                tags: serde_json::from_str(&tags_str).unwrap_or_default(),
                source_name: row.get(6)?,
                hlc: hlc_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                deleted: row.get::<_, i64>(8)? != 0,
            })
        })?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r?);
        }
        Ok(res)
    }

    pub fn set_assignment(
        &self,
        device_id: &str,
        screen_name: &str,
        image_hash: Hash,
        fit: Fit,
        focus: Option<(f32, f32)>,
    ) -> Result<AssignmentRecord> {
        let hlc = self.tick_local();
        let fit_json = serde_json::to_string(&fit)?;
        let focus_json = focus.map(|f| serde_json::to_string(&f).unwrap_or_default());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO assignments (device_id, screen_name, image_hash, fit, focus, hlc, deleted, seq)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, 0)
             ON CONFLICT(device_id, screen_name) DO UPDATE SET
                image_hash = excluded.image_hash,
                fit = excluded.fit,
                focus = excluded.focus,
                hlc = excluded.hlc,
                deleted = 0;",
            params![
                device_id,
                screen_name,
                image_hash.to_hex(),
                fit_json,
                focus_json,
                hlc.to_string_repr(),
            ],
        )?;

        Ok(AssignmentRecord {
            device_id: device_id.to_string(),
            screen_name: screen_name.to_string(),
            image_hash,
            fit,
            focus,
            hlc,
            deleted: false,
        })
    }

    pub fn get_assignment(&self, device_id: &str, screen_name: &str) -> Result<Option<AssignmentRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT device_id, screen_name, image_hash, fit, focus, hlc, deleted
             FROM assignments WHERE device_id = ?1 AND screen_name = ?2 AND deleted = 0",
        )?;
        let res = stmt.query_row(params![device_id, screen_name], |row| {
            let h_str: String = row.get(2)?;
            let fit_str: String = row.get(3)?;
            let focus_str: Option<String> = row.get(4)?;
            let hlc_str: String = row.get(5)?;
            Ok(AssignmentRecord {
                device_id: row.get(0)?,
                screen_name: row.get(1)?,
                image_hash: h_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                fit: serde_json::from_str(&fit_str).unwrap_or(Fit::Fill),
                focus: focus_str.and_then(|s| serde_json::from_str(&s).ok()),
                hlc: hlc_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                deleted: row.get::<_, i64>(6)? != 0,
            })
        }).optional()?;
        Ok(res)
    }

    pub fn list_assignments(&self) -> Result<Vec<AssignmentRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT device_id, screen_name, image_hash, fit, focus, hlc, deleted
             FROM assignments WHERE deleted = 0",
        )?;
        let rows = stmt.query_map([], |row| {
            let h_str: String = row.get(2)?;
            let fit_str: String = row.get(3)?;
            let focus_str: Option<String> = row.get(4)?;
            let hlc_str: String = row.get(5)?;
            Ok(AssignmentRecord {
                device_id: row.get(0)?,
                screen_name: row.get(1)?,
                image_hash: h_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                fit: serde_json::from_str(&fit_str).unwrap_or(Fit::Fill),
                focus: focus_str.and_then(|s| serde_json::from_str(&s).ok()),
                hlc: hlc_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                deleted: row.get::<_, i64>(6)? != 0,
            })
        })?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r?);
        }
        Ok(res)
    }

    pub fn upsert_device(&self, dev: &DeviceRecord) -> Result<()> {
        let screens_json = serde_json::to_string(&dev.screens)?;
        let kind_str = match dev.kind {
            DeviceKind::Linux => "linux",
            DeviceKind::Android => "android",
        };
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO devices (id, name, kind, screens, hlc, deleted, seq)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                kind = excluded.kind,
                screens = excluded.screens,
                hlc = excluded.hlc,
                deleted = excluded.deleted;",
            params![
                dev.id,
                dev.name,
                kind_str,
                screens_json,
                dev.hlc.to_string_repr(),
                if dev.deleted { 1 } else { 0 },
            ],
        )?;
        Ok(())
    }

    pub fn list_devices(&self) -> Result<Vec<DeviceRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, kind, screens, hlc, deleted FROM devices WHERE deleted = 0",
        )?;
        let rows = stmt.query_map([], |row| {
            let kind_str: String = row.get(2)?;
            let screens_str: String = row.get(3)?;
            let hlc_str: String = row.get(4)?;
            Ok(DeviceRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                kind: if kind_str == "android" { DeviceKind::Android } else { DeviceKind::Linux },
                screens: serde_json::from_str(&screens_str).unwrap_or_default(),
                hlc: hlc_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                deleted: row.get::<_, i64>(5)? != 0,
            })
        })?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r?);
        }
        Ok(res)
    }

    /// Query delta batch containing all mutations since sequence `since_seq`
    pub fn changes_since(&self, since_seq: u64, limit: usize) -> Result<SyncBatch> {
        let conn = self.conn.lock().unwrap();
        let mut batch = SyncBatch::default();
        let mut max_seq = since_seq;

        // Images
        {
            let mut stmt = conn.prepare(
                "SELECT hash, width, height, mime, added_at, tags, source_name, hlc, deleted, seq
                 FROM images WHERE seq > ?1 ORDER BY seq ASC LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![since_seq, limit], |row| {
                let h_str: String = row.get(0)?;
                let tags_str: String = row.get(5)?;
                let hlc_str: String = row.get(7)?;
                let seq: u64 = row.get(9)?;
                Ok((
                    ImageRecord {
                        hash: h_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                        width: row.get(1)?,
                        height: row.get(2)?,
                        mime: row.get(3)?,
                        added_at: row.get(4)?,
                        tags: serde_json::from_str(&tags_str).unwrap_or_default(),
                        source_name: row.get(6)?,
                        hlc: hlc_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                        deleted: row.get::<_, i64>(8)? != 0,
                    },
                    seq,
                ))
            })?;
            for r in rows {
                let (record, seq) = r?;
                if seq > max_seq {
                    max_seq = seq;
                }
                batch.images.push(record);
            }
        }

        // Devices
        {
            let mut stmt = conn.prepare(
                "SELECT id, name, kind, screens, hlc, deleted, seq
                 FROM devices WHERE seq > ?1 ORDER BY seq ASC LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![since_seq, limit], |row| {
                let kind_str: String = row.get(2)?;
                let screens_str: String = row.get(3)?;
                let hlc_str: String = row.get(4)?;
                let seq: u64 = row.get(6)?;
                Ok((
                    DeviceRecord {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        kind: if kind_str == "android" { DeviceKind::Android } else { DeviceKind::Linux },
                        screens: serde_json::from_str(&screens_str).unwrap_or_default(),
                        hlc: hlc_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                        deleted: row.get::<_, i64>(5)? != 0,
                    },
                    seq,
                ))
            })?;
            for r in rows {
                let (record, seq) = r?;
                if seq > max_seq {
                    max_seq = seq;
                }
                batch.devices.push(record);
            }
        }

        // Assignments
        {
            let mut stmt = conn.prepare(
                "SELECT device_id, screen_name, image_hash, fit, focus, hlc, deleted, seq
                 FROM assignments WHERE seq > ?1 ORDER BY seq ASC LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![since_seq, limit], |row| {
                let h_str: String = row.get(2)?;
                let fit_str: String = row.get(3)?;
                let focus_str: Option<String> = row.get(4)?;
                let hlc_str: String = row.get(5)?;
                let seq: u64 = row.get(7)?;
                Ok((
                    AssignmentRecord {
                        device_id: row.get(0)?,
                        screen_name: row.get(1)?,
                        image_hash: h_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                        fit: serde_json::from_str(&fit_str).unwrap_or(Fit::Fill),
                        focus: focus_str.and_then(|s| serde_json::from_str(&s).ok()),
                        hlc: hlc_str.parse().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?,
                        deleted: row.get::<_, i64>(6)? != 0,
                    },
                    seq,
                ))
            })?;
            for r in rows {
                let (record, seq) = r?;
                if seq > max_seq {
                    max_seq = seq;
                }
                batch.assignments.push(record);
            }
        }

        batch.max_seq = max_seq;
        Ok(batch)
    }

    /// Apply an incoming sync delta batch atomically using Last-Write-Wins (LWW) via HLC
    pub fn apply_batch(&self, batch: &SyncBatch) -> Result<ApplyReport> {
        let mut report = ApplyReport::default();
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        // Apply Images
        for img in &batch.images {
            let existing_hlc: Option<String> = tx
                .query_row("SELECT hlc FROM images WHERE hash = ?1", params![img.hash.to_hex()], |row| row.get(0))
                .optional()?;

            let should_apply = match existing_hlc {
                None => true,
                Some(s) => match s.parse::<Hlc>() {
                    Ok(curr) => img.hlc > curr,
                    Err(_) => true,
                },
            };

            if should_apply {
                let tags_json = serde_json::to_string(&img.tags)?;
                tx.execute(
                    "INSERT INTO images (hash, width, height, mime, added_at, tags, source_name, hlc, deleted, seq)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 0)
                     ON CONFLICT(hash) DO UPDATE SET
                        tags = excluded.tags,
                        source_name = excluded.source_name,
                        hlc = excluded.hlc,
                        deleted = excluded.deleted;",
                    params![
                        img.hash.to_hex(),
                        img.width,
                        img.height,
                        img.mime,
                        img.added_at,
                        tags_json,
                        img.source_name,
                        img.hlc.to_string_repr(),
                        if img.deleted { 1 } else { 0 },
                    ],
                )?;
                report.applied_images += 1;
            }
        }

        // Apply Devices
        for dev in &batch.devices {
            let existing_hlc: Option<String> = tx
                .query_row("SELECT hlc FROM devices WHERE id = ?1", params![dev.id], |row| row.get(0))
                .optional()?;

            let should_apply = match existing_hlc {
                None => true,
                Some(s) => match s.parse::<Hlc>() {
                    Ok(curr) => dev.hlc > curr,
                    Err(_) => true,
                },
            };

            if should_apply {
                let screens_json = serde_json::to_string(&dev.screens)?;
                let kind_str = match dev.kind {
                    DeviceKind::Linux => "linux",
                    DeviceKind::Android => "android",
                };
                tx.execute(
                    "INSERT INTO devices (id, name, kind, screens, hlc, deleted, seq)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)
                     ON CONFLICT(id) DO UPDATE SET
                        name = excluded.name,
                        kind = excluded.kind,
                        screens = excluded.screens,
                        hlc = excluded.hlc,
                        deleted = excluded.deleted;",
                    params![
                        dev.id,
                        dev.name,
                        kind_str,
                        screens_json,
                        dev.hlc.to_string_repr(),
                        if dev.deleted { 1 } else { 0 },
                    ],
                )?;
                report.applied_devices += 1;
            }
        }

        // Apply Assignments
        for assign in &batch.assignments {
            let existing_hlc: Option<String> = tx
                .query_row(
                    "SELECT hlc FROM assignments WHERE device_id = ?1 AND screen_name = ?2",
                    params![assign.device_id, assign.screen_name],
                    |row| row.get(0),
                )
                .optional()?;

            let should_apply = match existing_hlc {
                None => true,
                Some(s) => match s.parse::<Hlc>() {
                    Ok(curr) => assign.hlc > curr,
                    Err(_) => true,
                },
            };

            if should_apply {
                let fit_json = serde_json::to_string(&assign.fit)?;
                let focus_json = assign.focus.map(|f| serde_json::to_string(&f).unwrap_or_default());
                tx.execute(
                    "INSERT INTO assignments (device_id, screen_name, image_hash, fit, focus, hlc, deleted, seq)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0)
                     ON CONFLICT(device_id, screen_name) DO UPDATE SET
                        image_hash = excluded.image_hash,
                        fit = excluded.fit,
                        focus = excluded.focus,
                        hlc = excluded.hlc,
                        deleted = excluded.deleted;",
                    params![
                        assign.device_id,
                        assign.screen_name,
                        assign.image_hash.to_hex(),
                        fit_json,
                        focus_json,
                        assign.hlc.to_string_repr(),
                        if assign.deleted { 1 } else { 0 },
                    ],
                )?;
                report.applied_assignments.push(assign.clone());
            }
        }

        tx.commit()?;
        Ok(report)
    }

    pub fn set_peer_cursor(&self, peer_id: &str, last_sent_seq: u64, last_recv_hlc: Option<&Hlc>) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let hlc_str = last_recv_hlc.map(|h| h.to_string_repr());
        conn.execute(
            "INSERT INTO peer_state (peer_id, last_sent_seq, last_recv_hlc)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(peer_id) DO UPDATE SET
                last_sent_seq = excluded.last_sent_seq,
                last_recv_hlc = COALESCE(excluded.last_recv_hlc, peer_state.last_recv_hlc);",
            params![peer_id, last_sent_seq, hlc_str],
        )?;
        Ok(())
    }

    pub fn get_peer_cursor(&self, peer_id: &str) -> Result<(u64, Option<Hlc>)> {
        let conn = self.conn.lock().unwrap();
        let res: Option<(u64, Option<String>)> = conn
            .query_row(
                "SELECT last_sent_seq, last_recv_hlc FROM peer_state WHERE peer_id = ?1",
                params![peer_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;

        match res {
            Some((seq, Some(hlc_str))) => Ok((seq, hlc_str.parse().ok())),
            Some((seq, None)) => Ok((seq, None)),
            None => Ok((0, None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn make_test_image() -> Vec<u8> {
        let mut img = image::RgbImage::new(64, 64);
        for pixel in img.pixels_mut() {
            *pixel = image::Rgb([120, 180, 240]);
        }
        let mut buf = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buf);
        img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
        buf
    }

    #[test]
    fn test_ingest_and_cas() {
        let dir = tempdir().unwrap();
        let store = Store::open(dir.path(), "device-1").unwrap();

        let bytes = make_test_image();
        let record = store.ingest_bytes(&bytes, "test.png", vec!["nature".into()]).unwrap();
        assert_eq!(record.width, 64);
        assert_eq!(record.height, 64);

        let blob_path = store.blob_path(&record.hash);
        assert!(blob_path.exists());

        let retrieved = store.get_image(&record.hash).unwrap().unwrap();
        assert_eq!(retrieved.hash, record.hash);
        assert_eq!(retrieved.tags, vec!["nature"]);
    }

    #[test]
    fn test_delta_and_apply_batch() {
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();
        let store1 = Store::open(dir1.path(), "device-1").unwrap();
        let store2 = Store::open(dir2.path(), "device-2").unwrap();

        let bytes = make_test_image();
        let img1 = store1.ingest_bytes(&bytes, "pic1.png", vec![]).unwrap();

        store1.set_assignment("device-1", "HDMI-1", img1.hash, Fit::Fill, None).unwrap();

        let batch = store1.changes_since(0, 100).unwrap();
        assert_eq!(batch.images.len(), 1);
        assert_eq!(batch.assignments.len(), 1);

        let report = store2.apply_batch(&batch).unwrap();
        assert_eq!(report.applied_images, 1);
        assert_eq!(report.applied_assignments.len(), 1);

        let assign2 = store2.get_assignment("device-1", "HDMI-1").unwrap().unwrap();
        assert_eq!(assign2.image_hash, img1.hash);
    }
}
```

---

### File: `docs/design_spec.md`
- **Summary:** sync_wall Architecture & Engineering Specification (113 lines)
- **Path:** `docs/design_spec.md`
- **Estimated Tokens:** 1,911
- **mtime:** 1792398397.426

````markdown
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
````

---

### File: `docs/protocol.md`
- **Summary:** sync_wall Wire & Synchronization Protocol (v1) (131 lines)
- **Path:** `docs/protocol.md`
- **Estimated Tokens:** 1,109
- **mtime:** 1792399974.37

````markdown
# sync_wall Wire & Synchronization Protocol (v1)

## 1. Network Layer & Transport

- **Transport**: HTTP/1.1 over TLS 1.3 (`rustls`).
- **Default Port**: `7889` (TCP).
- **Service Discovery**: mDNS broadcast under service name `_syncwall._tcp.local.` with TXT records:
  - `device_id=<sha256_fingerprint>`
  - `name=<device_name>`
  - `v=1`

---

## 2. Cryptographic Identity & Authentication

### 2.1 Identity
Each device generates a self-signed X.509 certificate upon first initialization using `rcgen`.
The **Device ID** is the lowercase 64-character hex SHA-256 fingerprint of the certificate DER bytes:
```
device_id = hex(blake3(cert_der))
```

### 2.2 Mutual TLS & Pinning
- Both client and server employ custom `rustls` certificate verifiers.
- Connections are only accepted if the remote certificate fingerprint is listed in the `devices` table.
- Certificate Authorities (CAs) and domain name checks are bypassed.

---

## 3. Pairing Flow

```
   Desktop (Linux)                                        Phone (Android)
         |                                                       |
         |  1. Generate 6-digit PIN (120s TTL)                   |
         |  2. Display QR:                                       |
         |     { host, port, token, server_cert }                |
         |                                                       |
         |                               3. Scan QR              |
         |                               4. Connect TLS (pinning |
         |                                  server_cert)         |
         |                                                       |
         |<----- POST /v1/pair ----------------------------------|
         |       { token, client_fingerprint, name, screens }    |
         |                                                       |
         |  5. Constant-time token verification                  |
         |  6. Invalidate token                                  |
         |  7. Insert client_fingerprint into trusted devices    |
         |                                                       |
         |------ HTTP 200 OK ----------------------------------->|
         |       { success: true, server_fingerprint }           |
         |                                                       |
         |                       8. Save server_fingerprint      |
         |                          as trusted peer              |
```

---

## 4. API Endpoints

### `GET /v1/hello`
Returns server metadata and current monotonic sequence number.

**Response**:
```json
{
  "device_id": "dafed48e...",
  "name": "linux-desktop",
  "schema_version": 1,
  "current_seq": 142
}
```

---

### `POST /v1/pair`
Enroll a new peer device using a single-use token.

**Request**:
```json
{
  "token": "356821",
  "client_fingerprint": "a1b2c3d4...",
  "device_name": "Pixel 8",
  "kind": "Android",
  "screens": [
    { "name": "display", "width": 1080, "height": 2400, "scale": 2.625 }
  ]
}
```

**Response**:
```json
{
  "success": true,
  "server_name": "linux-desktop",
  "server_fingerprint": "dafed48e..."
}
```

---

### `GET /v1/changes?since=<seq>&limit=<n>`
Fetches mutations since a given monotonic cursor `seq`.

**Response (`SyncBatch`)**:
```json
{
  "images": [
    {
      "hash": "7fc22df7...",
      "width": 1920,
      "height": 1080,
      "mime": "image/png",
      "added_at": 1774009000,
      "tags": ["nature"],
      "source_name": "wallpaper.png",
      "hlc": "00000192f1a23b40:00000000:dafed48e...",
      "deleted": false
    }
  ],
  "devices": [],
  "assignments": [
    {
      "device_id": "dafed48e...",
      "screen_name": "monitor0",
      "image_hash": "7fc22df7...",
      "fit": "Fill",
      "focus": [0.5, 0.5],
      "hlc": "00000192f1a23b40:00000001:dafed48e...",
      "deleted": false
    }
  ],
  "collections": [],
  "collection_items": [],
  "max_seq": 145
}
```

---

### `POST /v1/changes`
Pushes local mutations to the remote peer. Idempotent. Resolved using Last-Write-Wins (LWW) via HLC timestamp comparison.

---

### `GET /v1/blobs/{hash}`
Download original or on-demand cropped derivative.

**Query Parameters**:
- `target_w`: Target width in pixels (e.g. `1080`)
- `target_h`: Target height in pixels (e.g. `2400`)
- `fit`: `fill` | `center` | `fit`

**Headers**:
- `Range: bytes=0-1048575` (Resumable chunked transfers)
- Response carries `X-Blake3: <hex_hash>` and `Content-Range`.
````

---

### File: `linux/backend/Cargo.toml`
- **Summary:** Cargo.toml (Rust project)
- **Path:** `linux/backend/Cargo.toml`
- **Estimated Tokens:** 53
- **mtime:** 1792399737.027

```toml
[package]
name = "sync_wall_backend"
version = "0.1.0"
edition = "2021"

[dependencies]
sync_wall_core = { path = "../../core" }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
tracing = "0.1"
```

---

### File: `linux/backend/src/lib.rs`
- **Summary:** structs: Output, XfconfBackend, MockBackend; enums: BackendError; traits: WallpaperBackend; 1 functions
- **Path:** `linux/backend/src/lib.rs`
- **Estimated Tokens:** 1,715
- **mtime:** 1792399745.356

```rust
use std::path::Path;
use std::process::Command;
use thiserror::Error;
use sync_wall_core::model::Fit;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Command execution error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Backend command failed: {0}")]
    CommandFailed(String),
    #[error("Output not found: {0}")]
    OutputNotFound(String),
    #[error("No compatible backend detected")]
    NoBackendDetected,
}

pub type Result<T> = std::result::Result<T, BackendError>;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub is_primary: bool,
}

pub trait WallpaperBackend: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self) -> bool;
    fn list_outputs(&self) -> Result<Vec<Output>>;
    fn set(&self, output_name: &str, img_path: &Path, fit: Fit) -> Result<()>;
}

/// XFCE Desktop Backend using `xfconf-query`
pub struct XfconfBackend;

impl XfconfBackend {
    pub fn new() -> Self {
        Self
    }

    /// Discover monitor properties from xfconf-query
    fn find_monitor_properties(&self, output_name: &str) -> Vec<String> {
        let output = Command::new("xfconf-query")
            .args(["-c", "xfce4-desktop", "-p", "/backdrop", "-l"])
            .output();

        let mut matched = Vec::new();
        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.contains("last-image") && (output_name.is_empty() || trimmed.contains(output_name)) {
                    matched.push(trimmed.to_string());
                }
            }
        }

        // Fallback: if no properties enumerated yet, generate standard workspace 0..4 paths
        if matched.is_empty() {
            let monitor = if output_name.is_empty() { "monitor0" } else { output_name };
            for ws in 0..8 {
                matched.push(format!("/backdrop/screen0/{}/workspace{}/last-image", monitor, ws));
            }
        }
        matched
    }
}

impl Default for XfconfBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl WallpaperBackend for XfconfBackend {
    fn name(&self) -> &'static str {
        "xfconf"
    }

    fn detect(&self) -> bool {
        let de = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_uppercase();
        let has_xfce = de.contains("XFCE");
        let has_binary = Command::new("which")
            .arg("xfconf-query")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        has_xfce || has_binary
    }

    fn list_outputs(&self) -> Result<Vec<Output>> {
        let props = self.find_monitor_properties("");
        let mut monitor_names = std::collections::BTreeSet::new();

        for p in &props {
            // Path format: /backdrop/screen0/monitorXYZ/workspaceN/last-image
            let parts: Vec<&str> = p.split('/').collect();
            for part in parts {
                if part.starts_with("monitor") || part.starts_with("screen") {
                    monitor_names.insert(part.to_string());
                }
            }
        }

        if monitor_names.is_empty() {
            monitor_names.insert("monitor0".to_string());
        }

        let mut outputs = Vec::new();
        for (i, name) in monitor_names.into_iter().enumerate() {
            outputs.push(Output {
                name,
                width: 1920,
                height: 1080,
                is_primary: i == 0,
            });
        }
        Ok(outputs)
    }

    fn set(&self, output_name: &str, img_path: &Path, _fit: Fit) -> Result<()> {
        let path_str = img_path.to_str()
            .ok_or_else(|| BackendError::CommandFailed("Invalid image path UTF-8".to_string()))?;

        let properties = self.find_monitor_properties(output_name);

        for prop in properties {
            // Set image path
            let status = Command::new("xfconf-query")
                .args(["-c", "xfce4-desktop", "-p", &prop, "-s", path_str, "--create", "-t", "string"])
                .status()?;

            if !status.success() {
                tracing::warn!("xfconf-query set failed for property {}", prop);
            }

            // Set image style to 5 (zoomed / fill)
            let style_prop = prop.replace("last-image", "image-style");
            let _ = Command::new("xfconf-query")
                .args(["-c", "xfce4-desktop", "-p", &style_prop, "-s", "5", "--create", "-t", "int"])
                .status();
        }

        Ok(())
    }
}

/// Fallback / Mock backend for testing
pub struct MockBackend {
    pub name_str: &'static str,
    pub last_set: std::sync::Mutex<Option<(String, std::path::PathBuf)>>,
}

impl MockBackend {
    pub fn new(name: &'static str) -> Self {
        Self {
            name_str: name,
            last_set: std::sync::Mutex::new(None),
        }
    }
}

impl WallpaperBackend for MockBackend {
    fn name(&self) -> &'static str {
        self.name_str
    }

    fn detect(&self) -> bool {
        true
    }

    fn list_outputs(&self) -> Result<Vec<Output>> {
        Ok(vec![
            Output {
                name: "HDMI-1".to_string(),
                width: 1920,
                height: 1080,
                is_primary: true,
            },
            Output {
                name: "DP-1".to_string(),
                width: 2560,
                height: 1440,
                is_primary: false,
            },
        ])
    }

    fn set(&self, output_name: &str, img_path: &Path, _fit: Fit) -> Result<()> {
        *self.last_set.lock().unwrap() = Some((output_name.to_string(), img_path.to_path_buf()));
        Ok(())
    }
}

/// Factory to auto-detect backend
pub fn detect_backend() -> Box<dyn WallpaperBackend> {
    let xfconf = XfconfBackend::new();
    if xfconf.detect() {
        return Box::new(xfconf);
    }
    Box::new(MockBackend::new("mock"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xfconf_detection() {
        let backend = XfconfBackend::new();
        // On our current Linux container with xfconf-query, detect() should be true
        assert!(backend.detect());
        assert_eq!(backend.name(), "xfconf");
    }

    #[test]
    fn test_mock_backend() {
        let mock = MockBackend::new("mock-test");
        let outputs = mock.list_outputs().unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0].name, "HDMI-1");

        mock.set("HDMI-1", Path::new("/tmp/test.png"), Fit::Fill).unwrap();
        let last = mock.last_set.lock().unwrap().clone().unwrap();
        assert_eq!(last.0, "HDMI-1");
        assert_eq!(last.1, Path::new("/tmp/test.png"));
    }
}
```

---

### File: `linux/cli/Cargo.toml`
- **Summary:** Cargo.toml (Rust project)
- **Path:** `linux/cli/Cargo.toml`
- **Estimated Tokens:** 95
- **mtime:** 1792399726.553

```toml
[package]
name = "sync_wall"
version = "0.1.0"
edition = "2021"

[dependencies]
sync_wall_core = { path = "../../core" }
sync_wall_backend = { path = "../backend" }
tokio = { version = "1.38", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.5", features = ["derive"] }
dirs = "5.0"
qrcode = "0.14"
thiserror = "1.0"
```

---

### File: `linux/cli/src/main.rs`
- **Summary:** structs: Cli; enums: Commands; 4 functions
- **Path:** `linux/cli/src/main.rs`
- **Estimated Tokens:** 3,249
- **mtime:** 1792399686.372

```rust
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use qrcode::render::unicode;
use qrcode::QrCode;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use sync_wall_backend::detect_backend;
use sync_wall_core::model::{Fit, Hash, OutFormat};
use sync_wall_core::SyncWallEngine;

#[derive(Parser, Debug)]
#[command(name = "sync_wall", author, version, about = "sync_wall desktop CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Ingest images into the wallpaper store
    Add {
        /// Image file paths to add
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Optional tags to associate with the images
        #[arg(short, long)]
        tags: Vec<String>,
    },

    /// Set desktop wallpaper
    Set {
        /// Image BLAKE3 hash or direct file path
        target: String,

        /// Specific monitor/output name (defaults to all)
        #[arg(short, long)]
        screen: Option<String>,
    },

    /// List all cataloged wallpapers
    List,

    /// List active monitors/screens
    Screens,

    /// Generate pairing token and terminal QR code for phone pairing
    Pair {
        /// Custom host/IP address to advertise (defaults to local hostname)
        #[arg(long)]
        host: Option<String>,

        /// Port where sync_walld is listening
        #[arg(short, long, default_value_t = 7889)]
        port: u16,
    },

    /// Trigger sync with a peer device
    Sync {
        /// Peer URL (e.g. https://192.168.1.50:7889)
        url: String,
    },

    /// Check daemon status and doctor diagnostics
    Status,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Check if daemon Unix socket exists
    let socket_path = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
        .join("sync_wall/sync_walld.sock");

    if socket_path.exists() {
        if let Ok(mut stream) = UnixStream::connect(&socket_path).await {
            match handle_via_daemon(&cli.command, &mut stream).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    eprintln!("Daemon RPC returned error: {}, falling back to local engine...", e);
                }
            }
        }
    }

    // Fallback: in-process direct execution
    handle_standalone(&cli.command).await?;
    Ok(())
}

async fn handle_via_daemon(cmd: &Commands, stream: &mut UnixStream) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = stream.split();
    let mut lines = BufReader::new(reader).lines();

    match cmd {
        Commands::Status => {
            let req = json!({ "id": 1, "method": "status", "params": {} });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(res) = v.get("result") {
                    println!("\x1b[1;32msync_wall status (connected to daemon)\x1b[0m");
                    println!("Device ID:   {}", res["device_id"].as_str().unwrap_or(""));
                    println!("Backend:     {}", res["backend"].as_str().unwrap_or(""));
                    println!("Outputs:     {}", res["outputs"]);
                    println!("Total Images:{}", res["image_count"]);
                    println!("Devices:     {}", res["device_count"]);
                }
            }
        }
        Commands::Screens => {
            let req = json!({ "id": 1, "method": "list_outputs", "params": {} });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(outputs) = v.get("result").and_then(|r| r.as_array()) {
                    println!("\x1b[1mDetected Screen Outputs:\x1b[0m");
                    for out in outputs {
                        println!(" - {} ({}x{}, primary: {})", 
                            out["name"].as_str().unwrap_or(""),
                            out["width"],
                            out["height"],
                            out["is_primary"]
                        );
                    }
                }
            }
        }
        Commands::Add { files, tags } => {
            let paths_str: Vec<String> = files.iter()
                .map(|p| p.canonicalize().unwrap_or_else(|_| p.clone()).to_string_lossy().to_string())
                .collect();
            let req = json!({
                "id": 1,
                "method": "add",
                "params": { "paths": paths_str, "tags": tags }
            });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(added) = v.get("result").and_then(|r| r.as_array()) {
                    println!("Successfully added {} image(s):", added.len());
                    for img in added {
                        println!(" - \x1b[36m{}\x1b[0m ({}x{}, {})", 
                            img["hash"].as_str().unwrap_or(""),
                            img["width"],
                            img["height"],
                            img["source_name"].as_str().unwrap_or("")
                        );
                    }
                }
            }
        }
        Commands::List => {
            let req = json!({ "id": 1, "method": "list_images", "params": {} });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(images) = v.get("result").and_then(|r| r.as_array()) {
                    println!("\x1b[1mCataloged Wallpapers ({}):\x1b[0m", images.len());
                    for img in images {
                        println!(" - \x1b[36m{}\x1b[0m {}x{} [{}] {}", 
                            img["hash"].as_str().unwrap_or(""),
                            img["width"],
                            img["height"],
                            img["mime"].as_str().unwrap_or(""),
                            img["source_name"].as_str().unwrap_or("")
                        );
                    }
                }
            }
        }
        Commands::Set { target, screen } => {
            let hash = if let Ok(h) = target.parse::<Hash>() {
                h.to_string()
            } else {
                let path = PathBuf::from(target);
                if !path.exists() {
                    return Err(format!("File '{}' not found", target).into());
                }
                let bytes = std::fs::read(&path)?;
                Hash::compute(&bytes).to_string()
            };

            let req = json!({
                "id": 1,
                "method": "set",
                "params": { "hash": hash, "screen": screen }
            });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(err) = v.get("error").and_then(|e| e.as_str()) {
                    return Err(err.to_string().into());
                }
                println!("\x1b[32mWallpaper applied successfully!\x1b[0m");
            }
        }
        Commands::Pair { host, port } => {
            let req = json!({ "id": 1, "method": "pair_token", "params": {} });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(res) = v.get("result") {
                    let token = res["token"].as_str().unwrap_or("");
                    let device_id = res["device_id"].as_str().unwrap_or("");
                    let host_str = host.clone().unwrap_or_else(|| "127.0.0.1".to_string());

                    let qr_payload = json!({
                        "host": host_str,
                        "port": port,
                        "token": token,
                        "server_cert": device_id
                    }).to_string();

                    render_pairing_qr(&qr_payload, token);
                }
            }
        }
        Commands::Sync { url } => {
            let req = json!({ "id": 1, "method": "sync_peer", "params": { "url": url } });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                println!("Sync report: {:?}", v.get("result"));
            }
        }
    }
    Ok(())
}

async fn handle_standalone(cmd: &Commands) -> Result<(), Box<dyn std::error::Error>> {
    let base_data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("sync_wall");
    let engine = SyncWallEngine::open(&base_data_dir, "sync_wall_cli")?;
    let backend = detect_backend();

    match cmd {
        Commands::Status => {
            println!("\x1b[1;33msync_wall status (standalone)\x1b[0m");
            println!("Device ID:   {}", engine.device_id());
            println!("Backend:     {}", backend.name());
            let outputs = backend.list_outputs()?;
            println!("Outputs:     {:?}", outputs.iter().map(|o| &o.name).collect::<Vec<_>>());
            let images = engine.list_images()?;
            println!("Total Images:{}", images.len());
        }
        Commands::Screens => {
            let outputs = backend.list_outputs()?;
            println!("\x1b[1mDetected Screen Outputs:\x1b[0m");
            for out in outputs {
                println!(" - {} ({}x{}, primary: {})", out.name, out.width, out.height, out.is_primary);
            }
        }
        Commands::Add { files, tags } => {
            for f in files {
                let rec = engine.ingest_file(f, tags.clone())?;
                println!("Added \x1b[36m{}\x1b[0m ({}x{}, {})", rec.hash, rec.width, rec.height, rec.source_name);
            }
        }
        Commands::List => {
            let images = engine.list_images()?;
            println!("\x1b[1mCataloged Wallpapers ({}):\x1b[0m", images.len());
            for img in images {
                println!(" - \x1b[36m{}\x1b[0m {}x{} [{}] {}", img.hash, img.width, img.height, img.mime, img.source_name);
            }
        }
        Commands::Set { target, screen } => {
            let hash = if let Ok(h) = target.parse::<Hash>() {
                h
            } else {
                let p = PathBuf::from(target);
                let rec = engine.ingest_file(&p, vec![])?;
                rec.hash
            };

            let outputs = backend.list_outputs()?;
            let target_outputs = if let Some(s) = screen {
                outputs.into_iter().filter(|o| &o.name == s).collect()
            } else {
                outputs
            };

            for out in &target_outputs {
                engine.set_wallpaper(&out.name, hash, Fit::Fill, Some((0.5, 0.5)))?;
                let rendered = engine.render_wallpaper_for_screen(&out.name, (out.width, out.height), OutFormat::Png)?;
                backend.set(&out.name, &rendered, Fit::Fill)?;
                println!("Applied to screen \x1b[32m{}\x1b[0m -> {:?}", out.name, rendered);
            }
        }
        Commands::Pair { host, port } => {
            let token = engine.generate_pairing_token();
            let host_str = host.clone().unwrap_or_else(|| "127.0.0.1".to_string());
            let qr_payload = json!({
                "host": host_str,
                "port": port,
                "token": token,
                "server_cert": engine.device_id()
            }).to_string();

            render_pairing_qr(&qr_payload, &token);
        }
        Commands::Sync { url } => {
            let report = engine.sync_with_peer(url).await?;
            println!("Sync completed: {:?}", report);
        }
    }
    Ok(())
}

fn render_pairing_qr(payload: &str, token: &str) {
    println!("\n\x1b[1;36m=== Scan with sync_wall Android Companion App ===\x1b[0m");
    if let Ok(code) = QrCode::new(payload.as_bytes()) {
        let image = code.render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        println!("{}", image);
    }
    println!("One-Time Pairing PIN: \x1b[1;32m{}\x1b[0m (valid for 2 minutes)", token);
    println!("Payload: {}\n", payload);
}
```

---

### File: `linux/daemon/Cargo.toml`
- **Summary:** Cargo.toml (Rust project)
- **Path:** `linux/daemon/Cargo.toml`
- **Estimated Tokens:** 125
- **mtime:** 1792399754.198

```toml
[package]
name = "sync_walld"
version = "0.1.0"
edition = "2021"

[dependencies]
sync_wall_core = { path = "../../core" }
sync_wall_backend = { path = "../backend" }
tokio = { version = "1.38", features = ["full"] }
axum = { version = "0.7", features = ["tokio"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
dirs = "5.0"
clap = { version = "4.5", features = ["derive"] }
```

---

### File: `linux/daemon/src/main.rs`
- **Summary:** structs: Args, RpcRequest, RpcResponse; 3 functions
- **Path:** `linux/daemon/src/main.rs`
- **Estimated Tokens:** 2,149
- **mtime:** 1792399763.509

```rust
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tracing::{error, info, warn};

use sync_wall_backend::{detect_backend, WallpaperBackend};
use sync_wall_core::model::*;
use sync_wall_core::SyncWallEngine;

#[derive(Parser, Debug)]
#[command(author, version, about = "sync_wall Linux Headless Daemon")]
struct Args {
    #[arg(short, long)]
    port: Option<u16>,

    #[arg(long)]
    data_dir: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    pub id: u64,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    pub id: u64,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

struct DaemonContext {
    engine: SyncWallEngine,
    backend: Box<dyn WallpaperBackend>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let base_data_dir = args.data_dir.unwrap_or_else(|| {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("sync_wall")
    });
    fs::create_dir_all(&base_data_dir)?;

    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "linux-pc".to_string());

    info!("Initializing sync_wall engine at {:?}", base_data_dir);
    let engine = SyncWallEngine::open(&base_data_dir, &hostname)?;
    let backend = detect_backend();
    info!("Detected active wallpaper backend: {}", backend.name());

    let ctx = Arc::new(DaemonContext { engine, backend });

    // Setup Unix Domain Socket for CLI/UI JSON-RPC
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
        .join("sync_wall");
    fs::create_dir_all(&runtime_dir)?;
    let socket_path = runtime_dir.join("sync_walld.sock");
    if socket_path.exists() {
        let _ = fs::remove_file(&socket_path);
    }

    let unix_listener = UnixListener::bind(&socket_path)?;
    info!("Listening for local JSON-RPC at {:?}", socket_path);

    // Setup HTTP LAN sync server
    let server_port = args.port.unwrap_or(7889);
    let router = sync_wall_core::net::create_router(ctx.engine.server_state());
    let tcp_addr = format!("0.0.0.0:{}", server_port);
    let tcp_listener = tokio::net::TcpListener::bind(&tcp_addr).await?;
    info!("Hosting peer sync server at {}", tcp_addr);

    tokio::spawn(async move {
        if let Err(e) = axum::serve(tcp_listener, router).await {
            error!("Sync server error: {}", e);
        }
    });

    // Accept Unix socket connections
    loop {
        match unix_listener.accept().await {
            Ok((stream, _)) => {
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    handle_unix_client(stream, ctx_clone).await;
                });
            }
            Err(e) => {
                warn!("Accept failed: {}", e);
            }
        }
    }
}

async fn handle_unix_client(stream: UnixStream, ctx: Arc<DaemonContext>) {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        if line.trim().is_empty() {
            continue;
        }

        let req: RpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = RpcResponse {
                    id: 0,
                    result: None,
                    error: Some(format!("Invalid JSON-RPC request: {}", e)),
                };
                let resp_str = serde_json::to_string(&resp).unwrap_or_default() + "\n";
                let _ = writer.write_all(resp_str.as_bytes()).await;
                continue;
            }
        };

        let result = dispatch_rpc(&req.method, req.params, &ctx).await;
        let resp = match result {
            Ok(v) => RpcResponse {
                id: req.id,
                result: Some(v),
                error: None,
            },
            Err(err_msg) => RpcResponse {
                id: req.id,
                result: None,
                error: Some(err_msg),
            },
        };

        let resp_str = serde_json::to_string(&resp).unwrap_or_default() + "\n";
        if writer.write_all(resp_str.as_bytes()).await.is_err() {
            break;
        }
    }
}

async fn dispatch_rpc(
    method: &str,
    params: serde_json::Value,
    ctx: &DaemonContext,
) -> Result<serde_json::Value, String> {
    match method {
        "status" => {
            let outputs = ctx.backend.list_outputs().map_err(|e| e.to_string())?;
            let images = ctx.engine.list_images().map_err(|e| e.to_string())?;
            let devices = ctx.engine.store().list_devices().map_err(|e| e.to_string())?;
            Ok(serde_json::json!({
                "device_id": ctx.engine.device_id(),
                "backend": ctx.backend.name(),
                "outputs": outputs.iter().map(|o| &o.name).collect::<Vec<_>>(),
                "image_count": images.len(),
                "device_count": devices.len(),
            }))
        }
        "list_outputs" => {
            let outputs = ctx.backend.list_outputs().map_err(|e| e.to_string())?;
            Ok(serde_json::json!(outputs))
        }
        "add" => {
            let paths: Vec<String> = serde_json::from_value(params.get("paths").cloned().unwrap_or_default())
                .map_err(|e| e.to_string())?;
            let tags: Vec<String> = serde_json::from_value(params.get("tags").cloned().unwrap_or_default())
                .unwrap_or_default();

            let mut added = Vec::new();
            for p in paths {
                let rec = ctx.engine.ingest_file(PathBuf::from(p), tags.clone())
                    .map_err(|e| e.to_string())?;
                added.push(rec);
            }
            Ok(serde_json::json!(added))
        }
        "list_images" => {
            let images = ctx.engine.list_images().map_err(|e| e.to_string())?;
            Ok(serde_json::json!(images))
        }
        "set" => {
            let hash_str = params.get("hash").and_then(|v| v.as_str())
                .ok_or_else(|| "Missing hash param".to_string())?;
            let hash: Hash = hash_str.parse().map_err(|e| format!("Invalid hash: {}", e))?;

            let screen_name = params.get("screen").and_then(|v| v.as_str()).unwrap_or("");
            let outputs = ctx.backend.list_outputs().map_err(|e| e.to_string())?;

            let target_outputs = if screen_name.is_empty() {
                outputs.clone()
            } else {
                outputs.into_iter().filter(|o| o.name == screen_name).collect()
            };

            if target_outputs.is_empty() {
                return Err(format!("Output '{}' not found", screen_name));
            }

            for out in &target_outputs {
                // Record assignment
                ctx.engine.set_wallpaper(&out.name, hash, Fit::Fill, Some((0.5, 0.5)))
                    .map_err(|e| e.to_string())?;

                // Render cropped desktop derivative (PNG)
                let rendered_path = ctx.engine.render_wallpaper_for_screen(
                    &out.name,
                    (out.width, out.height),
                    OutFormat::Png,
                ).map_err(|e| e.to_string())?;

                // Apply to desktop
                ctx.backend.set(&out.name, &rendered_path, Fit::Fill)
                    .map_err(|e| e.to_string())?;
            }

            Ok(serde_json::json!({ "applied_screens": target_outputs.iter().map(|o| &o.name).collect::<Vec<_>>() }))
        }
        "pair_token" => {
            let token = ctx.engine.generate_pairing_token();
            Ok(serde_json::json!({
                "token": token,
                "device_id": ctx.engine.device_id(),
                "expires_in_secs": 120
            }))
        }
        "sync_peer" => {
            let url = params.get("url").and_then(|v| v.as_str())
                .ok_or_else(|| "Missing url param".to_string())?;
            let report = ctx.engine.sync_with_peer(url).await
                .map_err(|e| e.to_string())?;
            Ok(serde_json::json!(report))
        }
        _ => Err(format!("Unknown RPC method: {}", method)),
    }
}
```

---

