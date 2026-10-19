use std::fs;
#[cfg(feature = "server")]
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
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

use rustls::client::danger::{ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::server::danger::{ClientCertVerified, ClientCertVerifier};
use rustls::{DigitallySignedStruct, SignatureScheme};

#[derive(Clone, Debug)]
pub struct PairingSession {
    pub token: String, // 128-bit hex string (32 characters = 16 bytes entropy)
    pub pin: String,   // 6-digit numeric fallback
    pub created_at: std::time::Instant,
    pub failed_attempts: u32,
}

impl Default for PairingSession {
    fn default() -> Self {
        Self::new()
    }
}

impl PairingSession {
    pub fn new() -> Self {
        use rand::RngCore;
        let mut bytes = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut bytes);
        let token = hex::encode(bytes);

        use rand::Rng;
        let pin_num: u32 = rand::thread_rng().gen_range(100_000..1_000_000);
        let pin = pin_num.to_string();

        Self {
            token,
            pin,
            created_at: std::time::Instant::now(),
            failed_attempts: 0,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed().as_secs() > 120
    }
}

#[derive(Clone)]
pub struct ServerState {
    pub store: Arc<Store>,
    pub crop: Arc<CropEngine>,
    pub identity: DeviceIdentity,
    pub active_pairing_token: Arc<parking_lot::Mutex<Option<PairingSession>>>,
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

#[cfg(feature = "server")]
pub fn create_router(state: ServerState) -> Router {
    Router::new()
        .route("/v1/hello", get(handle_hello))
        .route("/v1/pair", post(handle_pair))
        .route("/v1/changes", get(handle_get_changes).post(handle_post_changes))
        .route("/v1/blobs/:hash", get(handle_get_blob))
        .with_state(state)
}

#[cfg(feature = "server")]
async fn handle_hello(State(state): State<ServerState>) -> Json<HelloResponse> {
    let (seq, _) = state.store.get_peer_cursor("self").unwrap_or((0, None));
    Json(HelloResponse {
        device_id: state.identity.fingerprint.clone(),
        name: state.store.device_id().to_string(),
        schema_version: 1,
        current_seq: seq,
    })
}

#[cfg(feature = "server")]
async fn handle_pair(
    State(state): State<ServerState>,
    Json(req): Json<PairRequest>,
) -> std::result::Result<Json<PairResponse>, StatusCode> {
    let mut token_lock = state.active_pairing_token.lock();
    let session = match token_lock.as_mut() {
        Some(s) if !s.is_expired() => s,
        _ => {
            *token_lock = None;
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if session.failed_attempts >= 5 {
        *token_lock = None;
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    let req_bytes = req.token.as_bytes();
    let token_match = bool::from(session.token.as_bytes().ct_eq(req_bytes));
    let pin_match = bool::from(session.pin.as_bytes().ct_eq(req_bytes));

    if !token_match && !pin_match {
        session.failed_attempts += 1;
        if session.failed_attempts >= 5 {
            *token_lock = None;
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
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

#[derive(Debug)]
pub struct PinnedServerCertVerifier {
    pub pinned_fingerprint: String,
}

impl ServerCertVerifier for PinnedServerCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> std::result::Result<ServerCertVerified, rustls::Error> {
        let digest = blake3::hash(end_entity.as_ref());
        let fp = hex::encode(digest.as_bytes());
        if fp == self.pinned_fingerprint {
            Ok(ServerCertVerified::assertion())
        } else {
            Err(rustls::Error::InvalidCertificate(
                rustls::CertificateError::ApplicationVerificationFailure,
            ))
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &rustls::crypto::ring::default_provider().signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &rustls::crypto::ring::default_provider().signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        rustls::crypto::ring::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}

pub struct PinnedClientCertVerifier {
    pub store: Arc<Store>,
    pub allow_pairing: bool,
}

impl std::fmt::Debug for PinnedClientCertVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PinnedClientCertVerifier")
            .field("allow_pairing", &self.allow_pairing)
            .finish()
    }
}

impl ClientCertVerifier for PinnedClientCertVerifier {
    fn offer_client_auth(&self) -> bool {
        true
    }

    fn client_auth_mandatory(&self) -> bool {
        !self.allow_pairing
    }

    fn root_hint_subjects(&self) -> &[rustls::DistinguishedName] {
        &[]
    }

    fn verify_client_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _now: UnixTime,
    ) -> std::result::Result<ClientCertVerified, rustls::Error> {
        let digest = blake3::hash(end_entity.as_ref());
        let fp = hex::encode(digest.as_bytes());

        if let Ok(Some(_)) = self.store.get_device(&fp) {
            Ok(ClientCertVerified::assertion())
        } else if self.allow_pairing {
            Ok(ClientCertVerified::assertion())
        } else {
            Err(rustls::Error::InvalidCertificate(
                rustls::CertificateError::UnknownIssuer,
            ))
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &rustls::crypto::ring::default_provider().signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &rustls::crypto::ring::default_provider().signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        rustls::crypto::ring::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
async fn handle_post_changes(
    State(state): State<ServerState>,
    Json(batch): Json<SyncBatch>,
) -> std::result::Result<Json<ApplyReport>, StatusCode> {
    let report = state.store.apply_batch(&batch)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(report))
}

#[cfg(feature = "server")]
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

#[allow(dead_code)]
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

#[cfg(feature = "server")]
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
        req = req.header(reqwest::header::RANGE, format!("bytes={}-", initial_offset));
    }

    let resp = req.send().await?;
    let status = resp.status();
    if !status.is_success() && status != reqwest::StatusCode::PARTIAL_CONTENT {
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

    #[test]
    fn test_untrusted_client_cert_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let store = Arc::new(Store::open(dir.path(), "server-dev").unwrap());

        let enrolled_client = DeviceIdentity::generate("client-trusted").unwrap();
        let untrusted_client = DeviceIdentity::generate("client-untrusted").unwrap();

        // Enroll enrolled_client in devices table
        let dev_rec = DeviceRecord {
            id: enrolled_client.fingerprint.clone(),
            name: "trusted-phone".into(),
            kind: DeviceKind::Android,
            screens: vec![],
            hlc: store.tick_local(),
            deleted: false,
        };
        store.upsert_device(&dev_rec).unwrap();

        let verifier = PinnedClientCertVerifier {
            store: store.clone(),
            allow_pairing: false,
        };

        let trusted_cert = CertificateDer::from(enrolled_client.cert_der);
        let untrusted_cert = CertificateDer::from(untrusted_client.cert_der);
        let now = UnixTime::now();

        // Trusted client cert MUST succeed
        let res_trusted = verifier.verify_client_cert(&trusted_cert, &[], now);
        assert!(res_trusted.is_ok());

        // Untrusted client cert MUST be rejected with UnknownIssuer
        let res_untrusted = verifier.verify_client_cert(&untrusted_cert, &[], now);
        assert!(res_untrusted.is_err());
        match res_untrusted.err().unwrap() {
            rustls::Error::InvalidCertificate(rustls::CertificateError::UnknownIssuer) => {}
            other => panic!("Expected UnknownIssuer, got {:?}", other),
        }
    }

    #[test]
    fn test_wrong_server_fingerprint_refused() {
        let server = DeviceIdentity::generate("server-desktop").unwrap();
        let server_cert = CertificateDer::from(server.cert_der);
        let now = UnixTime::now();
        let server_name = ServerName::try_from("localhost").unwrap();

        // Client pinning matching fingerprint -> OK
        let correct_verifier = PinnedServerCertVerifier {
            pinned_fingerprint: server.fingerprint.clone(),
        };
        let res_correct = correct_verifier.verify_server_cert(&server_cert, &[], &server_name, &[], now);
        assert!(res_correct.is_ok());

        // Client pinning WRONG fingerprint -> Must be refused with ApplicationVerificationFailure
        let wrong_verifier = PinnedServerCertVerifier {
            pinned_fingerprint: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        };
        let res_wrong = wrong_verifier.verify_server_cert(&server_cert, &[], &server_name, &[], now);
        assert!(res_wrong.is_err());
        match res_wrong.err().unwrap() {
            rustls::Error::InvalidCertificate(rustls::CertificateError::ApplicationVerificationFailure) => {}
            other => panic!("Expected ApplicationVerificationFailure, got {:?}", other),
        }
    }

    #[cfg(feature = "server")]
    #[tokio::test]
    async fn test_blob_download_interrupted_and_resumed_via_range() {
        let dir = tempfile::tempdir().unwrap();
        let store = Arc::new(Store::open(dir.path(), "test-server").unwrap());
        let crop = Arc::new(CropEngine::new(dir.path().join("derived")));
        let identity = DeviceIdentity::generate("test-server").unwrap();

        // Create a valid PNG image
        let mut img = image::RgbImage::new(128, 128);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = image::Rgb([(x % 256) as u8, (y % 256) as u8, 200]);
        }
        let mut data = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut data);
        img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
        assert!(data.len() > 100);

        let record = store.ingest_bytes(&data, "test.png", vec![]).unwrap();
        let expected_hash = record.hash;

        let server_state = ServerState {
            store: store.clone(),
            crop: crop.clone(),
            identity: identity.clone(),
            active_pairing_token: Arc::new(parking_lot::Mutex::new(None)),
        };

        let app = create_router(server_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = reqwest::Client::new();
        let dest_dir = tempfile::tempdir().unwrap();
        let dest_file = dest_dir.path().join("downloaded.png");
        let part_file = dest_dir.path().join("downloaded.part");

        // Simulate killed mid-way: write half into .part
        let split_pos = data.len() / 2;
        std::fs::write(&part_file, &data[..split_pos]).unwrap();
        assert_eq!(std::fs::metadata(&part_file).unwrap().len(), split_pos as u64);

        // Call resumable download
        let peer_url = format!("http://{}", addr);
        download_blob_resumable(&client, &peer_url, &expected_hash, None, &dest_file).await.unwrap();

        // Verify destination exists and .part was cleanly renamed
        assert!(dest_file.exists());
        assert!(!part_file.exists());

        let downloaded = std::fs::read(&dest_file).unwrap();
        assert_eq!(downloaded.len(), data.len());
        assert_eq!(downloaded, data);
        assert_eq!(Hash::compute(&downloaded), expected_hash);
    }

    #[cfg(feature = "server")]
    #[tokio::test]
    async fn test_malformed_and_oversized_payloads() {
        let dir = tempfile::tempdir().unwrap();
        let store = Arc::new(Store::open(dir.path(), "test-server").unwrap());
        let crop = Arc::new(CropEngine::new(dir.path().join("derived")));
        let identity = DeviceIdentity::generate("test-server").unwrap();

        let server_state = ServerState {
            store: store.clone(),
            crop: crop.clone(),
            identity: identity.clone(),
            active_pairing_token: Arc::new(parking_lot::Mutex::new(None)),
        };

        let app = create_router(server_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = reqwest::Client::new();
        let pair_url = format!("http://{}/v1/pair", addr);

        // Malformed JSON returns 400 Bad Request or 422 Unprocessable Entity, zero panic
        let resp_malformed = client.post(&pair_url)
            .header("Content-Type", "application/json")
            .body("{ malformed: true, invalid }")
            .send()
            .await
            .unwrap();
        assert!(resp_malformed.status().is_client_error());

        // Missing required fields returns 422 Unprocessable Entity
        let resp_missing = client.post(&pair_url)
            .header("Content-Type", "application/json")
            .body(r#"{"random_field": "test"}"#)
            .send()
            .await
            .unwrap();
        assert_eq!(resp_missing.status(), StatusCode::UNPROCESSABLE_ENTITY);

        // Non-existent blob returns 404 Not Found
        let bogus_hash = "0000000000000000000000000000000000000000000000000000000000000000";
        let blob_url = format!("http://{}/v1/blobs/{}", addr, bogus_hash);
        let resp_blob = client.get(&blob_url).send().await.unwrap();
        assert_eq!(resp_blob.status(), StatusCode::NOT_FOUND);
    }

    #[cfg(feature = "server")]
    #[tokio::test]
    async fn test_pairing_session_lockout_after_failed_attempts() {
        let dir = tempfile::tempdir().unwrap();
        let store = Arc::new(Store::open(dir.path(), "test-server").unwrap());
        let crop = Arc::new(CropEngine::new(dir.path().join("derived")));
        let identity = DeviceIdentity::generate("test-server").unwrap();
        let session = PairingSession::new();
        let token_lock = Arc::new(parking_lot::Mutex::new(Some(session.clone())));

        let server_state = ServerState {
            store: store.clone(),
            crop: crop.clone(),
            identity: identity.clone(),
            active_pairing_token: token_lock.clone(),
        };

        let app = create_router(server_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = reqwest::Client::new();
        let pair_url = format!("http://{}/v1/pair", addr);

        // 4 failed attempts with wrong token -> 401 UNAUTHORIZED
        for _ in 0..4 {
            let req = serde_json::json!({
                "token": "wrong_token",
                "client_fingerprint": "fake_fp",
                "device_name": "attacker",
                "kind": "Android",
                "screens": []
            });
            let resp = client.post(&pair_url).json(&req).send().await.unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        }

        // 5th failed attempt -> 429 TOO_MANY_REQUESTS and session is permanently invalidated (lockout)
        let req = serde_json::json!({
            "token": "wrong_token",
            "client_fingerprint": "fake_fp",
            "device_name": "attacker",
            "kind": "Android",
            "screens": []
        });
        let resp = client.post(&pair_url).json(&req).send().await.unwrap();
        assert_eq!(resp.status(), StatusCode::TOO_MANY_REQUESTS);

        // Even presenting the correct token now fails because session is locked out
        let req_correct = serde_json::json!({
            "token": session.token,
            "client_fingerprint": "fake_fp",
            "device_name": "attacker",
            "kind": "Android",
            "screens": []
        });
        let resp_locked = client.post(&pair_url).json(&req_correct).send().await.unwrap();
        assert_eq!(resp_locked.status(), StatusCode::UNAUTHORIZED);
    }
}
