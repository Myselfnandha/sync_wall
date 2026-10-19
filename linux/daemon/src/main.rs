use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tracing::{error, info, warn};

use sync_wall_backend::{detect_backend, WallpaperBackend};
use sync_wall_core::model::*;
use sync_wall_core::SyncWallEngine;

#[derive(Parser, Debug)]
#[command(author, version, about = "sync_wall Linux Headless Daemon")]
struct Args {
    #[arg(short, long)]
    port: Option<u16>,

    #[arg(long)]
    data_dir: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    pub id: u64,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    pub id: u64,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

struct DaemonContext {
    engine: SyncWallEngine,
    backend: Box<dyn WallpaperBackend>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let base_data_dir = args.data_dir.unwrap_or_else(|| {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("sync_wall")
    });
    fs::create_dir_all(&base_data_dir)?;

    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "linux-pc".to_string());

    info!("Initializing sync_wall engine at {:?}", base_data_dir);
    let engine = SyncWallEngine::open(&base_data_dir, &hostname)?;
    let backend = detect_backend();
    info!("Detected active wallpaper backend: {}", backend.name());

    let ctx = Arc::new(DaemonContext { engine, backend });

    // Setup Unix Domain Socket for CLI/UI JSON-RPC
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
        .join("sync_wall");
    fs::create_dir_all(&runtime_dir)?;
    let socket_path = runtime_dir.join("sync_walld.sock");
    if socket_path.exists() {
        let _ = fs::remove_file(&socket_path);
    }

    let unix_listener = UnixListener::bind(&socket_path)?;
    info!("Listening for local JSON-RPC at {:?}", socket_path);

    // Setup HTTP LAN sync server
    let server_port = args.port.unwrap_or(7889);
    let router = sync_wall_core::net::create_router(ctx.engine.server_state());
    let tcp_addr = format!("0.0.0.0:{}", server_port);
    let tcp_listener = tokio::net::TcpListener::bind(&tcp_addr).await?;
    info!("Hosting peer sync server at {}", tcp_addr);

    tokio::spawn(async move {
        if let Err(e) = axum::serve(tcp_listener, router).await {
            error!("Sync server error: {}", e);
        }
    });

    // Accept Unix socket connections
    loop {
        match unix_listener.accept().await {
            Ok((stream, _)) => {
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    handle_unix_client(stream, ctx_clone).await;
                });
            }
            Err(e) => {
                warn!("Accept failed: {}", e);
            }
        }
    }
}

async fn handle_unix_client(stream: UnixStream, ctx: Arc<DaemonContext>) {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        if line.trim().is_empty() {
            continue;
        }

        let req: RpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = RpcResponse {
                    id: 0,
                    result: None,
                    error: Some(format!("Invalid JSON-RPC request: {}", e)),
                };
                let resp_str = serde_json::to_string(&resp).unwrap_or_default() + "\n";
                let _ = writer.write_all(resp_str.as_bytes()).await;
                continue;
            }
        };

        let result = dispatch_rpc(&req.method, req.params, &ctx).await;
        let resp = match result {
            Ok(v) => RpcResponse {
                id: req.id,
                result: Some(v),
                error: None,
            },
            Err(err_msg) => RpcResponse {
                id: req.id,
                result: None,
                error: Some(err_msg),
            },
        };

        let resp_str = serde_json::to_string(&resp).unwrap_or_default() + "\n";
        if writer.write_all(resp_str.as_bytes()).await.is_err() {
            break;
        }
    }
}

