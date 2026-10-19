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

    pub fn get_device(&self, id: &str) -> Result<Option<DeviceRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, kind, screens, hlc, deleted FROM devices WHERE id = ? AND deleted = 0",
        )?;
        let mut rows = stmt.query_map([id], |row| {
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

        if let Some(r) = rows.next() {
            Ok(Some(r?))
        } else {
            Ok(None)
        }
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

    #[test]
    fn test_batch_replay_idempotence() {
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();
        let store1 = Store::open(dir1.path(), "device-1").unwrap();
        let store2 = Store::open(dir2.path(), "device-2").unwrap();

        let bytes = make_test_image();
        let img1 = store1.ingest_bytes(&bytes, "pic1.png", vec!["test".into()]).unwrap();
        store1.set_assignment("device-1", "eDP-1", img1.hash, Fit::Fill, Some((0.5, 0.5))).unwrap();

        let batch = store1.changes_since(0, 100).unwrap();

        // First application
        let report1 = store2.apply_batch(&batch).unwrap();
        assert_eq!(report1.applied_images, 1);
        assert_eq!(report1.applied_assignments.len(), 1);

        let img_before = store2.list_images().unwrap();
        let assign_before = store2.list_assignments().unwrap();

        // Second application of the exact same batch: MUST be an idempotent no-op
        let report2 = store2.apply_batch(&batch).unwrap();
        assert_eq!(report2.applied_images, 0);
        assert_eq!(report2.applied_assignments.len(), 0);

        let img_after = store2.list_images().unwrap();
        let assign_after = store2.list_assignments().unwrap();

        assert_eq!(img_before, img_after);
        assert_eq!(assign_before, assign_after);
    }

    #[test]
    fn test_concurrent_edits_convergence_shuffled_order() {
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();
        let store1 = Store::open(dir1.path(), "device-1").unwrap();
        let store2 = Store::open(dir2.path(), "device-2").unwrap();

        let bytes1 = make_test_image();
        let mut bytes2 = bytes1.clone();
        bytes2.push(99); // different bytes -> different BLAKE3 hash

        let img1 = store1.ingest_bytes(&bytes1, "img1.png", vec!["nature".into()]).unwrap();
        let img2 = store2.ingest_bytes(&bytes2, "img2.png", vec!["city".into()]).unwrap();

        // Concurrent edit on same screen "eDP-1"
        // device-1 sets img1 on eDP-1 with HLC 1
        store1.set_assignment("device-1", "eDP-1", img1.hash, Fit::Fill, None).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        // device-1 updates eDP-1 with higher HLC
        let assign_winner = store1.set_assignment("device-1", "eDP-1", img1.hash, Fit::CropCenter, Some((0.2, 0.8))).unwrap();

        // device-2 sets its own independent assignment for HDMI-1
        store2.set_assignment("device-2", "HDMI-1", img2.hash, Fit::Fill, None).unwrap();

        // Generate batches from both
        let batch1 = store1.changes_since(0, 100).unwrap();
        let batch2 = store2.changes_since(0, 100).unwrap();

        // Apply in shuffled/cross order: store1 applies batch2, store2 applies batch1
        store1.apply_batch(&batch2).unwrap();
        store2.apply_batch(&batch1).unwrap();

        // Both stores MUST reach exact identical state
        let mut images1 = store1.list_images().unwrap();
        let mut images2 = store2.list_images().unwrap();
        images1.sort_by_key(|i| i.hash.to_hex());
        images2.sort_by_key(|i| i.hash.to_hex());
        assert_eq!(images1, images2);

        let mut assign1 = store1.list_assignments().unwrap();
        let mut assign2 = store2.list_assignments().unwrap();
        assign1.sort_by_key(|a| (a.device_id.clone(), a.screen_name.clone()));
        assign2.sort_by_key(|a| (a.device_id.clone(), a.screen_name.clone()));
        assert_eq!(assign1, assign2);

        // Verify the LWW winner persisted on both nodes
        let edp1 = store2.get_assignment("device-1", "eDP-1").unwrap().unwrap();
        assert_eq!(edp1.hlc, assign_winner.hlc);
        assert_eq!(edp1.fit, Fit::CropCenter);
        assert_eq!(edp1.focus, Some((0.2, 0.8)));
    }
}
