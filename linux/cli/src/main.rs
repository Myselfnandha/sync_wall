use std::path::PathBuf;
use clap::{Parser, Subcommand};
use qrcode::render::unicode;
use qrcode::QrCode;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use sync_wall_backend::detect_backend;
use sync_wall_core::model::{Fit, Hash, OutFormat};
use sync_wall_core::SyncWallEngine;

#[derive(Parser, Debug)]
#[command(name = "sync_wall", author, version, about = "sync_wall desktop CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Ingest images into the wallpaper store
    Add {
        /// Image file paths to add
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Optional tags to associate with the images
        #[arg(short, long)]
        tags: Vec<String>,
    },

    /// Set desktop wallpaper
    Set {
        /// Image BLAKE3 hash or direct file path
        target: String,

        /// Specific monitor/output name (defaults to all)
        #[arg(short, long)]
        screen: Option<String>,
    },

    /// List all cataloged wallpapers
    List,

    /// List active monitors/screens
    Screens,

    /// Generate pairing token and terminal QR code for phone pairing
    Pair {
        /// Custom host/IP address to advertise (defaults to local hostname)
        #[arg(long)]
        host: Option<String>,

        /// Port where sync_walld is listening
        #[arg(short, long, default_value_t = 7889)]
        port: u16,
    },

    /// Fetch wallpapers from online sources (Unsplash, Wallhaven, Bing)
    Fetch {
        #[command(subcommand)]
        source: FetchSource,
    },

    /// Trigger sync with a peer device
    Sync {
        /// Peer URL (e.g. https://192.168.1.50:7889)
        url: String,
    },

    /// Check daemon status and doctor diagnostics
    Status,
}

#[derive(Subcommand, Debug)]
pub enum FetchSource {
    /// Fetch from Unsplash (using Tabliss official collection 1053828 or custom query)
    Unsplash {
        /// Search keywords (e.g. nature, minimalist, neon)
        #[arg(short, long)]
        query: Option<String>,

        /// Collection ID (defaults to Tabliss official collection 1053828)
        #[arg(short, long)]
        collection: Option<String>,

        /// Orientation: landscape (default) or portrait
        #[arg(short, long, default_value = "landscape")]
        orientation: String,

        /// Number of photos to fetch (1-10)
        #[arg(short, long, default_value_t = 1)]
        count: usize,

        /// Unsplash Access Key (or use UNSPLASH_ACCESS_KEY env var)
        #[arg(long)]
        key: Option<String>,

        /// Immediately apply first fetched wallpaper to desktop
        #[arg(long)]
        apply: bool,
    },

    /// Fetch free high-res wallpapers from Wallhaven (no API key required)
    Wallhaven {
        /// Search keywords (e.g. cyberpunk, mountains, anime)
        #[arg(short, long)]
        query: Option<String>,

        /// Number of photos to fetch (1-10)
        #[arg(short, long, default_value_t = 1)]
        count: usize,

        /// Immediately apply first fetched wallpaper to desktop
        #[arg(long)]
        apply: bool,
    },

    /// Fetch Bing Daily Wallpaper (free, no API key required)
    Bing {
        /// Immediately apply to desktop
        #[arg(long)]
        apply: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Check if daemon Unix socket exists
    let socket_path = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
        .join("sync_wall/sync_walld.sock");

    if socket_path.exists() {
        if let Ok(mut stream) = UnixStream::connect(&socket_path).await {
            match handle_via_daemon(&cli.command, &mut stream).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    eprintln!("Daemon RPC returned error: {}, falling back to local engine...", e);
                }
            }
        }
    }

    // Fallback: in-process direct execution
    handle_standalone(&cli.command).await?;
    Ok(())
}

