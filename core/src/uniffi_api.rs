use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum SyncWallFfiError {
    #[error("Internal error: {msg}")]
    Internal { msg: String },
    #[error("IO error: {msg}")]
    Io { msg: String },
    #[error("Not found: {msg}")]
    NotFound { msg: String },
    #[error("Network error: {msg}")]
    Network { msg: String },
    #[error("Pairing failed: {msg}")]
    PairingFailed { msg: String },
    #[error("Sync failed: {msg}")]
    SyncFailed { msg: String },
}

impl From<crate::EngineError> for SyncWallFfiError {
    fn from(e: crate::EngineError) -> Self {
        match e {
            crate::EngineError::NotFound(s) => SyncWallFfiError::NotFound { msg: s },
            crate::EngineError::Io(e) => SyncWallFfiError::Io { msg: e.to_string() },
            crate::EngineError::Net(e) => SyncWallFfiError::Network { msg: e.to_string() },
            crate::EngineError::Reqwest(e) => SyncWallFfiError::Network { msg: e.to_string() },
            other => SyncWallFfiError::Internal { msg: other.to_string() },
        }
    }
}

impl From<reqwest::Error> for SyncWallFfiError {
    fn from(e: reqwest::Error) -> Self {
        SyncWallFfiError::Network { msg: e.to_string() }
    }
}

