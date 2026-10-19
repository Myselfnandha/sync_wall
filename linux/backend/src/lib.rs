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

/// Helper to normalize xfconf properties into clean output IDs
/// Strips "monitor" prefix, rejects "screen\d*" entirely, and drops legacy "0" / "monitor0"
/// if named outputs (e.g. "eDP-1", "HDMI-1") are present.
pub fn normalize_xfconf_outputs<I, S>(properties: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut named_monitors = std::collections::BTreeSet::new();
    let mut numeric_monitors = std::collections::BTreeSet::new();

    for prop in properties {
        let s = prop.as_ref();
        for part in s.split('/') {
            // Reject any screen component (e.g. screen0)
            if part.starts_with("screen") {
                continue;
            }
            if let Some(stripped) = part.strip_prefix("monitor") {
                if stripped.chars().all(|c| c.is_ascii_digit()) {
                    numeric_monitors.insert(stripped.to_string());
                } else if !stripped.is_empty() {
                    named_monitors.insert(stripped.to_string());
                }
            }
        }
    }

    if !named_monitors.is_empty() {
        named_monitors.into_iter().collect()
    } else if !numeric_monitors.is_empty() {
        numeric_monitors.into_iter().collect()
    } else {
        vec!["default".to_string()]
    }
}

/// Detect active connected displays via xrandr query
fn detect_xrandr_outputs() -> Option<Vec<Output>> {
    let output = Command::new("xrandr").arg("--query").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut list = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == "connected" {
            let name = parts[0].to_string();
            let is_primary = parts.get(2).copied() == Some("primary");
            let mode_part = if is_primary { parts.get(3) } else { parts.get(2) };

            let (mut width, mut height) = (1920, 1080);
            if let Some(mode) = mode_part {
                if let Some(geom) = mode.split('+').next() {
                    let wh: Vec<&str> = geom.split('x').collect();
                    if wh.len() == 2 {
                        if let (Ok(w), Ok(h)) = (wh[0].parse(), wh[1].parse()) {
                            width = w;
                            height = h;
                        }
                    }
                }
            }
            list.push(Output {
                name,
                width,
                height,
                is_primary,
            });
        }
    }

    if list.is_empty() {
        None
    } else {
        if !list.iter().any(|o| o.is_primary) {
            list[0].is_primary = true;
        }
        Some(list)
    }
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
        let target_seg = if output_name.is_empty() {
            String::new()
        } else {
            format!("/monitor{}/", output_name)
        };

        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.contains("last-image") {
                    if output_name.is_empty() || trimmed.contains(&target_seg) {
                        matched.push(trimmed.to_string());
                    } else if (output_name == "0" || output_name == "eDP-1" || output_name == "default")
                        && trimmed.contains("/monitor0/")
                    {
                        // Also alias to monitor0 for primary display so legacy xfce components get updated
                        matched.push(trimmed.to_string());
                    }
                }
            }
        }

        // Fallback: if no properties enumerated yet, generate standard workspace paths
        if matched.is_empty() {
            let monitor_seg = if output_name.is_empty() {
                "monitor0".to_string()
            } else {
                format!("monitor{}", output_name)
            };
            for ws in 0..8 {
                matched.push(format!("/backdrop/screen0/{}/workspace{}/last-image", monitor_seg, ws));
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
        // 1. Try xrandr first for real connected hardware outputs and accurate resolution
        if let Some(xrandr_outs) = detect_xrandr_outputs() {
            return Ok(xrandr_outs);
        }

        // 2. Fallback: Parse and normalize properties from xfconf-query
        let props = self.find_monitor_properties("");
        let clean_names = normalize_xfconf_outputs(&props);

        let mut outputs = Vec::new();
        for (i, name) in clean_names.into_iter().enumerate() {
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

            // Set image style to 5 (zoomed / fill) across all workspaces
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

    #[test]
    fn test_normalize_xfconf_outputs() {
        let raw_props = vec![
            "/backdrop/screen0/monitor0/workspace0/last-image",
            "/backdrop/screen0/monitoreDP-1/workspace0/last-image",
            "/backdrop/screen0/monitoreDP-1/workspace1/last-image",
            "/backdrop/screen0/monitorHDMI-1/workspace0/last-image",
        ];
        let outputs = normalize_xfconf_outputs(&raw_props);
        // "screen0" must be discarded
        assert!(!outputs.contains(&"screen0".to_string()));
        // "monitor0" must be discarded because named outputs exist
        assert!(!outputs.contains(&"monitor0".to_string()));
        assert!(!outputs.contains(&"0".to_string()));
        // Clean named outputs must be present
        assert_eq!(outputs, vec!["HDMI-1".to_string(), "eDP-1".to_string()]);

        // When only legacy monitor0 exists
        let fallback_props = vec!["/backdrop/screen0/monitor0/workspace0/last-image"];
        let fallback_outs = normalize_xfconf_outputs(&fallback_props);
        assert_eq!(fallback_outs, vec!["0".to_string()]);
    }
}
