use std::{fs, net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting dir:{:?} at {}...", dir, addr);
    let state = HttpServeState { path: dir.clone() };
    let router = axum::Router::new()
        .nest_service("/tower", ServeDir::new(dir))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    if p.exists() {
        if p.is_dir() {
            let mut content = String::new();
            for entry in fs::read_dir(p).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let name = entry.file_name();
                content.push_str(&format!(
                    "path:{},name:{}/\n",
                    path.display(),
                    name.to_string_lossy()
                ));
            }
            (StatusCode::OK, content)
        } else {
            match tokio::fs::read(p).await {
                Ok(content) => {
                    let content = String::from_utf8_lossy(&content);
                    info!("read file length: {}", content.len());
                    (StatusCode::OK, content.to_string())
                }
                Err(e) => {
                    warn!("Error reading file e: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))
                }
            }
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let path = Path("Cargo.toml".to_string());
        let (status, content) = file_handler(State(state), path).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.contains("[package]"));
    }
}
