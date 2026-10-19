# Wallpaper Sources & Ingestion Guide (Tabliss, Unsplash, Wallhaven, Bing)

## 1. Overview & Inspiration: Tabliss Integration

[Tabliss](https://tabliss.io/) is a popular, open-source new tab extension known for its elegant, distraction-free desktop photography. By reverse-engineering Tabliss's wallpaper engine (`joelshepherd/tabliss`), `sync_wall` adopts its best practices for sourcing, categorizing, and attributing desktop wallpapers.

### Key Tabliss Sourcing Mechanics
- **Official Curated Collection**: Collection ID `1053828` on Unsplash. This is a hand-picked gallery of high-resolution, landscape-oriented wallpapers with minimal clutter, optimal contrast, and scenic composition.
- **Orientation Filtering**: Automatically requesting `orientation=landscape` for desktop screens (or `orientation=portrait` when requested for mobile companions).
- **Photographer Attribution**: Preserving photographer credits and original descriptions for UI attribution.
- **Keyless Fallbacks**: For environments where users have not configured an Unsplash API key, `sync_wall` provides zero-configuration public APIs (Wallhaven and Bing Daily Wallpaper) that deliver instant high-resolution photography.

---

## 2. Supported Sources

### 2.1 Unsplash (Tabliss Model)
- **API Endpoint**: `https://api.unsplash.com/photos/random`
- **Default Collection**: `1053828` (Tabliss Curated Wallpapers)
- **Authentication**: Requires an Unsplash Access Key (`Authorization: Client-ID <KEY>`).
- **Configuration**:
  - Environment variables: `UNSPLASH_ACCESS_KEY` or `UNSPLASH_API_KEY`
  - CLI flag: `--key <KEY>`
- **Parameters**:
  - `--query <keywords>`: E.g. `minimalist`, `mountains`, `japan`
  - `--collection <id>`: Defaults to `1053828`
  - `--orientation <landscape|portrait>`: Defaults to `landscape`
  - `--count <1-10>`: Number of photos to retrieve in batch

### 2.2 Wallhaven (Zero-Key High-Res Provider)
- **API Endpoint**: `https://wallhaven.cc/api/v1/search?q=<query>&sorting=random&purity=100`
- **Authentication**: None required for general SFW (purity=100) high-resolution wallpapers.
- **Parameters**:
  - `--query <keywords>`: Freeform search query (defaults to `nature`).
  - `--count <1-10>`: Batch download count.

### 2.3 Bing Daily Wallpaper (Zero-Key Daily Provider)
- **API Endpoint**: `https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US`
- **Authentication**: None required.
- **Features**: Ingests the current Bing daily featured landscape photograph along with its copyright attribution string.

---

## 3. CAS Pipeline & Transactional Ingestion

When any online source downloads an image, it flows through the immutable `sync_wall_core` pipeline:
1. **Network Download**: Retrieved over HTTPS using `reqwest` with `rustls-tls-webpki-roots`.
2. **BLAKE3 Hash Calculation**: The raw image bytes are hashed using BLAKE3 (`Hash::compute(&bytes)`).
3. **Deduplication Check**: If the image hash already exists in the SQLite store, the re-download is deduplicated without disk bloat.
4. **Atomic CAS Ingestion**: If new, written to a temporary `.tmp` file and atomically renamed to `objects/ab/cdef...` in the content-addressable store.
5. **EXIF & Resolution Inspection**: Image dimensions and mime-types are extracted.
6. **Relational Metadata**: Stored with HLC timestamp, source name/attribution, and searchable tags.
7. **Monotonic Sequence Bump**: Triggers state sync so paired phone and companion devices discover the new wallpaper on next sync.

---

## 4. CLI Usage Examples

### Fetch and Immediately Apply Bing Daily Wallpaper
```bash
# Fetches today's Bing wallpaper, renders per-monitor crop, and applies to desktop
sync_wall fetch bing --apply
```

### Fetch Curated Wallpapers via Unsplash (Tabliss Collection)
```bash
export UNSPLASH_ACCESS_KEY="your-unsplash-access-key"

# Fetch from Tabliss curated collection 1053828
sync_wall fetch unsplash

# Fetch with query and immediate application
sync_wall fetch unsplash --query "cyberpunk neon" --apply
```

### Fetch from Wallhaven without API Keys
```bash
# Fetch 3 minimalist wallpapers
sync_wall fetch wallhaven --query "minimalist" --count 3

# Fetch and apply a mountain landscape
sync_wall fetch wallhaven --query "mountains" --apply
```

---

## 5. Daemon RPC Integration

When `sync_walld` is running, the CLI communicates over the Unix domain socket (`$XDG_RUNTIME_DIR/sync_wall/sync_walld.sock`) via JSON-RPC:

```json
{
  "id": 1,
  "method": "fetch",
  "params": {
    "provider": "wallhaven",
    "query": "aurora borealis",
    "count": 1,
    "apply": true
  }
}
```

The daemon handles downloading, storage, rendering, and desktop property updates in the background, returning the registered `ImageRecord` objects.
