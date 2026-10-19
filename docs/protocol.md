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
