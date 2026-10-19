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