impl From<crate::store::StoreError> for SyncWallFfiError {
    fn from(e: crate::store::StoreError) -> Self {
        SyncWallFfiError::Internal { msg: e.to_string() }
    }
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct WallpaperMetadata {
    pub hash: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub mime: String,
    pub file_size: u64,
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct PairingResult {
    pub success: bool,
    pub server_name: String,
    pub server_fingerprint: String,
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct SyncReport {
    pub applied_images: u32,
    pub applied_assignments: u32,
    pub new_sequence: u64,
}

#[derive(uniffi::Object)]
pub struct SyncWallEngineBinding {
    inner: Arc<crate::SyncWallEngine>,
}

#[uniffi::export]
impl SyncWallEngineBinding {
    #[uniffi::constructor]
    pub fn new(base_dir: String, device_name: String) -> Result<Arc<Self>, SyncWallFfiError> {
        let engine = crate::SyncWallEngine::open(&base_dir, &device_name)?;
        Ok(Arc::new(Self {
            inner: Arc::new(engine),
        }))
    }

    pub fn get_device_id(&self) -> String {
        self.inner.device_id().to_string()
    }

    pub fn ingest_bytes(&self, bytes: Vec<u8>, name: String) -> Result<String, SyncWallFfiError> {
        let record = self.inner.ingest_bytes(&bytes, &name, vec![])?;
        Ok(record.hash.to_hex())
    }

    pub fn list_wallpapers(&self) -> Result<Vec<WallpaperMetadata>, SyncWallFfiError> {
        let images = self.inner.list_images()?;
        Ok(images
            .into_iter()
            .map(|img| {
                let size = self
                    .inner
                    .store()
                    .blob_path(&img.hash)
                    .metadata()
                    .map(|m| m.len())
                    .unwrap_or(0);
                WallpaperMetadata {
                    hash: img.hash.to_hex(),
                    name: img.source_name,
                    width: img.width,
                    height: img.height,
                    mime: img.mime,
                    file_size: size,
                }
            })
            .collect())
    }

    pub fn set_wallpaper(
        &self,
        screen_name: String,
        image_hash: String,
        fit: String,
    ) -> Result<String, SyncWallFfiError> {
        let hash = crate::model::Hash::from_str(&image_hash)
            .map_err(|e| SyncWallFfiError::Internal { msg: e })?;
        let fit_mode = match fit.to_lowercase().as_str() {
            "fit" => crate::model::Fit::Fit,
            "center" => crate::model::Fit::CropCenter,
            _ => crate::model::Fit::Fill,
        };
        let assign = self.inner.set_wallpaper(&screen_name, hash, fit_mode, None)?;
        Ok(assign.image_hash.to_hex())
    }

    pub fn render_wallpaper(
        &self,
        screen_name: String,
        target_w: u32,
        target_h: u32,
        format: String,
    ) -> Result<String, SyncWallFfiError> {
        let out_fmt = match format.to_lowercase().as_str() {
            "png" => crate::model::OutFormat::Png,
            _ => crate::model::OutFormat::WebpLossy(90),
        };
        let path = self
            .inner
            .render_wallpaper_for_screen(&screen_name, (target_w, target_h), out_fmt)?;
        Ok(path.to_string_lossy().to_string())
    }

    pub fn get_active_wallpaper_path(&self, screen_name: String) -> Option<String> {
        let current_dir = self.inner.base_dir().join("current");
        let png = current_dir.join(format!("{}.png", screen_name));
        if png.exists() {
            return Some(png.to_string_lossy().to_string());
        }
        let webp = current_dir.join(format!("{}.webp", screen_name));
        if webp.exists() {
            return Some(webp.to_string_lossy().to_string());
        }
        None
    }

    pub fn pair_with_server(
        &self,
        server_url: String,
        token: String,
        device_name: String,
        screen_w: u32,
        screen_h: u32,
    ) -> Result<PairingResult, SyncWallFfiError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| SyncWallFfiError::Internal { msg: e.to_string() })?;

        rt.block_on(async {
            let client = reqwest::Client::builder().build()?;
            let pair_url = format!("{}/v1/pair", server_url.trim_end_matches('/'));
            let req = crate::net::PairRequest {
                token,
                client_fingerprint: self.inner.device_id().to_string(),
                device_name,
                kind: crate::model::DeviceKind::Android,
                screens: vec![crate::model::Screen {
                    name: "mobile_screen".into(),
                    width: screen_w,
                    height: screen_h,
                    scale: 1.0,
                }],
            };

            let resp = client.post(&pair_url).json(&req).send().await?;
            if !resp.status().is_success() {
                return Err(SyncWallFfiError::PairingFailed {
                    msg: format!("Server returned HTTP {}", resp.status()),
                });
            }

            let pair_resp: crate::net::PairResponse = resp.json().await?;
            if !pair_resp.success {
                return Err(SyncWallFfiError::PairingFailed {
                    msg: "Pairing rejected by server".into(),
                });
            }

            // Save server to trusted devices
            let server_record = crate::model::DeviceRecord {
                id: pair_resp.server_fingerprint.clone(),
                name: pair_resp.server_name.clone(),
                kind: crate::model::DeviceKind::Linux,
                screens: vec![],
                hlc: crate::model::Hlc::new(
                    chrono::Utc::now().timestamp_millis() as u64,
                    0,
                    self.inner.device_id(),
                ),
                deleted: false,
            };
            self.inner.store().upsert_device(&server_record)?;

            Ok(PairingResult {
                success: true,
                server_name: pair_resp.server_name,
                server_fingerprint: pair_resp.server_fingerprint,
            })
        })
    }

    pub fn sync_with_peer(&self, peer_url: String) -> Result<SyncReport, SyncWallFfiError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| SyncWallFfiError::Internal { msg: e.to_string() })?;

        rt.block_on(async {
            let report = self.inner.sync_with_peer(&peer_url).await?;
            let (last_seq, _) = self.inner.store().get_peer_cursor(&peer_url)?;
            Ok(SyncReport {
                applied_images: report.applied_images as u32,
                applied_assignments: report.applied_assignments.len() as u32,
                new_sequence: last_seq,
            })
        })
    }

    pub fn fetch_bing_daily(&self) -> Result<WallpaperMetadata, SyncWallFfiError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| SyncWallFfiError::Internal { msg: e.to_string() })?;

        rt.block_on(async {
            let record = self.inner.fetch_bing_daily().await?;
            let size = self
                .inner
                .store()
                .blob_path(&record.hash)
                .metadata()
                .map(|m| m.len())
                .unwrap_or(0);
            Ok(WallpaperMetadata {
                hash: record.hash.to_hex(),
                name: record.source_name,
                width: record.width,
                height: record.height,
                mime: record.mime,
                file_size: size,
            })
        })
    }
}
