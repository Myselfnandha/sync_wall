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
