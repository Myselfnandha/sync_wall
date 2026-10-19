pub mod crop;
pub mod model;
pub mod net;
pub mod source;
pub mod store;
pub mod uniffi_api;

uniffi::setup_scaffolding!("sync_wall_core");

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;

use crate::crop::CropEngine;
use crate::model::*;
use crate::net::DeviceIdentity;
#[cfg(feature = "server")]
use crate::net::ServerState;
use crate::source::{SourceError, SourceManager, UnsplashConfig};
use crate::store::Store;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Store error: {0}")]
    Store(#[from] store::StoreError),
    #[error("Crop error: {0}")]
    Crop(#[from] crop::CropError),
    #[error("Network error: {0}")]
    Net(#[from] net::NetError),
    #[error("Source error: {0}")]
    Source(#[from] SourceError),
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
    base_dir: PathBuf,
    store: Arc<Store>,
    crop: Arc<CropEngine>,
    identity: DeviceIdentity,
    active_token: Arc<parking_lot::Mutex<Option<crate::net::PairingSession>>>,
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
            base_dir,
            store,
            crop,
            identity,
            active_token,
        })
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
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

    #[cfg(feature = "server")]
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

    /// Render or fetch cached derivative for an assignment and promote to stable current/<screen_name>.<ext>
    /// This guarantees that when LRU cache evicts files from derived/, the desktop never loses its active wallpaper.
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

        let cached_path = self.crop.get_or_render(&assign.image_hash, &source_path, &spec)?;

        // Fix 2: Copy rendered file to stable current/<screen_name>.<ext> via atomic temp file
        let ext = match format {
            OutFormat::Png => "png",
            OutFormat::WebpLossy(_) => "webp",
        };
        let current_dir = self.base_dir.join("current");
        fs::create_dir_all(&current_dir)?;
        let stable_path = current_dir.join(format!("{}.{}", screen_name, ext));
        let tmp_path = current_dir.join(format!("{}.{}.tmp", screen_name, ext));

        fs::copy(&cached_path, &tmp_path)?;
        fs::rename(&tmp_path, &stable_path)?;

        Ok(stable_path)
    }

    /// Generate a one-time pairing session with 128-bit cryptographically secure token and 6-digit PIN fallback
    pub fn generate_pairing_session(&self) -> crate::net::PairingSession {
        let session = crate::net::PairingSession::new();
        *self.active_token.lock() = Some(session.clone());
        session
    }

    pub fn generate_pairing_token(&self) -> String {
        self.generate_pairing_session().token
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

    pub fn source_manager(&self) -> SourceManager {
        SourceManager::new(self.store.clone())
    }

    pub async fn fetch_unsplash(&self, config: &UnsplashConfig) -> Result<Vec<ImageRecord>> {
        let mgr = self.source_manager();
        let records = mgr.fetch_unsplash(config).await?;
        Ok(records)
    }

    pub async fn fetch_wallhaven(&self, query: Option<&str>, count: usize) -> Result<Vec<ImageRecord>> {
        let mgr = self.source_manager();
        let records = mgr.fetch_wallhaven(query, count).await?;
        Ok(records)
    }

    pub async fn fetch_bing_daily(&self) -> Result<ImageRecord> {
        let mgr = self.source_manager();
        let record = mgr.fetch_bing_daily().await?;
        Ok(record)
    }
}