async fn dispatch_rpc(
    method: &str,
    params: serde_json::Value,
    ctx: &DaemonContext,
) -> Result<serde_json::Value, String> {
    match method {
        "status" => {
            let outputs = ctx.backend.list_outputs().map_err(|e| e.to_string())?;
            let images = ctx.engine.list_images().map_err(|e| e.to_string())?;
            let devices = ctx.engine.store().list_devices().map_err(|e| e.to_string())?;
            Ok(serde_json::json!({
                "device_id": ctx.engine.device_id(),
                "backend": ctx.backend.name(),
                "outputs": outputs.iter().map(|o| &o.name).collect::<Vec<_>>(),
                "image_count": images.len(),
                "device_count": devices.len(),
            }))
        }
        "list_outputs" => {
            let outputs = ctx.backend.list_outputs().map_err(|e| e.to_string())?;
            Ok(serde_json::json!(outputs))
        }
        "add" => {
            let paths: Vec<String> = serde_json::from_value(params.get("paths").cloned().unwrap_or_default())
                .map_err(|e| e.to_string())?;
            let tags: Vec<String> = serde_json::from_value(params.get("tags").cloned().unwrap_or_default())
                .unwrap_or_default();

            let mut added = Vec::new();
            for p in paths {
                let rec = ctx.engine.ingest_file(PathBuf::from(p), tags.clone())
                    .map_err(|e| e.to_string())?;
                added.push(rec);
            }
            Ok(serde_json::json!(added))
        }
        "list_images" => {
            let images = ctx.engine.list_images().map_err(|e| e.to_string())?;
            Ok(serde_json::json!(images))
        }
        "set" => {
            let hash_str = params.get("hash").and_then(|v| v.as_str())
                .ok_or_else(|| "Missing hash param".to_string())?;
            let hash: Hash = hash_str.parse().map_err(|e| format!("Invalid hash: {}", e))?;

            let screen_name = params.get("screen").and_then(|v| v.as_str()).unwrap_or("");
            let outputs = ctx.backend.list_outputs().map_err(|e| e.to_string())?;

            let target_outputs = if screen_name.is_empty() {
                outputs.clone()
            } else {
                outputs.into_iter().filter(|o| o.name == screen_name).collect()
            };

            if target_outputs.is_empty() {
                return Err(format!("Output '{}' not found", screen_name));
            }

            for out in &target_outputs {
                // Record assignment
                ctx.engine.set_wallpaper(&out.name, hash, Fit::Fill, Some((0.5, 0.5)))
                    .map_err(|e| e.to_string())?;

                // Render cropped desktop derivative (PNG)
                let rendered_path = ctx.engine.render_wallpaper_for_screen(
                    &out.name,
                    (out.width, out.height),
                    OutFormat::Png,
                ).map_err(|e| e.to_string())?;

                // Apply to desktop
                ctx.backend.set(&out.name, &rendered_path, Fit::Fill)
                    .map_err(|e| e.to_string())?;
            }

            Ok(serde_json::json!({ "applied_screens": target_outputs.iter().map(|o| &o.name).collect::<Vec<_>>() }))
        }
        "pair_token" => {
            let session = ctx.engine.generate_pairing_session();
            Ok(serde_json::json!({
                "token": session.token,
                "pin": session.pin,
                "device_id": ctx.engine.device_id(),
                "expires_in_secs": 120
            }))
        }
        "sync_peer" => {
            let url = params.get("url").and_then(|v| v.as_str())
                .ok_or_else(|| "Missing url param".to_string())?;
            let report = ctx.engine.sync_with_peer(url).await
                .map_err(|e| e.to_string())?;
            Ok(serde_json::json!(report))
        }
        "fetch" => {
            let provider = params.get("provider").and_then(|v| v.as_str()).unwrap_or("bing");
            let apply = params.get("apply").and_then(|v| v.as_bool()).unwrap_or(false);

            let records = match provider {
                "unsplash" => {
                    let config = sync_wall_core::source::UnsplashConfig {
                        access_key: params.get("key").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        query: params.get("query").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        collection: params.get("collection").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        orientation: params.get("orientation").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        count: params.get("count").and_then(|v| v.as_u64()).unwrap_or(1) as usize,
                    };
                    ctx.engine.fetch_unsplash(&config).await.map_err(|e| e.to_string())?
                }
                "wallhaven" => {
                    let query = params.get("query").and_then(|v| v.as_str());
                    let count = params.get("count").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
                    ctx.engine.fetch_wallhaven(query, count).await.map_err(|e| e.to_string())?
                }
                "bing" => {
                    let rec = ctx.engine.fetch_bing_daily().await.map_err(|e| e.to_string())?;
                    vec![rec]
                }
                other => return Err(format!("Unsupported source provider: {}", other)),
            };

            if apply {
                if let Some(first) = records.first() {
                    let outputs = ctx.backend.list_outputs().map_err(|e| e.to_string())?;
                    for out in &outputs {
                        ctx.engine.set_wallpaper(&out.name, first.hash, Fit::Fill, Some((0.5, 0.5)))
                            .map_err(|e| e.to_string())?;
                        let rendered = ctx.engine.render_wallpaper_for_screen(
                            &out.name,
                            (out.width, out.height),
                            OutFormat::Png,
                        ).map_err(|e| e.to_string())?;
                        ctx.backend.set(&out.name, &rendered, Fit::Fill)
                            .map_err(|e| e.to_string())?;
                    }
                }
            }

            Ok(serde_json::json!(records))
        }
        _ => Err(format!("Unknown RPC method: {}", method)),
    }
}
