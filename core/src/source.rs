use std::sync::Arc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::ImageRecord;
use crate::store::Store;

pub const TABLISS_OFFICIAL_UNSPLASH_COLLECTION: &str = "1053828";

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("Network request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Store error: {0}")]
    Store(#[from] crate::store::StoreError),
    #[error("API error: {0}")]
    Api(String),
    #[error("No images found in response")]
    NoImagesFound,
}

pub type Result<T> = std::result::Result<T, SourceError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsplashConfig {
    pub access_key: Option<String>,
    pub query: Option<String>,
    pub collection: Option<String>,
    pub orientation: Option<String>, // "landscape" | "portrait" | "squarish"
    pub count: usize,
}

impl Default for UnsplashConfig {
    fn default() -> Self {
        Self {
            access_key: None,
            query: None,
            collection: Some(TABLISS_OFFICIAL_UNSPLASH_COLLECTION.to_string()),
            orientation: Some("landscape".to_string()),
            count: 1,
        }
    }
}

#[derive(Deserialize, Debug)]
struct UnsplashPhotoUrls {
    raw: Option<String>,
    full: Option<String>,
    regular: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct UnsplashUser {
    name: Option<String>,
    username: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct UnsplashPhoto {
    id: String,
    description: Option<String>,
    alt_description: Option<String>,
    urls: UnsplashPhotoUrls,
    user: Option<UnsplashUser>,
}

#[derive(Deserialize, Debug)]
struct WallhavenSearchResponse {
    data: Vec<WallhavenPhoto>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct WallhavenPhoto {
    id: String,
    path: String,
    dimension_x: u32,
    dimension_y: u32,
}

#[derive(Deserialize, Debug)]
struct BingArchiveResponse {
    images: Vec<BingImage>,
}

#[derive(Deserialize, Debug)]
struct BingImage {
    url: String,
    copyright: Option<String>,
}

pub struct SourceManager {
    store: Arc<Store>,
    client: reqwest::Client,
}

impl SourceManager {
    pub fn new(store: Arc<Store>) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("sync_wall/0.1.0")
            .build()
            .unwrap_or_default();
        Self { store, client }
    }

    /// Fetch photos from Unsplash following Tabliss's integration model.
    /// Uses collection 1053828 (Tabliss curated wallpapers) by default.
    pub async fn fetch_unsplash(&self, config: &UnsplashConfig) -> Result<Vec<ImageRecord>> {
        let access_key = config.access_key.clone()
            .or_else(|| std::env::var("UNSPLASH_ACCESS_KEY").ok())
            .or_else(|| std::env::var("UNSPLASH_API_KEY").ok());

        let access_key = match access_key {
            Some(k) if !k.trim().is_empty() => k,
            _ => {
                return Err(SourceError::Api(
                    "Unsplash API requires an access key. Set UNSPLASH_ACCESS_KEY environment variable or pass --key.".to_string()
                ));
            }
        };

        let mut url = format!("https://api.unsplash.com/photos/random?count={}", config.count.clamp(1, 10));
        if let Some(ref q) = config.query {
            url.push_str(&format!("&query={}", urlencoding::encode(q)));
        } else if let Some(ref coll) = config.collection {
            url.push_str(&format!("&collections={}", urlencoding::encode(coll)));
        }

        if let Some(ref ori) = config.orientation {
            url.push_str(&format!("&orientation={}", ori));
        }

        let resp = self.client.get(&url)
            .header("Authorization", format!("Client-ID {}", access_key))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SourceError::Api(format!("Unsplash API error ({}): {}", status, body)));
        }

        let photos: Vec<UnsplashPhoto> = resp.json().await?;
        if photos.is_empty() {
            return Err(SourceError::NoImagesFound);
        }

        let mut records = Vec::new();
        for p in photos {
            let download_url = p.urls.full.or(p.urls.regular).or(p.urls.raw)
                .ok_or_else(|| SourceError::Api("Missing download URL in Unsplash response".to_string()))?;

            let img_resp = self.client.get(&download_url).send().await?;
            let bytes = img_resp.bytes().await?;

            let author = p.user.and_then(|u| u.name).unwrap_or_else(|| "Unknown".to_string());
            let title = p.alt_description.or(p.description).unwrap_or_else(|| format!("unsplash_{}", p.id));
            let _filename = format!("unsplash_{}.jpg", p.id);

            let mut tags = vec!["unsplash".to_string()];
            if let Some(ref q) = config.query {
                tags.push(q.clone());
            }

            let source_label = format!("{} (by {})", title, author);
            let rec = self.store.ingest_bytes(&bytes, &source_label, tags)?;
            records.push(rec);
        }

        Ok(records)
    }

    /// Fetch free high-res wallpapers from Wallhaven without requiring any API key.
    pub async fn fetch_wallhaven(&self, query: Option<&str>, count: usize) -> Result<Vec<ImageRecord>> {
        let q = query.unwrap_or("nature");
        let url = format!(
            "https://wallhaven.cc/api/v1/search?q={}&sorting=random&purity=100",
            urlencoding::encode(q)
        );

        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(SourceError::Api(format!("Wallhaven request failed: {}", resp.status())));
        }