async fn handle_via_daemon(cmd: &Commands, stream: &mut UnixStream) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = stream.split();
    let mut lines = BufReader::new(reader).lines();

    match cmd {
        Commands::Status => {
            let req = json!({ "id": 1, "method": "status", "params": {} });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(res) = v.get("result") {
                    println!("\x1b[1;32msync_wall status (connected to daemon)\x1b[0m");
                    println!("Device ID:   {}", res["device_id"].as_str().unwrap_or(""));
                    println!("Backend:     {}", res["backend"].as_str().unwrap_or(""));
                    println!("Outputs:     {}", res["outputs"]);
                    println!("Total Images:{}", res["image_count"]);
                    println!("Devices:     {}", res["device_count"]);
                }
            }
        }
        Commands::Screens => {
            let req = json!({ "id": 1, "method": "list_outputs", "params": {} });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(outputs) = v.get("result").and_then(|r| r.as_array()) {
                    println!("\x1b[1mDetected Screen Outputs:\x1b[0m");
                    for out in outputs {
                        println!(" - {} ({}x{}, primary: {})", 
                            out["name"].as_str().unwrap_or(""),
                            out["width"],
                            out["height"],
                            out["is_primary"]
                        );
                    }
                }
            }
        }
        Commands::Add { files, tags } => {
            let paths_str: Vec<String> = files.iter()
                .map(|p| p.canonicalize().unwrap_or_else(|_| p.clone()).to_string_lossy().to_string())
                .collect();
            let req = json!({
                "id": 1,
                "method": "add",
                "params": { "paths": paths_str, "tags": tags }
            });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(added) = v.get("result").and_then(|r| r.as_array()) {
                    println!("Successfully added {} image(s):", added.len());
                    for img in added {
                        println!(" - \x1b[36m{}\x1b[0m ({}x{}, {})", 
                            img["hash"].as_str().unwrap_or(""),
                            img["width"],
                            img["height"],
                            img["source_name"].as_str().unwrap_or("")
                        );
                    }
                }
            }
        }
        Commands::List => {
            let req = json!({ "id": 1, "method": "list_images", "params": {} });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(images) = v.get("result").and_then(|r| r.as_array()) {
                    println!("\x1b[1mCataloged Wallpapers ({}):\x1b[0m", images.len());
                    for img in images {
                        println!(" - \x1b[36m{}\x1b[0m {}x{} [{}] {}", 
                            img["hash"].as_str().unwrap_or(""),
                            img["width"],
                            img["height"],
                            img["mime"].as_str().unwrap_or(""),
                            img["source_name"].as_str().unwrap_or("")
                        );
                    }
                }
            }
        }
        Commands::Set { target, screen } => {
            let hash = if let Ok(h) = target.parse::<Hash>() {
                h.to_string()
            } else {
                let path = PathBuf::from(target);
                if !path.exists() {
                    return Err(format!("File '{}' not found", target).into());
                }
                let bytes = std::fs::read(&path)?;
                Hash::compute(&bytes).to_string()
            };

            let req = json!({
                "id": 1,
                "method": "set",
                "params": { "hash": hash, "screen": screen }
            });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(err) = v.get("error").and_then(|e| e.as_str()) {
                    return Err(err.to_string().into());
                }
                println!("\x1b[32mWallpaper applied successfully!\x1b[0m");
            }
        }
        Commands::Pair { host, port } => {
            let req = json!({ "id": 1, "method": "pair_token", "params": {} });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(res) = v.get("result") {
                    let token = res["token"].as_str().unwrap_or("");
                    let pin = res["pin"].as_str().unwrap_or("");
                    let device_id = res["device_id"].as_str().unwrap_or("");
                    let host_str = host.clone().unwrap_or_else(|| "127.0.0.1".to_string());

                    let qr_payload = json!({
                        "host": host_str,
                        "port": port,
                        "token": token,
                        "server_cert": device_id
                    }).to_string();

                    render_pairing_qr(&qr_payload, token, pin);
                }
            }
        }
        Commands::Sync { url } => {
            let req = json!({ "id": 1, "method": "sync_peer", "params": { "url": url } });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                println!("Sync report: {:?}", v.get("result"));
            }
        }
        Commands::Fetch { source } => {
            let req_params = match source {
                FetchSource::Unsplash { query, collection, orientation, count, key, apply } => {
                    json!({
                        "provider": "unsplash",
                        "query": query,
                        "collection": collection,
                        "orientation": orientation,
                        "count": count,
                        "key": key,
                        "apply": apply,
                    })
                }
                FetchSource::Wallhaven { query, count, apply } => {
                    json!({
                        "provider": "wallhaven",
                        "query": query,
                        "count": count,
                        "apply": apply,
                    })
                }
                FetchSource::Bing { apply } => {
                    json!({
                        "provider": "bing",
                        "apply": apply,
                    })
                }
            };
            let req = json!({ "id": 1, "method": "fetch", "params": req_params });
            writer.write_all(format!("{}\n", req).as_bytes()).await?;
            if let Some(line) = lines.next_line().await? {
                let v: serde_json::Value = serde_json::from_str(&line)?;
                if let Some(err) = v.get("error").and_then(|e| e.as_str()) {
                    return Err(err.to_string().into());
                }
                if let Some(records) = v.get("result").and_then(|r| r.as_array()) {
                    println!("Daemon fetched {} wallpaper(s):", records.len());
                    for rec in records {
                        println!(" - \x1b[36m{}\x1b[0m ({}x{}, {})",
                            rec["hash"].as_str().unwrap_or(""),
                            rec["width"],
                            rec["height"],
                            rec["source_name"].as_str().unwrap_or("")
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

async fn handle_standalone(cmd: &Commands) -> Result<(), Box<dyn std::error::Error>> {
    let base_data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("sync_wall");
    let engine = SyncWallEngine::open(&base_data_dir, "sync_wall_cli")?;
    let backend = detect_backend();

    match cmd {
        Commands::Status => {
            println!("\x1b[1;33msync_wall status (standalone)\x1b[0m");
            println!("Device ID:   {}", engine.device_id());
            println!("Backend:     {}", backend.name());
            let outputs = backend.list_outputs()?;
            println!("Outputs:     {:?}", outputs.iter().map(|o| &o.name).collect::<Vec<_>>());
            let images = engine.list_images()?;
            println!("Total Images:{}", images.len());
        }
        Commands::Screens => {
            let outputs = backend.list_outputs()?;
            println!("\x1b[1mDetected Screen Outputs:\x1b[0m");
            for out in outputs {
                println!(" - {} ({}x{}, primary: {})", out.name, out.width, out.height, out.is_primary);
            }
        }
        Commands::Add { files, tags } => {
            for f in files {
                let rec = engine.ingest_file(f, tags.clone())?;
                println!("Added \x1b[36m{}\x1b[0m ({}x{}, {})", rec.hash, rec.width, rec.height, rec.source_name);
            }
        }
        Commands::List => {
            let images = engine.list_images()?;
            println!("\x1b[1mCataloged Wallpapers ({}):\x1b[0m", images.len());
            for img in images {
                println!(" - \x1b[36m{}\x1b[0m {}x{} [{}] {}", img.hash, img.width, img.height, img.mime, img.source_name);
            }
        }
        Commands::Set { target, screen } => {
            let hash = if let Ok(h) = target.parse::<Hash>() {
                h
            } else {
                let p = PathBuf::from(target);
                let rec = engine.ingest_file(&p, vec![])?;
                rec.hash
            };

            let outputs = backend.list_outputs()?;
            let target_outputs = if let Some(s) = screen {
                outputs.into_iter().filter(|o| &o.name == s).collect()
            } else {
                outputs
            };

            for out in &target_outputs {
                engine.set_wallpaper(&out.name, hash, Fit::Fill, Some((0.5, 0.5)))?;
                let rendered = engine.render_wallpaper_for_screen(&out.name, (out.width, out.height), OutFormat::Png)?;
                backend.set(&out.name, &rendered, Fit::Fill)?;
                println!("Applied to screen \x1b[32m{}\x1b[0m -> {:?}", out.name, rendered);
            }
        }
        Commands::Pair { host, port } => {
            let session = engine.generate_pairing_session();
            let host_str = host.clone().unwrap_or_else(|| "127.0.0.1".to_string());
            let qr_payload = json!({
                "host": host_str,
                "port": port,
                "token": session.token,
                "server_cert": engine.device_id()
            }).to_string();

            render_pairing_qr(&qr_payload, &session.token, &session.pin);
        }
        Commands::Fetch { source } => {
            match source {
                FetchSource::Unsplash { query, collection, orientation, count, key, apply } => {
                    let config = sync_wall_core::source::UnsplashConfig {
                        access_key: key.clone(),
                        query: query.clone(),
                        collection: collection.clone(),
                        orientation: Some(orientation.clone()),
                        count: *count,
                    };
                    println!("Fetching from Unsplash (Tabliss integration)...");
                    let records = engine.fetch_unsplash(&config).await?;
                    println!("Successfully fetched {} photo(s) from Unsplash:", records.len());
                    for rec in &records {
                        println!(" - \x1b[36m{}\x1b[0m ({}x{}, {})", rec.hash, rec.width, rec.height, rec.source_name);
                    }
                    if *apply {
                        if let Some(first) = records.first() {
                            let outputs = backend.list_outputs()?;
                            for out in &outputs {
                                engine.set_wallpaper(&out.name, first.hash, Fit::Fill, Some((0.5, 0.5)))?;
                                let rendered = engine.render_wallpaper_for_screen(&out.name, (out.width, out.height), OutFormat::Png)?;
                                backend.set(&out.name, &rendered, Fit::Fill)?;
                                println!("Applied \x1b[32m{}\x1b[0m to {}", first.hash, out.name);
                            }
                        }
                    }
                }
                FetchSource::Wallhaven { query, count, apply } => {
                    println!("Fetching from Wallhaven (query: {:?})...", query.as_deref().unwrap_or("nature"));
                    let records = engine.fetch_wallhaven(query.as_deref(), *count).await?;
                    println!("Successfully fetched {} photo(s) from Wallhaven:", records.len());
                    for rec in &records {
                        println!(" - \x1b[36m{}\x1b[0m ({}x{}, {})", rec.hash, rec.width, rec.height, rec.source_name);
                    }
                    if *apply {
                        if let Some(first) = records.first() {
                            let outputs = backend.list_outputs()?;
                            for out in &outputs {
                                engine.set_wallpaper(&out.name, first.hash, Fit::Fill, Some((0.5, 0.5)))?;
                                let rendered = engine.render_wallpaper_for_screen(&out.name, (out.width, out.height), OutFormat::Png)?;
                                backend.set(&out.name, &rendered, Fit::Fill)?;
                                println!("Applied \x1b[32m{}\x1b[0m to {}", first.hash, out.name);
                            }
                        }
                    }
                }
                FetchSource::Bing { apply } => {
                    println!("Fetching Bing Daily Wallpaper...");
                    let rec = engine.fetch_bing_daily().await?;
                    println!("Successfully fetched Bing Daily: \x1b[36m{}\x1b[0m ({})", rec.hash, rec.source_name);
                    if *apply {
                        let outputs = backend.list_outputs()?;
                        for out in &outputs {
                            engine.set_wallpaper(&out.name, rec.hash, Fit::Fill, Some((0.5, 0.5)))?;
                            let rendered = engine.render_wallpaper_for_screen(&out.name, (out.width, out.height), OutFormat::Png)?;
                            backend.set(&out.name, &rendered, Fit::Fill)?;
                            println!("Applied \x1b[32m{}\x1b[0m to {}", rec.hash, out.name);
                        }
                    }
                }
            }
        }
        Commands::Sync { url } => {
            let report = engine.sync_with_peer(url).await?;
            println!("Sync completed: {:?}", report);
        }
    }
    Ok(())
}

fn render_pairing_qr(payload: &str, token: &str, pin: &str) {
    println!("\n\x1b[1;36m=== Scan with sync_wall Android Companion App ===\x1b[0m");
    if let Ok(code) = QrCode::new(payload.as_bytes()) {
        let image = code.render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        println!("{}", image);
    }
    println!("One-Time QR Token (128-bit): \x1b[1;32m{}\x1b[0m", token);
    println!("Manual Entry PIN (Fallback):  \x1b[1;33m{}\x1b[0m", pin);
    println!("Expires in 120 seconds (maximum 5 attempts before lockout)\n");
}