        let search_res: WallhavenSearchResponse = resp.json().await?;
        if search_res.data.is_empty() {
            return Err(SourceError::NoImagesFound);
        }

        let mut records = Vec::new();
        for p in search_res.data.into_iter().take(count.clamp(1, 10)) {
            let img_resp = self.client.get(&p.path).send().await?;
            let bytes = img_resp.bytes().await?;

            let filename = format!("wallhaven_{}.jpg", p.id);
            let tags = vec!["wallhaven".to_string(), q.to_string()];

            let rec = self.store.ingest_bytes(&bytes, &filename, tags)?;
            records.push(rec);
        }

        Ok(records)
    }

    /// Fetch Bing Daily Wallpaper (free, no API key required)
    pub async fn fetch_bing_daily(&self) -> Result<ImageRecord> {
        let url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US";
        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(SourceError::Api(format!("Bing archive request failed: {}", resp.status())));
        }

        let res: BingArchiveResponse = resp.json().await?;
        let img = res.images.into_iter().next()
            .ok_or(SourceError::NoImagesFound)?;

        let full_url = if img.url.starts_with("http") {
            img.url
        } else {
            format!("https://www.bing.com{}", img.url)
        };

        let img_resp = self.client.get(&full_url).send().await?;
        let bytes = img_resp.bytes().await?;

        let label = img.copyright.unwrap_or_else(|| "Bing Daily Wallpaper".to_string());
        let rec = self.store.ingest_bytes(&bytes, &label, vec!["bing".to_string(), "daily".to_string()])?;
        Ok(rec)
    }
}

// Simple urlencoding helper to avoid another heavy dependency
mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut result = String::new();
        for c in s.chars() {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
                result.push(c);
            } else if c == ' ' {
                result.push('+');
            } else {
                for b in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", b));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlencode() {
        assert_eq!(urlencoding::encode("minimalist nature"), "minimalist+nature");
        assert_eq!(urlencoding::encode("test/query?#"), "test%2Fquery%3F%23");
        assert_eq!(urlencoding::encode("simple-text_123.valid~"), "simple-text_123.valid~");
    }

    #[test]
    fn test_unsplash_config_defaults() {
        let cfg = UnsplashConfig::default();
        assert_eq!(cfg.collection.as_deref(), Some(TABLISS_OFFICIAL_UNSPLASH_COLLECTION));
        assert_eq!(cfg.collection.as_deref(), Some("1053828"));
        assert_eq!(cfg.orientation.as_deref(), Some("landscape"));
        assert_eq!(cfg.count, 1);
        assert!(cfg.query.is_none());
        assert!(cfg.access_key.is_none());
    }

    #[test]
    fn test_unsplash_json_parse() {
        let raw_json = r#"[
            {
                "id": "abc123xyz",
                "description": "Misty mountain forest at sunrise",
                "alt_description": "Scenic view of trees in fog",
                "urls": {
                    "raw": "https://images.unsplash.com/raw/test.jpg",
                    "full": "https://images.unsplash.com/full/test.jpg",
                    "regular": "https://images.unsplash.com/regular/test.jpg"
                },
                "user": {
                    "name": "Jane Photographer",
                    "username": "janephoto"
                }
            }
        ]"#;

        let photos: Vec<UnsplashPhoto> = serde_json::from_str(raw_json).expect("valid unsplash json");
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].id, "abc123xyz");
        assert_eq!(photos[0].urls.full.as_deref(), Some("https://images.unsplash.com/full/test.jpg"));
        assert_eq!(photos[0].user.as_ref().and_then(|u| u.name.as_deref()), Some("Jane Photographer"));
    }

    #[test]
    fn test_wallhaven_json_parse() {
        let raw_json = r#"{
            "data": [
                {
                    "id": "w91234",
                    "path": "https://w.wallhaven.cc/full/w9/wallhaven-w91234.jpg",
                    "dimension_x": 3840,
                    "dimension_y": 2160
                }
            ]
        }"#;

        let res: WallhavenSearchResponse = serde_json::from_str(raw_json).expect("valid wallhaven json");
        assert_eq!(res.data.len(), 1);
        assert_eq!(res.data[0].id, "w91234");
        assert_eq!(res.data[0].dimension_x, 3840);
        assert_eq!(res.data[0].dimension_y, 2160);
    }

    #[test]
    fn test_bing_archive_json_parse() {
        let raw_json = r#"{
            "images": [
                {
                    "url": "/th?id=OHR.VeniceCanal_EN-US12345_1920x1080.jpg",
                    "copyright": "Grand Canal in Venice (© Test Photographer)"
                }
            ]
        }"#;

        let res: BingArchiveResponse = serde_json::from_str(raw_json).expect("valid bing json");
        assert_eq!(res.images.len(), 1);
        assert_eq!(res.images[0].copyright.as_deref(), Some("Grand Canal in Venice (© Test Photographer)"));
    }
}

